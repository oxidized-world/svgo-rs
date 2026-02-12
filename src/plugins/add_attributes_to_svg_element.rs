use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub enum AttributeSpec<'a> {
  Name(&'a str),
  KeyValue(&'a str, &'a str),
}

/// Add attributes to the outer <svg> element.
pub struct AddAttributesToSVGElementPlugin<'a> {
  pub arena: &'a Bump,
  pub attributes: Vec<AttributeSpec<'a>>,
}

pub struct AddAttributesToSVGElementPluginConfig<'a> {
  pub attributes: Vec<AttributeSpec<'a>>,
}

impl<'a> AddAttributesToSVGElementPlugin<'a> {
  pub fn new(config: AddAttributesToSVGElementPluginConfig<'a>, arena: &'a Bump) -> Self {
    AddAttributesToSVGElementPlugin {
      arena,
      attributes: config.attributes,
    }
  }

  fn has_attr(el: &XMLAstElement<'a>, name: &str) -> bool {
    el.attributes.iter().any(|(k, _)| *k == name)
  }
}

impl<'a> Plugin<'a> for AddAttributesToSVGElementPlugin<'a> {
  fn root_enter(&self, root: &mut XMLAstRoot<'a>) {
    for child in &mut root.children {
      let XMLAstChild::Element(el) = child else {
        continue;
      };

      if el.name != "svg" {
        continue;
      }

      for attr in &self.attributes {
        match attr {
          AttributeSpec::Name(name) => {
            if !Self::has_attr(el, name) {
              el.attributes.push((*name, self.arena.alloc_str("")));
            }
          }
          AttributeSpec::KeyValue(key, value) => {
            if !Self::has_attr(el, key) {
              el.attributes.push((*key, *value));
            }
          }
        }
      }
    }
  }
}
