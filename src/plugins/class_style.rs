use crate::xml_ast::{XmlElement, XmlNode};
use crate::plugin_pipeline::XmlPlugin;

pub struct ClassStylePlugin;

impl XmlPlugin for ClassStylePlugin {
  fn process(&self, element: &mut XmlElement) {
    fn traverse(element: &mut XmlElement) {
      // 修改 class 属性对应的 style
      if let Some(class) = element.attributes.get("class") {
        element.attributes.insert(
          "style".to_string(),
          format!("generated-style-for-{}", class),
        );
      }

      // 递归处理子元素
      for child in &mut element.children {
        if let XmlNode::Element(elem) = child {
          traverse(elem);
        }
      }
    }

    traverse(element);
  }
}
