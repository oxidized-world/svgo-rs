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
    // 创建带有名称的开始元素
    let start = WriteEvent::start_element(Name::local(&element.name));

    // 添加属性（使用新的 API 方式）
    let mut start_with_attrs = start;
    for (k, v) in &element.attributes {
      start_with_attrs = start_with_attrs.attr(Name::local(k), v);
    }

    // 写入开始标签
    emitter.write(start_with_attrs)?;

    // 递归处理子节点
    for child in &element.children {
      match child {
        XmlNode::Element(e) => write_element(e, emitter)?,
        XmlNode::Text(t) => emitter.write(WriteEvent::characters(t))?,
      }
    }

    // 创建结束标签
    let end_name = Name::local(&element.name);
    emitter.write(WriteEvent::end_element().name(end_name))?;

    Ok(())
  }

  write_element(element, &mut emitter)?;
  Ok(String::from_utf8(writer)?)
}
