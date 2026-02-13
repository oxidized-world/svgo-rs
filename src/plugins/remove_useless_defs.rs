use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct RemoveUselessDefsPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct RemoveUselessDefsPluginConfig {}

impl<'a> RemoveUselessDefsPlugin<'a> {
  pub fn new(_config: RemoveUselessDefsPluginConfig, arena: &'a Bump) -> Self {
    RemoveUselessDefsPlugin { arena }
  }

  fn is_non_rendering(name: &str) -> bool {
    matches!(
      name,
      "defs"
        | "clipPath"
        | "mask"
        | "pattern"
        | "marker"
        | "symbol"
        | "linearGradient"
        | "radialGradient"
        | "filter"
    )
  }

  fn collect_useful_nodes(el: &XMLAstElement<'a>, out: &mut Vec<XMLAstChild<'a>>) {
    for child in &el.children {
      if let XMLAstChild::Element(child_el) = child {
        let has_id = child_el.attributes.iter().any(|(k, _)| *k == "id");
        if has_id || child_el.name == "style" {
          out.push(XMLAstChild::Element(child_el.clone()));
        } else {
          Self::collect_useful_nodes(child_el, out);
        }
      }
    }
  }

  fn process_children(&self, children: &mut BumpVec<'a, XMLAstChild<'a>>, arena: &'a Bump) {
    let mut i = 0usize;
    while i < children.len() {
      let mut remove_current = false;
      if let Some(XMLAstChild::Element(el)) = children.get_mut(i) {
        self.process_children(&mut el.children, arena);

        if el.name == "defs"
          || (Self::is_non_rendering(el.name) && !el.attributes.iter().any(|(k, _)| *k == "id"))
        {
          let mut useful = Vec::new();
          Self::collect_useful_nodes(el, &mut useful);
          if useful.is_empty() {
            remove_current = true;
          } else {
            let mut replacement = BumpVec::new_in(arena);
            for child in useful {
              replacement.push(child);
            }
            el.children = replacement;
          }
        }
      }

      if remove_current {
        children.remove(i);
      } else {
        i += 1;
      }
    }
  }
}

impl<'a> Plugin<'a> for RemoveUselessDefsPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    self.process_children(&mut root.children, self.arena);
  }
}
