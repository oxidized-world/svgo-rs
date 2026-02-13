use std::collections::HashSet;

use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct RemoveUnusedNSPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
}

pub struct RemoveUnusedNSPluginConfig {}

impl<'a> RemoveUnusedNSPlugin<'a> {
  pub fn new(_config: RemoveUnusedNSPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveUnusedNSPlugin {
      _marker: std::marker::PhantomData,
    }
  }

  fn collect_used_ns(children: &[XMLAstChild<'a>], used: &mut HashSet<String>) {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        if let Some((ns, _)) = el.name.split_once(':') {
          used.insert(ns.to_string());
        }
        for (name, _) in &el.attributes {
          if let Some((ns, _)) = name.split_once(':') {
            used.insert(ns.to_string());
          }
        }
        Self::collect_used_ns(&el.children, used);
      }
    }
  }

  fn find_root_svg_mut<'b>(
    children: &'b mut [XMLAstChild<'a>],
  ) -> Option<&'b mut XMLAstElement<'a>> {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        if el.name == "svg" {
          return Some(el);
        }
      }
    }
    None
  }
}

impl<'a> Plugin<'a> for RemoveUnusedNSPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    let mut used = HashSet::new();
    Self::collect_used_ns(&root.children, &mut used);

    if let Some(svg) = Self::find_root_svg_mut(&mut root.children) {
      svg.attributes.retain(|(name, _)| {
        if !name.starts_with("xmlns:") {
          return true;
        }
        let prefix = &name["xmlns:".len()..];
        used.contains(prefix)
      });
    }
  }
}
