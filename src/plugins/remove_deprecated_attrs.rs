use std::collections::HashSet;

use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct RemoveDeprecatedAttrsPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
  pub remove_unsafe: bool,
  reg_attr_in_selector: Regex,
}

pub struct RemoveDeprecatedAttrsPluginConfig {
  pub remove_unsafe: bool,
}

impl<'a> RemoveDeprecatedAttrsPlugin<'a> {
  pub fn new(config: RemoveDeprecatedAttrsPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveDeprecatedAttrsPlugin {
      _marker: std::marker::PhantomData,
      remove_unsafe: config.remove_unsafe,
      reg_attr_in_selector: Regex::new(r"\[\s*([A-Za-z_:][\w:.-]*)").unwrap(),
    }
  }

  fn collect_attrs_in_stylesheet(
    &self,
    children: &BumpVec<'a, XMLAstChild<'a>>,
    attrs: &mut HashSet<String>,
  ) {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        if el.name == "style" {
          for style_child in &el.children {
            match style_child {
              XMLAstChild::Text(text) => {
                for cap in self.reg_attr_in_selector.captures_iter(text.value) {
                  if let Some(name) = cap.get(1) {
                    attrs.insert(name.as_str().to_string());
                  }
                }
              }
              XMLAstChild::Cdata(cdata) => {
                for cap in self.reg_attr_in_selector.captures_iter(cdata.value) {
                  if let Some(name) = cap.get(1) {
                    attrs.insert(name.as_str().to_string());
                  }
                }
              }
              _ => {}
            }
          }
        }
        self.collect_attrs_in_stylesheet(&el.children, attrs);
      }
    }
  }

  fn process_element(&self, el: &mut XMLAstElement<'a>, attrs_in_stylesheet: &HashSet<String>) {
    // Special case: xml:lang can be removed when lang exists and it is not referenced from stylesheet.
    if el.attributes.iter().any(|(k, _)| *k == "xml:lang")
      && el.attributes.iter().any(|(k, _)| *k == "lang")
      && !attrs_in_stylesheet.contains("xml:lang")
    {
      el.attributes.retain(|(k, _)| *k != "xml:lang");
    }

    let safe_deprecated = [
      "version",
      "zoomAndPan",
      "contentScriptType",
      "contentStyleType",
    ];
    let unsafe_deprecated = ["enable-background"];

    el.attributes.retain(|(name, _)| {
      if attrs_in_stylesheet.contains(*name) {
        return true;
      }
      if safe_deprecated.contains(name) {
        return false;
      }
      if self.remove_unsafe && unsafe_deprecated.contains(name) {
        return false;
      }
      true
    });

    for child in &mut el.children {
      if let XMLAstChild::Element(child_el) = child {
        self.process_element(child_el, attrs_in_stylesheet);
      }
    }
  }
}

impl<'a> Plugin<'a> for RemoveDeprecatedAttrsPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    let mut attrs_in_stylesheet: HashSet<String> = HashSet::new();
    self.collect_attrs_in_stylesheet(&root.children, &mut attrs_in_stylesheet);

    for child in &mut root.children {
      if let XMLAstChild::Element(el) = child {
        self.process_element(el, &attrs_in_stylesheet);
      }
    }
  }
}
