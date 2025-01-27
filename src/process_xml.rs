use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use quick_xml::Error;
use std::io::Cursor;

// 插件 trait 定义
pub trait XmlPlugin {
  fn handle_start_element(&mut self, name: &[u8], attrs: &mut Vec<(String, String)>);
}

type Attrs = Vec<(String, String)>;

fn parse_attributes(start: &BytesStart) -> Result<Attrs, Error> {
  let mut attrs = Vec::new();

  for attr_result in start.attributes() {
    let attr = attr_result?;

    // 转换为字符串（带 UTF-8 校验）
    let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();

    // 处理属性值
    let value = String::from_utf8_lossy(&attr.value).into_owned();

    attrs.push((key, value));
  }

  Ok(attrs)
}

// XML 处理主函数
pub fn process_xml(xml: &str, plugins: &mut Vec<Box<dyn XmlPlugin>>) -> Result<String, Error> {
  let mut reader = Reader::from_str(xml);
  let mut writer = Writer::new(Cursor::new(Vec::new()));

  // 保留 XML 声明
  // if let Some(decl) = reader.declaration() {
  //   writer.write_event(Event::Decl(decl.to_owned()))?;
  // }

  loop {
    match reader.read_event()? {
      Event::Start(start) => {
        let mut elem = start.to_owned();
        let mut attrs = parse_attributes(&start)?;
        plugins
          .iter_mut()
          .for_each(|p| p.handle_start_element(&elem.name().local_name().into_inner(), &mut attrs));
        elem.clear_attributes();
        attrs.iter().for_each(|(k, v)| {
          elem.push_attribute((k.as_bytes(), v.as_bytes()));
        });
        writer.write_event(Event::Start(elem))?;
      }
      Event::Empty(empty) => {
        let mut attrs = parse_attributes(&empty)?;
        plugins.iter_mut().for_each(|p| {
          p.handle_start_element(&empty.name().local_name().into_inner(), &mut attrs)
        });
        writer.write_event(Event::Empty(empty))?;
      }
      Event::Eof => break,
      other => writer.write_event(other)?,
    }
  }

  Ok(String::from_utf8(writer.into_inner().into_inner()).unwrap())
}
