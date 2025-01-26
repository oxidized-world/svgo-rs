use crate::xml_ast::{parse_xml, serialize_xml, XmlElement};

pub trait XmlPlugin {
  fn process(&self, element: &mut XmlElement);
}

pub struct PluginPipeline {
  plugins: Vec<Box<dyn XmlPlugin>>,
}

impl PluginPipeline {
  pub fn new() -> Self {
    Self {
      plugins: Vec::new(),
    }
  }

  pub fn add_plugin(&mut self, plugin: Box<dyn XmlPlugin>) {
    self.plugins.push(plugin);
  }

  pub fn process(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut dom = parse_xml(input)?;
    for plugin in &self.plugins {
      plugin.process(&mut dom);
    }
    serialize_xml(&dom)
  }
}
