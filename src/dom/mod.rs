// src/dom/mod.rs
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SvgElement {
  pub name: String,
  pub attributes: HashMap<String, String>,
  pub children: Vec<SvgNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SvgNode {
  Element(SvgElement),
  Text(String),
  DocType(String),
}

impl SvgElement {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      attributes: HashMap::new(),
      children: Vec::new(),
    }
  }

  pub fn add_attribute(&mut self, key: &str, value: &str) {
    self.attributes.insert(key.to_string(), value.to_string());
  }

  pub fn add_child(&mut self, child: SvgNode) {
    self.children.push(child);
  }

  pub fn get_attribute(&self, key: &str) -> Option<&String> {
    self.attributes.get(key)
  }

  pub fn remove_attribute(&mut self, key: &str) -> Option<String> {
    self.attributes.remove(key)
  }
}
