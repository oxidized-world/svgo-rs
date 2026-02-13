use std::collections::HashSet;

use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstRoot};

pub struct RemoveEmptyContainersPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
}

pub struct RemoveEmptyContainersPluginConfig {}

impl<'a> RemoveEmptyContainersPlugin<'a> {
  pub fn new(_config: RemoveEmptyContainersPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveEmptyContainersPlugin {
      _marker: std::marker::PhantomData,
    }
  }

  fn is_container(name: &str) -> bool {
    matches!(
      name,
      "a"
        | "defs"
        | "g"
        | "marker"
        | "mask"
        | "missing-glyph"
        | "pattern"
        | "switch"
        | "symbol"
        | "clipPath"
    )
  }

  fn prune_containers(
    &self,
    children: &mut BumpVec<'a, XMLAstChild<'a>>,
    parent_name: Option<&str>,
    removed_ids: &mut HashSet<String>,
  ) {
    let mut i = 0usize;
    while i < children.len() {
      let mut should_remove = false;

      if let Some(XMLAstChild::Element(el)) = children.get_mut(i) {
        self.prune_containers(&mut el.children, Some(el.name), removed_ids);

        if el.name != "svg" && Self::is_container(el.name) && el.children.is_empty() {
          if el.name == "pattern" && !el.attributes.is_empty() {
            i += 1;
            continue;
          }
          if el.name == "mask" && el.attributes.iter().any(|(k, _)| *k == "id") {
            i += 1;
            continue;
          }
          if parent_name == Some("switch") {
            i += 1;
            continue;
          }
          if el.name == "g" && el.attributes.iter().any(|(k, _)| *k == "filter") {
            i += 1;
            continue;
          }

          if let Some((_, id)) = el.attributes.iter().find(|(k, _)| *k == "id") {
            removed_ids.insert((*id).to_string());
          }
          should_remove = true;
        }
      }

      if should_remove {
        children.remove(i);
      } else {
        i += 1;
      }
    }
  }
}

impl<'a> Plugin<'a> for RemoveEmptyContainersPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    let mut removed_ids = HashSet::new();
    self.prune_containers(&mut root.children, None, &mut removed_ids);
  }
}
