use std::collections::HashMap;

use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstRoot};

pub struct SortDefsChildrenPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct SortDefsChildrenPluginConfig {}

impl<'a> SortDefsChildrenPlugin<'a> {
  pub fn new(_config: SortDefsChildrenPluginConfig, arena: &'a Bump) -> Self {
    SortDefsChildrenPlugin { arena }
  }

  fn sort_defs(&self, children: &mut BumpVec<'a, XMLAstChild<'a>>) {
    for child in children.iter_mut() {
      if let XMLAstChild::Element(el) = child {
        self.sort_defs(&mut el.children);
        if el.name == "defs" {
          let mut freq = HashMap::new();
          for c in &el.children {
            if let XMLAstChild::Element(e) = c {
              *freq.entry(e.name.to_string()).or_insert(0usize) += 1;
            }
          }
          let mut sorted = el.children.iter().cloned().collect::<Vec<XMLAstChild<'a>>>();
          sorted.sort_by(|a, b| match (a, b) {
            (XMLAstChild::Element(ae), XMLAstChild::Element(be)) => {
              let af = *freq.get(ae.name).unwrap_or(&0);
              let bf = *freq.get(be.name).unwrap_or(&0);
              if af != bf {
                return bf.cmp(&af);
              }
              if ae.name.len() != be.name.len() {
                return be.name.len().cmp(&ae.name.len());
              }
              be.name.cmp(ae.name)
            }
            _ => std::cmp::Ordering::Equal,
          });
          el.children.clear();
          for item in sorted {
            el.children.push(item);
          }
        }
      }
    }
  }
}

impl<'a> Plugin<'a> for SortDefsChildrenPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    let _ = self.arena;
    self.sort_defs(&mut root.children);
  }
}
