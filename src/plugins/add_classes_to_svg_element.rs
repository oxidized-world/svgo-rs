use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

/// Add classnames to the outer <svg> element.
pub struct AddClassesToSVGElementPlugin<'a> {
  pub arena: &'a Bump,
  pub class_names: Vec<&'a str>,
}

pub struct AddClassesToSVGElementPluginConfig<'a> {
  pub class_names: Vec<&'a str>,
}

impl<'a> AddClassesToSVGElementPlugin<'a> {
  pub fn new(config: AddClassesToSVGElementPluginConfig<'a>, arena: &'a Bump) -> Self {
    AddClassesToSVGElementPlugin {
      arena,
      class_names: config.class_names,
    }
  }

  fn get_attr_value<'b>(el: &'b XMLAstElement<'a>, name: &str) -> Option<&'b str> {
    el.attributes.iter().find(|(k, _)| *k == name).map(|(_, v)| *v)
  }

  fn set_attr_value(el: &mut XMLAstElement<'a>, name: &'a str, value: &'a str) {
    for (k, v) in &mut el.attributes {
      if *k == name {
        *v = value;
        return;
      }
    }
    el.attributes.push((name, value));
  }
}

impl<'a> Plugin<'a> for AddClassesToSVGElementPlugin<'a> {
  fn root_enter(&self, root: &mut XMLAstRoot<'a>) {
    for child in &mut root.children {
      let XMLAstChild::Element(el) = child else {
        continue;
      };

      if el.name != "svg" {
        continue;
      }

      let mut classes: Vec<&str> =
        Self::get_attr_value(el, "class").unwrap_or("").split_whitespace().collect();

      for class_name in &self.class_names {
        if class_name.is_empty() {
          continue;
        }
        if !classes.contains(class_name) {
          classes.push(class_name);
        }
      }

      let value = self.arena.alloc_str(&classes.join(" "));
      Self::set_attr_value(el, "class", value);
    }
  }
}
