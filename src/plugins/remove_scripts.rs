use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use phf::phf_set;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct RemoveScriptsPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct RemoveScriptsPluginConfig {}

static EVENT_ATTRS: phf::Set<&'static str> = phf_set! {
  "onbegin", "onend", "onrepeat", "onload", "onerror", "onfocusin", "onfocusout", "onactivate",
  "onclick", "onmousedown", "onmouseup", "onmouseover", "onmousemove", "onmouseout", "onunload"
};

impl<'a> RemoveScriptsPlugin<'a> {
  pub fn new(_config: RemoveScriptsPluginConfig, arena: &'a Bump) -> Self {
    RemoveScriptsPlugin { arena }
  }

  fn clean_children(&self, children: &mut BumpVec<'a, XMLAstChild<'a>>) {
    let mut i = 0usize;
    while i < children.len() {
      let mut replacement: Option<Vec<XMLAstChild<'a>>> = None;

      if let Some(XMLAstChild::Element(el)) = children.get_mut(i) {
        el.attributes.retain(|(name, _)| !EVENT_ATTRS.contains(name));
        self.clean_children(&mut el.children);

        if el.name == "a" {
          let has_js_href = el.attributes.iter().any(|(name, value)| {
            (*name == "href" || name.ends_with(":href"))
              && value.trim_start().to_ascii_lowercase().starts_with("javascript:")
          });

          if has_js_href {
            replacement = Some(
              el.children
                .iter()
                .filter(|child| !matches!(child, XMLAstChild::Text(_)))
                .cloned()
                .collect(),
            );
          }
        }
      }

      if let Some(nodes) = replacement {
        let count = nodes.len();
        children.remove(i);
        for (offset, node) in nodes.into_iter().enumerate() {
          children.insert(i + offset, node);
        }
        if count == 0 {
          continue;
        }
        i += count;
      } else {
        i += 1;
      }
    }
  }
}

impl<'a> Plugin<'a> for RemoveScriptsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "script" {
      return VisitAction::Remove;
    }
    el.attributes.retain(|(name, _)| !EVENT_ATTRS.contains(name));
    VisitAction::Keep
  }

  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    let _ = self.arena;
    self.clean_children(&mut root.children);
  }
}
