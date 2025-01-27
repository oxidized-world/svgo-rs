// src/optimizer.rs
use crate::dom::{SvgElement, SvgNode};
use crate::error::Result;
use crate::plugins::Plugin;
use quick_xml::events::{BytesStart, BytesText};
use quick_xml::{events::Event, Reader, Writer};

pub struct SvgOptimizer {
  plugins: Vec<Box<dyn Plugin>>,
}

impl SvgOptimizer {
  pub fn new(plugins: Vec<Box<dyn Plugin>>) -> Self {
    Self { plugins }
  }

  pub fn optimize(&self, input: &[u8]) -> Result<Vec<u8>> {
    let mut reader = Reader::from_reader(input);
    reader.config_mut().trim_text(true);

    let mut writer = Writer::new(Vec::new());
    let mut stack = Vec::new(); // 用于跟踪元素层级

    let mut current_element: Option<SvgElement> = None;

    loop {
      match reader.read_event()? {
        Event::Start(e) => {
          let elem = self.parse_start_element(e);

          // 将当前元素压栈，开始处理新元素
          if let Some(parent) = current_element.take() {
            stack.push(parent);
          }
          current_element = Some(elem);
        }

        Event::End(_) => {
          // 结束当前元素处理，弹出父元素
          if let Some(mut elem) = current_element.take() {
            // 处理插件
            self.process_element(&mut elem)?;

            if let Some(mut parent) = stack.pop() {
              parent.add_child(SvgNode::Element(elem));
              current_element = Some(parent);
            } else {
              // 到达根元素
              current_element = Some(elem);
            }
          }
        }

        Event::Text(text) => {
          // 处理文本节点
          if let Some(elem) = &mut current_element {
            let content = text.unescape()?.into_owned();
            if !content.trim().is_empty() {
              elem.add_child(SvgNode::Text(content));
            }
          }
        }

        Event::Eof => break,
        _ => {}
      }
    }

    // 序列化最终元素
    if let Some(root) = current_element {
      self.serialize_element(root, &mut writer)?;
    }
    Ok(writer.into_inner())
  }

  fn parse_start_element(&self, e: BytesStart) -> SvgElement {
    let mut element: SvgElement = SvgElement::new(std::str::from_utf8(e.name().as_ref()).unwrap());

    for attr in e.attributes() {
      let attr = attr.unwrap();
      let key = std::str::from_utf8(attr.key.as_ref()).unwrap();
      let value = attr.unescape_value().unwrap();
      element.add_attribute(key, &value);
    }

    element
  }

  fn process_element(&self, element: &mut SvgElement) -> Result<()> {
    for plugin in &self.plugins {
      plugin.process_element(element)?;
    }

    for child in &mut element.children {
      if let SvgNode::Element(child_elem) = child {
        self.process_element(child_elem)?;
      }
    }

    Ok(())
  }

  fn serialize_element(
    &self,
    element: SvgElement,
    writer: &mut quick_xml::Writer<Vec<u8>>,
  ) -> Result<()> {
    let mut start = BytesStart::new(&element.name);
    for (k, v) in &element.attributes {
      start.push_attribute((k.as_str(), v.as_str()));
    }

    match writer.write_event(Event::Start(start.clone())) {
      Ok(_) => {}
      Err(e) => panic!("Error writing event: {:?}", e),
    }

    for child in element.children {
      match child {
        SvgNode::Element(e) => self.serialize_element(e, writer)?,
        SvgNode::Text(t) => {
          match writer.write_event(Event::Text(BytesText::new(&t))) {
            Ok(_) => {}
            Err(e) => panic!("Error writing event: {:?}", e),
          }
        }
      }
    }

    match writer.write_event(Event::End(start.to_end())) {
      Ok(_) => {}
      Err(e) => panic!("Error writing event: {:?}", e),
    }
    Ok(())
  }
}
