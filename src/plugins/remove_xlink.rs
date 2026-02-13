use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot, XMLAstText};

pub struct RemoveXlinkPlugin<'a> {
  pub arena: &'a Bump,
  pub include_legacy: bool,
}

pub struct RemoveXlinkPluginConfig {
  pub include_legacy: bool,
}

impl<'a> RemoveXlinkPlugin<'a> {
  pub fn new(config: RemoveXlinkPluginConfig, arena: &'a Bump) -> Self {
    RemoveXlinkPlugin {
      arena,
      include_legacy: config.include_legacy,
    }
  }

  fn is_legacy_elem(name: &str) -> bool {
    matches!(
      name,
      "cursor" | "filter" | "font-face-uri" | "glyphRef" | "tref"
    )
  }

  fn has_xlink_attr(children: &[XMLAstChild<'a>]) -> bool {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        if el.attributes.iter().any(|(name, _)| name.starts_with("xlink:")) {
          return true;
        }
        if Self::has_xlink_attr(&el.children) {
          return true;
        }
      }
    }
    false
  }

  fn remove_xmlns_xlink(children: &mut [XMLAstChild<'a>]) {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        el.attributes.retain(|(name, _)| *name != "xmlns:xlink");
        Self::remove_xmlns_xlink(&mut el.children);
      }
    }
  }
}

impl<'a> Plugin<'a> for RemoveXlinkPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if let Some((_, show_value)) = el.attributes.iter().find(|(k, _)| *k == "xlink:show") {
      if !el.attributes.iter().any(|(k, _)| *k == "target") {
        let mapped = match *show_value {
          "new" => Some("_blank"),
          "replace" => Some("_self"),
          _ => None,
        };
        if let Some(mapped) = mapped {
          el.attributes
            .push((self.arena.alloc_str("target"), self.arena.alloc_str(mapped)));
        }
      }
      el.attributes.retain(|(k, _)| *k != "xlink:show");
    }

    if let Some((_, title_value)) = el.attributes.iter().find(|(k, _)| *k == "xlink:title") {
      let has_title = el
        .children
        .iter()
        .any(|c| matches!(c, XMLAstChild::Element(child_el) if child_el.name == "title"));
      if !has_title {
        let mut title_children = BumpVec::new_in(self.arena);
        title_children.push(XMLAstChild::Text(XMLAstText {
          value: self.arena.alloc_str(title_value),
        }));
        let title = XMLAstElement {
          name: self.arena.alloc_str("title"),
          attributes: BumpVec::new_in(self.arena),
          children: title_children,
        };
        el.children.insert(0, XMLAstChild::Element(title));
      }
      el.attributes.retain(|(k, _)| *k != "xlink:title");
    }

    let mut has_xlink_href: Option<&'a str> = None;
    for (name, value) in &el.attributes {
      if *name == "xlink:href" {
        has_xlink_href = Some(*value);
      }
    }

    if let Some(href_value) = has_xlink_href {
      if self.include_legacy || !Self::is_legacy_elem(el.name) {
        if !el.attributes.iter().any(|(k, _)| *k == "href") {
          el.attributes.push(("href", href_value));
        }
        el.attributes.retain(|(k, _)| *k != "xlink:href");
      }
    }

    el.attributes.retain(|(name, _)| {
      if *name == "xmlns:xlink" {
        return true;
      }
      if name.starts_with("xlink:") && *name != "xlink:href" {
        return false;
      }
      true
    });

    VisitAction::Keep
  }

  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    if !Self::has_xlink_attr(&root.children) {
      Self::remove_xmlns_xlink(&mut root.children);
    }
  }
}
