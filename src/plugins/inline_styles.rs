use bumpalo::Bump;
use phf::phf_set;
use regex::Regex;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct InlineStylesPlugin<'a> {
  pub arena: &'a Bump,
  reg_rule: Regex,
}

#[derive(Clone)]
struct InlineRule {
  selector: String,
  declarations: Vec<(String, String)>,
}

#[derive(Clone)]
struct ElementMarker {
  name: String,
  id: Option<String>,
  classes: Vec<String>,
}

#[derive(Clone)]
struct NodeContext {
  marker: ElementMarker,
  previous_siblings: Vec<ElementMarker>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SelectorCombinator {
  Descendant,
  Child,
  AdjacentSibling,
  GeneralSibling,
}

pub struct InlineStylesPluginConfig {}

impl<'a> InlineStylesPlugin<'a> {
  pub fn new(_config: InlineStylesPluginConfig, arena: &'a Bump) -> Self {
    InlineStylesPlugin {
      arena,
      reg_rule: Regex::new(r"(?s)([^{}]+)\{([^}]*)\}").unwrap(),
    }
  }

  fn decode_css_entities(input: &str) -> String {
    input
      .replace("&gt;", ">")
      .replace("&lt;", "<")
      .replace("&quot;", "\"")
      .replace("&apos;", "'")
      .replace("&amp;", "&")
  }

  fn parse_declarations(body: &str) -> Vec<(String, String)> {
    let mut decls: Vec<(String, String)> = Vec::new();
    for item in body.split(';') {
      let Some((k, v)) = item.split_once(':') else {
        continue;
      };
      let key = k.trim().to_lowercase();
      let val = v.trim().to_string();
      if !key.is_empty() && !val.is_empty() && PRESENTATION_ATTRS.contains(key.as_str()) {
        decls.push((key, val));
      }
    }
    decls
  }

  fn collect_rules(&self, children: &[XMLAstChild<'a>], rules: &mut Vec<InlineRule>) {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        if el.name == "foreignObject" {
          continue;
        }
        if el.name == "style" {
          let style_type =
            el.attributes.iter().find(|(k, _)| *k == "type").map(|(_, v)| *v).unwrap_or("");
          if style_type.is_empty() || style_type == "text/css" {
            let mut css = String::new();
            for c in &el.children {
              match c {
                XMLAstChild::Text(t) => css.push_str(t.value),
                XMLAstChild::Cdata(cd) => css.push_str(cd.value),
                _ => {}
              }
            }
            let css = Self::decode_css_entities(&css);
            for cap in self.reg_rule.captures_iter(&css) {
              let selector = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
              let body = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("");
              if selector.is_empty() || body.is_empty() {
                continue;
              }
              let decls = Self::parse_declarations(body);
              if !decls.is_empty() {
                for sel in selector.split(',') {
                  let s = sel.trim();
                  if !s.is_empty() {
                    rules.push(InlineRule {
                      selector: s.to_string(),
                      declarations: decls.clone(),
                    });
                  }
                }
              }
            }
          }
        }
        self.collect_rules(&el.children, rules);
      }
    }
  }

  fn marker_from_element(el: &XMLAstElement<'a>) -> ElementMarker {
    let id = el.attributes.iter().find(|(k, _)| *k == "id").map(|(_, v)| (*v).to_string());
    let classes = el
      .attributes
      .iter()
      .find(|(k, _)| *k == "class")
      .map(|(_, v)| v.split_whitespace().map(|c| c.to_string()).collect())
      .unwrap_or_default();
    ElementMarker {
      name: el.name.to_ascii_lowercase(),
      id,
      classes,
    }
  }

  fn matches_simple_selector(marker: &ElementMarker, selector: &str) -> bool {
    let selector = selector.trim();
    if selector.is_empty() {
      return false;
    }

    if selector == "*" {
      return true;
    }

    if selector.contains('[') || selector.contains(':') {
      return false;
    }

    let mut rest = selector;

    if !rest.starts_with('.') && !rest.starts_with('#') {
      let mut end = rest.len();
      for (i, ch) in rest.char_indices() {
        if ch == '.' || ch == '#' {
          end = i;
          break;
        }
      }
      let tag = &rest[..end];
      if !tag.is_empty() && !tag.eq_ignore_ascii_case(&marker.name) {
        return false;
      }
      rest = &rest[end..];
    }

    while !rest.is_empty() {
      if let Some(class_selector) = rest.strip_prefix('.') {
        let mut end = class_selector.len();
        for (i, ch) in class_selector.char_indices() {
          if ch == '.' || ch == '#' {
            end = i;
            break;
          }
        }
        let class_name = &class_selector[..end];
        if class_name.is_empty() || !marker.classes.iter().any(|c| c == class_name) {
          return false;
        }
        rest = &class_selector[end..];
        continue;
      }

      if let Some(id_selector) = rest.strip_prefix('#') {
        let mut end = id_selector.len();
        for (i, ch) in id_selector.char_indices() {
          if ch == '.' || ch == '#' {
            end = i;
            break;
          }
        }
        let id_name = &id_selector[..end];
        if id_name.is_empty() || marker.id.as_deref() != Some(id_name) {
          return false;
        }
        rest = &id_selector[end..];
        continue;
      }

      return false;
    }

    true
  }

  fn parse_selector_chain(selector: &str) -> Option<(Vec<String>, Vec<SelectorCombinator>)> {
    let normalized = selector.replace('>', " > ").replace('+', " + ").replace('~', " ~ ");
    let parts: Vec<&str> = normalized.split_whitespace().filter(|p| !p.is_empty()).collect();
    if parts.is_empty() {
      return None;
    }

    let mut simples: Vec<String> = Vec::new();
    let mut combinators: Vec<SelectorCombinator> = Vec::new();
    let mut pending_combinator: Option<SelectorCombinator> = None;

    for part in parts {
      if part == ">" || part == "+" || part == "~" {
        if pending_combinator.is_some() || simples.is_empty() {
          return None;
        }

        pending_combinator = Some(match part {
          ">" => SelectorCombinator::Child,
          "+" => SelectorCombinator::AdjacentSibling,
          "~" => SelectorCombinator::GeneralSibling,
          _ => return None,
        });
        continue;
      }

      if simples.is_empty() {
        simples.push(part.to_string());
      } else {
        combinators.push(pending_combinator.unwrap_or(SelectorCombinator::Descendant));
        pending_combinator = None;
        simples.push(part.to_string());
      }
    }

    if pending_combinator.is_some() || simples.is_empty() || combinators.len() + 1 != simples.len()
    {
      return None;
    }

    Some((simples, combinators))
  }

  fn set_attr(el: &mut XMLAstElement<'a>, name: &'a str, value: &'a str) {
    for (k, v) in &mut el.attributes {
      if *k == name {
        *v = value;
        return;
      }
    }
    el.attributes.push((name, value));
  }

  fn matches_selector(
    marker: &ElementMarker,
    ancestors: &[NodeContext],
    previous_siblings: &[ElementMarker],
    selector: &str,
  ) -> bool {
    let Some((simples, combinators)) = Self::parse_selector_chain(selector) else {
      return false;
    };

    if simples.is_empty() {
      return false;
    }

    let rightmost = simples.last().map(|s| s.as_str()).unwrap_or("");

    if !Self::matches_simple_selector(marker, rightmost) {
      return false;
    }

    if simples.len() == 1 {
      return true;
    }

    let mut cursor_ancestors = ancestors.to_vec();
    let mut cursor_previous_siblings = previous_siblings.to_vec();

    for i in (0..simples.len() - 1).rev() {
      let target = simples[i].as_str();
      match combinators[i] {
        SelectorCombinator::Child => {
          let Some(parent) = cursor_ancestors.last() else {
            return false;
          };
          if !Self::matches_simple_selector(&parent.marker, target) {
            return false;
          }
          cursor_previous_siblings = parent.previous_siblings.clone();
          cursor_ancestors.pop();
        }
        SelectorCombinator::Descendant => {
          let mut found = false;
          while let Some(ancestor) = cursor_ancestors.pop() {
            if Self::matches_simple_selector(&ancestor.marker, target) {
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
          let prev_idx = cursor_previous_siblings.len() - 1;
          if !Self::matches_simple_selector(&cursor_previous_siblings[prev_idx], target) {
            return false;
          }
          cursor_previous_siblings.truncate(prev_idx);
        }
        SelectorCombinator::GeneralSibling => {
          let mut found_idx: Option<usize> = None;
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

  fn apply_rules_to_children(
    &self,
    children: &mut [XMLAstChild<'a>],
    rules: &[InlineRule],
    ancestors: &mut Vec<NodeContext>,
    in_foreign_object: bool,
  ) {
    let mut previous_siblings: Vec<ElementMarker> = Vec::new();

    for child in children {
      if let XMLAstChild::Element(el) = child {
        let next_foreign_object = in_foreign_object || el.name == "foreignObject";
        let marker = Self::marker_from_element(el);
        if !next_foreign_object {
          for rule in rules {
            if Self::matches_selector(&marker, ancestors, &previous_siblings, &rule.selector) {
              for (k, v) in &rule.declarations {
                Self::set_attr(el, self.arena.alloc_str(k), self.arena.alloc_str(v));
              }
            }
          }
        }
        ancestors.push(NodeContext {
          marker: marker.clone(),
          previous_siblings: previous_siblings.clone(),
        });
        self.apply_rules_to_children(&mut el.children, rules, ancestors, next_foreign_object);
        ancestors.pop();

        previous_siblings.push(marker);
      }
    }
  }
}

static PRESENTATION_ATTRS: phf::Set<&'static str> = phf_set! {
  "fill", "fill-opacity", "fill-rule", "stroke", "stroke-width", "stroke-opacity",
  "stroke-linecap", "stroke-linejoin", "stroke-dasharray", "stroke-dashoffset", "opacity",
  "color", "display", "visibility", "stop-color", "stop-opacity", "font-family", "font-size"
};

impl<'a> Plugin<'a> for InlineStylesPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    let mut rules: Vec<InlineRule> = Vec::new();
    self.collect_rules(&root.children, &mut rules);
    if rules.is_empty() {
      return;
    }
    let mut ancestors: Vec<NodeContext> = Vec::new();
    self.apply_rules_to_children(&mut root.children, &rules, &mut ancestors, false);
  }
}
