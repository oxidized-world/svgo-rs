use std::collections::HashMap;

use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct ReusePathsPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct ReusePathsPluginConfig {}

impl<'a> ReusePathsPlugin<'a> {
  pub fn new(_config: ReusePathsPluginConfig, arena: &'a Bump) -> Self {
    ReusePathsPlugin { arena }
  }

  fn process_children(
    &self,
    children: &mut BumpVec<'a, XMLAstChild<'a>>,
    defs_paths: &mut Vec<XMLAstElement<'a>>,
    counter: &mut usize,
  ) {
    for child in children.iter_mut() {
      if let XMLAstChild::Element(el) = child {
        self.process_children(&mut el.children, defs_paths, counter);
      }
    }

    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    let mut group_order: Vec<String> = Vec::new();
    for (idx, child) in children.iter().enumerate() {
      if let XMLAstChild::Element(el) = child {
        if el.name == "path" {
          let d = el.attributes.iter().find(|(k, _)| *k == "d").map(|(_, v)| *v).unwrap_or("");
          if d.is_empty() {
            continue;
          }
          let fill =
            el.attributes.iter().find(|(k, _)| *k == "fill").map(|(_, v)| *v).unwrap_or("");
          let stroke = el
            .attributes
            .iter()
            .find(|(k, _)| *k == "stroke")
            .map(|(_, v)| *v)
            .unwrap_or("");
          let key = format!("{d};s:{stroke};f:{fill}");
          if !groups.contains_key(&key) {
            group_order.push(key.clone());
          }
          groups.entry(key).or_default().push(idx);
        }
      }
    }

    for key in group_order {
      let Some(indexes) = groups.get(&key) else {
        continue;
      };
      if indexes.len() <= 1 {
        continue;
      }

      let Some(XMLAstChild::Element(first_path)) = children.get(indexes[0]) else {
        continue;
      };
      let mut reusable = XMLAstElement {
        name: self.arena.alloc_str("path"),
        attributes: BumpVec::new_in(self.arena),
        children: BumpVec::new_in(self.arena),
      };
      for key in ["fill", "stroke", "d"] {
        if let Some((_, v)) = first_path.attributes.iter().find(|(k, _)| *k == key) {
          reusable.attributes.push((self.arena.alloc_str(key), *v));
        }
      }
      let first_id = first_path
        .attributes
        .iter()
        .find(|(k, _)| *k == "id")
        .map(|(_, v)| (*v).to_string());
      let id = if let Some(id) = first_id {
        id
      } else {
        let id = format!("reuse-{}", *counter);
        *counter += 1;
        id
      };
      reusable
        .attributes
        .push((self.arena.alloc_str("id"), self.arena.alloc_str(&id)));
      defs_paths.push(reusable.clone());

      for (pos, idx) in indexes.iter().enumerate() {
        if let Some(XMLAstChild::Element(path_el)) = children.get_mut(*idx) {
          path_el.name = self.arena.alloc_str("use");
          path_el.attributes.retain(|(k, _)| *k != "d" && *k != "fill" && *k != "stroke");
          if pos == 0 {
            path_el.attributes.retain(|(k, v)| !(*k == "id" && *v == id));
          }
          path_el.attributes.push((
            self.arena.alloc_str("xlink:href"),
            self.arena.alloc_str(&format!("#{id}")),
          ));
        }
      }
    }
  }
}

impl<'a> Plugin<'a> for ReusePathsPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    let mut defs_paths = Vec::new();
    let mut counter = 0usize;
    self.process_children(&mut root.children, &mut defs_paths, &mut counter);

    if defs_paths.is_empty() {
      return;
    }

    let mut defs_children = BumpVec::new_in(self.arena);
    for path in defs_paths {
      defs_children.push(XMLAstChild::Element(path));
    }
    let defs = XMLAstElement {
      name: self.arena.alloc_str("defs"),
      attributes: BumpVec::new_in(self.arena),
      children: defs_children,
    };

    for child in root.children.iter_mut() {
      if let XMLAstChild::Element(svg) = child {
        if svg.name == "svg" {
          if !svg.attributes.iter().any(|(k, _)| *k == "xmlns:xlink") {
            svg.attributes.push((
              self.arena.alloc_str("xmlns:xlink"),
              self.arena.alloc_str("http://www.w3.org/1999/xlink"),
            ));
          }
          svg.children.insert(0, XMLAstChild::Element(defs));
          return;
        }
      }
    }

    root.children.insert(0, XMLAstChild::Element(defs));
  }
}
