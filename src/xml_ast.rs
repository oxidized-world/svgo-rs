use std::collections::HashMap;
use std::io::Cursor;
use xml::name::Name;
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EmitterConfig, EventWriter, XmlEvent as WriteEvent};

// ====================== DOM 结构定义 ======================
#[derive(Debug, Clone)]
pub enum XmlNode {
  Element(XmlElement),
  Text(String),
}

#[derive(Debug, Clone)]
pub struct XmlElement {
  pub name: String,
  pub attributes: HashMap<String, String>,
  pub children: Vec<XmlNode>,
}

// ====================== XML 解析/序列化 ======================
pub fn parse_xml(input: &str) -> Result<XmlElement, Box<dyn std::error::Error>> {
  let parser = EventReader::new(Cursor::new(input));
  let mut stack = Vec::new();
  let mut root = None;

  for event in parser {
    match event? {
      XmlEvent::StartElement {
        name, attributes, ..
      } => {
        let element = XmlElement {
          name: name.local_name,
          attributes: attributes
            .into_iter()
            .map(|a| (a.name.local_name, a.value))
            .collect(),
          children: Vec::new(),
        };
        stack.push(element);
      }
      XmlEvent::EndElement { .. } => {
        if let Some(element) = stack.pop() {
          if let Some(parent) = stack.last_mut() {
            parent.children.push(XmlNode::Element(element));
          } else {
            root = Some(element);
          }
        }
      }
      XmlEvent::Characters(text) => {
        if let Some(parent) = stack.last_mut() {
          parent.children.push(XmlNode::Text(text));
        }
      }
      _ => {}
    }
  }

  root.ok_or("Invalid XML: no root element".into())
}

pub fn serialize_xml(element: &XmlElement) -> Result<String, Box<dyn std::error::Error>> {
  let mut writer = Vec::new();
  let mut emitter = EmitterConfig::new()
    .perform_indent(true)
    .create_writer(&mut writer);

  fn write_element<W: std::io::Write>(
    element: &XmlElement,
    emitter: &mut EventWriter<W>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    // 创建基础元素构建器
    let mut element_builder = WriteEvent::start_element(Name::local(&element.name));

    // 添加属性（使用单独的 attr 方法）
    for (k, v) in &element.attributes {
      element_builder = element_builder.attr(Name::local(k), v.as_str());
    }

    // 写入开始标签
    emitter.write(element_builder)?;

    // 处理子节点
    for child in &element.children {
      match child {
        XmlNode::Element(e) => write_element(e, emitter)?,
        XmlNode::Text(t) => emitter.write(WriteEvent::characters(t))?,
      }
    }

    // 写入结束标签
    emitter.write(WriteEvent::end_element())?;
    // emitter.write(WriteEvent::start_element(name).attributes(attributes))?;

    for child in &element.children {
      match child {
        XmlNode::Element(e) => write_element(e, emitter)?,
        XmlNode::Text(t) => emitter.write(WriteEvent::characters(t))?,
      }
    }

    emitter.write(WriteEvent::end_element())?;
    Ok(())
  }

  write_element(element, &mut emitter)?;
  Ok(String::from_utf8(writer)?)
}
