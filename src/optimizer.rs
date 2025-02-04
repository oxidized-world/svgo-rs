// src/optimizer.rs
use crate::dom::{Decl, SvgElement, SvgNode};
use crate::error::Result;
use crate::plugins::Plugin;
use quick_xml::events::{BytesDecl, BytesStart, BytesText};
use quick_xml::{events::Event, Reader, Writer};

pub struct SvgOptimizer {
  plugins: Vec<Box<dyn Plugin>>,
}

impl SvgOptimizer {
  pub fn new(plugins: Vec<Box<dyn Plugin>>) -> Self {
    Self { plugins }
  }

  pub fn vec_to_string(&self, vec: &Vec<u8>) -> std::result::Result<String, std::str::Utf8Error> {
    // 将 &mut Vec<u8> 转为 &[u8] 切片
    let bytes = &**vec;
    // 转换为 &str，再转为 String
    std::str::from_utf8(bytes).map(|s| s.to_string())
  }

  pub fn optimize(&self, input: &[u8]) -> Result<Vec<u8>> {
    let mut reader = Reader::from_reader(input);
    reader.config_mut().trim_text(true);

    let mut writer = Writer::new(Vec::new());
    let mut stack = Vec::new(); // 用于跟踪元素层级

    let mut current_element: Option<SvgElement> = SvgElement::new("root").into();

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

        Event::DocType(text) => {
          // 处理文档类型声明
          let content = text.unescape()?.into_owned();
          if !content.trim().is_empty() {
            let elem = SvgNode::DocType(content);
            current_element.as_mut().unwrap().add_child(elem);
            if let Some(parent) = &mut current_element {
              self.process_element(parent)?;
            }
          }
        }

        Event::Comment(text) => {
          // 处理注释
          let content = text.unescape()?.into_owned();
          if !content.trim().is_empty() {
            let elem = SvgNode::Comment(content);
            current_element.as_mut().unwrap().add_child(elem);
            if let Some(parent) = &mut current_element {
              self.process_element(parent)?;
            }
          }
        }

        Event::Decl(d) => {
          let mut encoding: Option<String> = Some("".to_string());
          let mut standalone: Option<String> = Some("".to_string());

          if let Some(dd) = d.encoding() {
            match dd {
              Ok(res) => encoding = Some(String::from_utf8_lossy(&res).into_owned()),
              Err(err) => panic!("parse encoding value error: {:?}", err),
            }
          }

          if let Some(dd) = d.standalone() {
            match dd {
              Ok(res) => standalone = Some(String::from_utf8_lossy(&res).into_owned()),
              Err(err) => panic!("parse standalone value error: {:?}", err),
            }
          }

          let elem = SvgNode::Decl(Decl {
            version: self.vec_to_string(d.version().unwrap().to_mut()).unwrap(),
            encoding: encoding,
            standalone: standalone,
          });
          current_element.as_mut().unwrap().add_child(elem);
          if let Some(parent) = &mut current_element {
            self.process_element(parent)?;
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
    let name = element.name;
    let mut start = BytesStart::new(&name);
    // ignore root element, because it's not a real SVG element
    if name != "root" {
      for (k, v) in &element.attributes {
        start.push_attribute((k.as_str(), v.as_str()));
      }

      match writer.write_event(Event::Start(start.clone())) {
        Ok(_) => {}
        Err(e) => panic!("Error writing event: {:?}", e),
      }
    }

    for child in element.children {
      match child {
        SvgNode::Element(e) => self.serialize_element(e, writer)?,
        SvgNode::Text(t) => match writer.write_event(Event::Text(BytesText::new(&t))) {
          Ok(_) => {}
          Err(e) => panic!("Error writing event: {:?}", e),
        },
        SvgNode::DocType(t) => match writer.write_event(Event::DocType(BytesText::new(&t))) {
          Ok(_) => {}
          Err(e) => panic!("Error writing event: {:?}", e),
        },
        SvgNode::Comment(c) => match writer.write_event(Event::Comment(BytesText::new(&c))) {
          Ok(_) => {}
          Err(e) => panic!("Error writing event: {:?}", e),
        },
        SvgNode::Decl(d) => match writer.write_event(Event::Decl(BytesDecl::new(
          &d.version,
          d.encoding.as_deref(),
          d.standalone.as_deref(),
        ))) {
          Ok(_) => {}
          Err(e) => panic!("Error writing event: {:?}", e),
        },
      }
    }

    // ignore root element, because it's not a real SVG element
    if name != "root" {
      match writer.write_event(Event::End(start.to_end())) {
        Ok(_) => {}
        Err(e) => panic!("Error writing event: {:?}", e),
      }
    }
    Ok(())
  }
}
