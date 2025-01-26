use crate::plugin_pipeline::XmlPlugin;
use crate::xml_ast::{XmlElement, XmlNode};

pub struct UppercaseIdPlugin;

impl XmlPlugin for UppercaseIdPlugin {
  fn process(&self, element: &mut XmlElement) {
    fn traverse(element: &mut XmlElement) {
      // 转换 id 为大写
      if let Some(id) = element.attributes.get_mut("id") {
        *id = id.to_uppercase();
      }

      for child in &mut element.children {
        if let XmlNode::Element(elem) = child {
          traverse(elem);
        }
      }
    }

    traverse(element);
  }
}
