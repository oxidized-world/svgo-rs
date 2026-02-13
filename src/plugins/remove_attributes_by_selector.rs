use std::collections::HashMap;

use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

#[derive(Clone)]
struct SelectorElement {
  name: String,
  id: Option<String>,
  classes: Vec<String>,
  attrs: HashMap<String, String>,
}

#[derive(Clone)]
struct NodeContext {
  element: SelectorElement,
  previous_siblings: Vec<SelectorElement>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SelectorCombinator {
  Descendant,
  Child,
  AdjacentSibling,
  GeneralSibling,
}

#[derive(Clone)]
struct AttrSelector {
  name: String,
  value: Option<String>,
}

#[derive(Clone)]
struct SimpleSelector {
  tag: Option<String>,
  id: Option<String>,
  classes: Vec<String>,
  attr: Option<AttrSelector>,
}

pub struct RemoveAttributesBySelectorPlugin<'a> {
  pub selector: &'a str,
  pub attributes: Vec<&'a str>,
}

pub struct RemoveAttributesBySelectorPluginConfig<'a> {
  pub selector: Option<&'a str>,
  pub attributes: Vec<&'a str>,
}

impl<'a> RemoveAttributesBySelectorPlugin<'a> {
  pub fn new(config: RemoveAttributesBySelectorPluginConfig<'a>, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveAttributesBySelectorPlugin {
      selector: config.selector.unwrap_or(""),
      attributes: config.attributes,
    }
  }

  fn marker_from_element(el: &XMLAstElement<'a>) -> SelectorElement {
    let mut attrs = HashMap::new();
    for (k, v) in &el.attributes {
      attrs.insert((*k).to_string(), (*v).to_string());
    }

    let id = attrs.get("id").cloned();
    let classes = attrs
      .get("class")
      .map(|v| v.split_whitespace().map(|c| c.to_string()).collect::<Vec<String>>())
      .unwrap_or_default();

    SelectorElement {
      name: el.name.to_ascii_lowercase(),
      id,
      classes,
      attrs,
    }
  }

  fn parse_simple_selector(raw: &str) -> Option<SimpleSelector> {
    let s = raw.trim();
    if s.is_empty() {
      return None;
    }

    let mut rest = s;
    let mut tag: Option<String> = None;
    let mut id: Option<String> = None;
    let mut classes: Vec<String> = Vec::new();
    let mut attr: Option<AttrSelector> = None;

    if !rest.starts_with('.') && !rest.starts_with('#') && !rest.starts_with('[') {
      let mut end = rest.len();
      for (i, ch) in rest.char_indices() {
        if ch == '.' || ch == '#' || ch == '[' {
          end = i;
          break;
        }
      }
      let t = &rest[..end];
      if !t.is_empty() && t != "*" {
        tag = Some(t.to_ascii_lowercase());
      }
      rest = &rest[end..];
    }

    while !rest.is_empty() {
      if let Some(part) = rest.strip_prefix('.') {
        let mut end = part.len();
        for (i, ch) in part.char_indices() {
          if ch == '.' || ch == '#' || ch == '[' {
            end = i;
            break;
          }
        }
        if end == 0 {
          return None;
        }
        classes.push(part[..end].to_string());
        rest = &part[end..];
        continue;
      }
      if let Some(part) = rest.strip_prefix('#') {
        let mut end = part.len();
        for (i, ch) in part.char_indices() {
          if ch == '.' || ch == '#' || ch == '[' {
            end = i;
            break;
          }
        }
        if end == 0 {
          return None;
        }
        id = Some(part[..end].to_string());
        rest = &part[end..];
        continue;
      }
      if let Some(part) = rest.strip_prefix('[') {
        let close = part.find(']')?;
        let inner = part[..close].trim();
        let (name, value) = if let Some((k, v)) = inner.split_once('=') {
          let value = v.trim().trim_matches('"').trim_matches('\'').to_string();
          (k.trim().to_string(), Some(value))
        } else {
          (inner.to_string(), None)
        };
        if name.is_empty() {
          return None;
        }
        attr = Some(AttrSelector { name, value });
        rest = &part[close + 1..];
        continue;
      }
      return None;
    }

    Some(SimpleSelector {
      tag,
      id,
      classes,
      attr,
    })
  }

  fn parse_selector_chain(
    selector: &str,
  ) -> Option<(Vec<SimpleSelector>, Vec<SelectorCombinator>)> {
    let normalized = selector.replace('>', " > ").replace('+', " + ").replace('~', " ~ ");
    let parts: Vec<&str> = normalized.split_whitespace().filter(|s| !s.is_empty()).collect();
    if parts.is_empty() {
      return None;
    }

    let mut simples = Vec::new();
    let mut combinators = Vec::new();
    let mut pending: Option<SelectorCombinator> = None;

    for part in parts {
      if part == ">" || part == "+" || part == "~" {
        if pending.is_some() || simples.is_empty() {
          return None;
        }
        pending = Some(match part {
          ">" => SelectorCombinator::Child,
          "+" => SelectorCombinator::AdjacentSibling,
          "~" => SelectorCombinator::GeneralSibling,
          _ => return None,
        });
        continue;
      }

      let simple = Self::parse_simple_selector(part)?;
      if simples.is_empty() {
        simples.push(simple);
      } else {
        combinators.push(pending.unwrap_or(SelectorCombinator::Descendant));
        pending = None;
        simples.push(simple);
      }
    }

    if pending.is_some() || combinators.len() + 1 != simples.len() {
      return None;
    }

    Some((simples, combinators))
  }

  fn matches_simple_selector(el: &SelectorElement, selector: &SimpleSelector) -> bool {
    if let Some(tag) = &selector.tag {
      if &el.name != tag {
        return false;
      }
    }
    if let Some(id) = &selector.id {
      if el.id.as_deref() != Some(id.as_str()) {
        return false;
      }
    }
    for class_name in &selector.classes {
      if !el.classes.iter().any(|c| c == class_name) {
        return false;
      }
    }
    if let Some(attr) = &selector.attr {
      let Some(actual) = el.attrs.get(&attr.name) else {
        return false;
      };
      if let Some(expected) = &attr.value {
        if actual != expected {
          return false;
        }
      }
    }

    true
  }

  fn matches_selector(
    current: &SelectorElement,
    ancestors: &[NodeContext],
    previous_siblings: &[SelectorElement],
    selector: &str,
  ) -> bool {
    let Some((simples, combinators)) = Self::parse_selector_chain(selector) else {
      return false;
    };
    if simples.is_empty() {
      return false;
    }

    if !Self::matches_simple_selector(current, simples.last().unwrap()) {
      return false;
    }

    if simples.len() == 1 {
      return true;
    }

    let mut cursor_ancestors = ancestors.to_vec();
    let mut cursor_previous_siblings = previous_siblings.to_vec();

    for i in (0..simples.len() - 1).rev() {
      let target = &simples[i];
      match combinators[i] {
        SelectorCombinator::Child => {
          let Some(parent) = cursor_ancestors.last() else {
            return false;
          };
          if !Self::matches_simple_selector(&parent.element, target) {
            return false;
          }
          cursor_previous_siblings = parent.previous_siblings.clone();
          cursor_ancestors.pop();
        }
        SelectorCombinator::Descendant => {
          let mut found = false;
          while let Some(ancestor) = cursor_ancestors.pop() {
            if Self::matches_simple_selector(&ancestor.element, target) {
              cursor_previous_siblings = ancestor.previous_siblings;
              found = true;
              break;
            }
          }
          if !found {
            return false;
          }
        }
        SelectorCombinator::AdjacentSibling => {
          if cursor_previous_siblings.is_empty() {
            return false;
          }
          let idx = cursor_previous_siblings.len() - 1;
          if !Self::matches_simple_selector(&cursor_previous_siblings[idx], target) {
            return false;
          }
          cursor_previous_siblings.truncate(idx);
        }
        SelectorCombinator::GeneralSibling => {
          let mut found_idx = None;
          for idx in (0..cursor_previous_siblings.len()).rev() {
            if Self::matches_simple_selector(&cursor_previous_siblings[idx], target) {
              found_idx = Some(idx);
              break;
            }
          }
          let Some(idx) = found_idx else {
            return false;
          };
          cursor_previous_siblings.truncate(idx);
        }
      }
    }

    true
  }

  fn process_children(&self, children: &mut [XMLAstChild<'a>], ancestors: &mut Vec<NodeContext>) {
    let mut previous_siblings: Vec<SelectorElement> = Vec::new();

    for child in children {
      if let XMLAstChild::Element(el) = child {
        let marker = Self::marker_from_element(el);
        if Self::matches_selector(&marker, ancestors, &previous_siblings, self.selector) {
          el.attributes.retain(|(name, _)| !self.attributes.iter().any(|a| a == name));
        }

        ancestors.push(NodeContext {
          element: marker.clone(),
          previous_siblings: previous_siblings.clone(),
        });
        self.process_children(&mut el.children, ancestors);
        ancestors.pop();

        previous_siblings.push(marker);
      }
    }
  }
}

impl<'a> Plugin<'a> for RemoveAttributesBySelectorPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    if self.selector.is_empty() || self.attributes.is_empty() {
      return;
    }

    let mut ancestors: Vec<NodeContext> = Vec::new();
    self.process_children(&mut root.children, &mut ancestors);
  }
}
