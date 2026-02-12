use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::BytesText;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

/// <!DOCTYPE ...>
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstDoctype<'arena> {
  pub name: &'arena str,
  pub data: XMLAstDoctypeData<'arena>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstDoctypeData<'arena> {
  pub doctype: &'arena str,
}

/// <?instruction ...?>
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstInstruction<'arena> {
  pub name: &'arena str,
  pub value: &'arena str,
}

/// <!-- comment -->
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstComment<'arena> {
  pub value: &'arena str,
}

/// <![CDATA[ ... ]]>
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstCdata<'arena> {
  pub value: &'arena str,
}

/// <?xml ... ?>
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstDecl<'arena> {
  pub value: &'arena str,
}

/// 文本节点
#[derive(Debug, Clone)]
pub struct XMLAstText<'arena> {
  pub value: &'arena str,
}

/// 元素节点
#[derive(Debug, Clone)]
pub struct XMLAstElement<'arena> {
  pub name: &'arena str,
  pub attributes: BumpVec<'arena, (&'arena str, &'arena str)>,
  pub children: BumpVec<'arena, XMLAstChild<'arena>>,
}

/// XMLAstChild: 所有非根节点的合集
#[derive(Debug, Clone)]
pub enum XMLAstChild<'arena> {
  Doctype(XMLAstDoctype<'arena>),
  Instruction(XMLAstInstruction<'arena>),
  Comment(XMLAstComment<'arena>),
  Cdata(XMLAstCdata<'arena>),
  Text(XMLAstText<'arena>),
  Element(XMLAstElement<'arena>),
  Decl(XMLAstDecl<'arena>),
}

/// 根节点
#[derive(Debug, Clone)]
pub struct XMLAstRoot<'arena> {
  pub children: BumpVec<'arena, XMLAstChild<'arena>>,
}

const TEXT_ELEMS: [&str; 12] = [
  "a",
  "altGlyph",
  "altGlyphDef",
  "altGlyphItem",
  "glyph",
  "glyphRef",
  "text",
  "textPath",
  "tref",
  "tspan",
  "pre",
  "title",
];

const ENTITY_DECLARATION_PATTERN: &str = r#"<!ENTITY\s+(\S+)\s+(?:'([^']+)'|\"([^\"]+)\")\s*>"#;

fn is_text_elem(name: &str) -> bool {
  TEXT_ELEMS.contains(&name)
}

fn decode_xml_entities(
  value: &str,
  entities: &HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
  let mut out = String::with_capacity(value.len());
  let mut i = 0;
  while i < value.len() {
    let ch = value[i..]
      .chars()
      .next()
      .ok_or_else(|| "invalid utf-8 boundary while decoding entities".to_string())?;
    if ch != '&' {
      out.push(ch);
      i += ch.len_utf8();
      continue;
    }

    let tail = &value[i + 1..];
    let Some(end_rel) = tail.find(';') else {
      return Err("unterminated entity reference".into());
    };
    let entity_name = &tail[..end_rel];
    let replacement = if let Some(hex) = entity_name.strip_prefix("#x") {
      let code = u32::from_str_radix(hex, 16)?;
      char::from_u32(code)
        .ok_or_else(|| format!("invalid hex char reference: {entity_name}"))?
        .to_string()
    } else if let Some(dec) = entity_name.strip_prefix('#') {
      let code = dec.parse::<u32>()?;
      char::from_u32(code)
        .ok_or_else(|| format!("invalid dec char reference: {entity_name}"))?
        .to_string()
    } else {
      match entity_name {
        "amp" => "&".to_string(),
        "lt" => "<".to_string(),
        "gt" => ">".to_string(),
        "quot" => "\"".to_string(),
        "apos" => "'".to_string(),
        _ => entities
          .get(entity_name)
          .cloned()
          .ok_or_else(|| format!("unrecognized entity `{entity_name}`"))?,
      }
    };
    out.push_str(&replacement);
    i += 2 + end_rel;
  }
  Ok(out)
}

fn parse_doctype_entities(doctype: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
  let re = Regex::new(ENTITY_DECLARATION_PATTERN)?;
  let mut entities = HashMap::new();
  for cap in re.captures_iter(doctype) {
    let name = cap
      .get(1)
      .ok_or_else(|| "entity name is missing".to_string())?
      .as_str()
      .to_string();
    let value = cap
      .get(2)
      .or_else(|| cap.get(3))
      .ok_or_else(|| "entity value is missing".to_string())?
      .as_str()
      .to_string();
    entities.insert(name, value);
  }
  Ok(entities)
}

fn parse_attributes<'a>(
  attributes: Attributes<'_>,
  reader: &Reader<&[u8]>,
  entities: &HashMap<String, String>,
  arena: &'a Bump,
) -> Result<BumpVec<'a, (&'a str, &'a str)>, Box<dyn Error>> {
  let mut attrs_vec = BumpVec::new_in(arena);
  for attr_result in attributes {
    let attr = attr_result?;
    let cow = reader.decoder().decode(attr.key.as_ref())?;
    let key: &str = arena.alloc_str(&cow);
    let decoded_attr = reader.decoder().decode(attr.value.as_ref())?;
    let decoded_value = decode_xml_entities(&decoded_attr, entities)?;
    let value = arena.alloc_str(&decoded_value);
    attrs_vec.push((key, &*value));
  }
  Ok(attrs_vec)
}

/// 解码 quick_xml 的字节切片并在 arena 中分配
fn decode_bytes<'arena>(
  bytes: &[u8],
  reader: &Reader<&[u8]>,
  arena: &'arena Bump,
) -> Result<&'arena str, Box<dyn Error>> {
  let cow = reader.decoder().decode(bytes)?;
  Ok(arena.alloc_str(&cow))
}

/// 解码包含转义字符的文本内容并在 arena 中分配
fn decode_escaped<'arena>(
  bytes_text: &BytesText,
  reader: &Reader<&[u8]>,
  entities: &HashMap<String, String>,
  arena: &'arena Bump,
) -> Result<&'arena str, Box<dyn Error>> {
  let cow = reader.decoder().decode(bytes_text.as_ref())?;
  let decoded = decode_xml_entities(&cow, entities)?;
  Ok(arena.alloc_str(&decoded))
}

pub fn parse_svg<'arena>(
  svg_string: &'arena str,
  arena: &'arena Bump,
) -> Result<XMLAstRoot<'arena>, Box<dyn Error>> {
  let mut reader = Reader::from_str(svg_string);
  reader.config_mut().trim_text(false);

  let mut root = XMLAstRoot {
    children: BumpVec::new_in(arena),
  };
  let mut entities: HashMap<String, String> = HashMap::new();
  let mut seen_root_element = false;
  // 使用栈来追踪父元素，Vec<XMLAstElement> 用于存储正在构建中的元素
  let mut parent_stack: Vec<XMLAstElement<'arena>> = Vec::new();
  // 缓冲区，用于 read_event_into
  let mut buf = Vec::new();

  loop {
    match reader.read_event_into(&mut buf) {
      Ok(Event::Start(e)) => {
        let name = decode_bytes(e.name().as_ref(), &reader, arena)?;
        let attributes = parse_attributes(e.attributes(), &reader, &entities, arena)?;
        let element = XMLAstElement {
          name,
          attributes,
          children: BumpVec::new_in(arena),
        };
        if parent_stack.is_empty() {
          seen_root_element = true;
        }
        parent_stack.push(element);
      }
      // --- 结束标签 </tag> ---
      Ok(Event::End(_e)) => {
        if let Some(finished_element) = parent_stack.pop() {
          let child_node = XMLAstChild::Element(finished_element);
          // 如果栈不为空，说明它有父元素，将其添加到父元素的 children 中
          if let Some(parent) = parent_stack.last_mut() {
            parent.children.push(child_node);
          } else {
            // 如果栈为空，说明这是顶级元素，添加到根节点的 children 中
            root.children.push(child_node);
          }
        } else {
          // 错误：遇到了没有匹配开始标签的结束标签
          return Err(
            format!(
              "Unexpected closing tag near position {}",
              reader.buffer_position()
            )
            .into(),
          );
        }
      }
      // --- 空标签 <tag ... /> ---
      Ok(Event::Empty(e)) => {
        let name = decode_bytes(e.name().as_ref(), &reader, arena)?;
        let attributes = parse_attributes(e.attributes(), &reader, &entities, arena)?;
        let element = XMLAstElement {
          name,
          attributes,
          children: BumpVec::new_in(arena),
        };
        if parent_stack.is_empty() {
          seen_root_element = true;
        }
        let child_node = XMLAstChild::Element(element);

        // 将空元素添加到当前父元素（栈顶）或根节点
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(child_node);
        } else {
          root.children.push(child_node);
        }
      }
      // --- 文本节点 ---
      Ok(Event::Text(e)) => {
        if let Some(parent) = parent_stack.last_mut() {
          let value = decode_escaped(&e, &reader, &entities, arena)?;
          if is_text_elem(parent.name) {
            parent.children.push(XMLAstChild::Text(XMLAstText { value }));
          } else {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
              parent.children.push(XMLAstChild::Text(XMLAstText {
                value: arena.alloc_str(trimmed),
              }));
            }
          }
        } else {
          let value = decode_escaped(&e, &reader, &entities, arena)?;
          if !value.trim().is_empty() {
            if seen_root_element {
              return Err("Text data outside of root node.".into());
            }
            return Err("Non-whitespace before first tag.".into());
          }
        }
      }
      // --- 注释 ---
      Ok(Event::Comment(e)) => {
        let value = decode_bytes(e.as_ref(), &reader, arena)?;
        let trimmed = value.trim();
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(XMLAstChild::Comment(XMLAstComment {
            value: arena.alloc_str(trimmed),
          }));
        } else {
          root.children.push(XMLAstChild::Comment(XMLAstComment {
            value: arena.alloc_str(trimmed),
          }));
        }
      }
      // --- CDATA <![CDATA[ ... ]]> ---
      Ok(Event::CData(e)) => {
        // CDATA 内容通常不需要 unescape，直接解码即可
        let value = decode_bytes(e.as_ref(), &reader, arena)?;
        let cdata_node = XMLAstChild::Cdata(XMLAstCdata { value });
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(cdata_node);
        } else {
          root.children.push(cdata_node);
        }
      }
      // --- Doctype <!DOCTYPE ...> ---
      Ok(Event::DocType(e)) => {
        let content = decode_bytes(e.as_ref(), &reader, arena)?;
        for (name, value) in parse_doctype_entities(content)? {
          entities.insert(name, value);
        }
        // 尝试从内容中提取第一个词作为名称（例如 <!DOCTYPE svg ...> 中的 "svg"）
        let name = content.split_whitespace().next().unwrap_or("");
        let doctype_node = XMLAstChild::Doctype(XMLAstDoctype {
          name,
          data: XMLAstDoctypeData { doctype: content },
        });
        // Doctype 通常在根级别
        if parent_stack.is_empty() {
          root.children.push(doctype_node);
        } else {
          // 在元素内部发现 Doctype
          eprintln!(
            "Warning: Found DOCTYPE inside an element near position {}",
            reader.buffer_position()
          );
          parent_stack.last_mut().unwrap().children.push(doctype_node);
        }
      }
      // --- Processing Instruction <? ... ?> ---
      Ok(Event::PI(e)) => {
        let content = decode_bytes(e.as_ref(), &reader, arena)?;
        // 将内容按第一个空格分割为 name 和 value
        let mut parts = content.splitn(2, |c: char| c.is_whitespace());
        let name = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("").trim_start();

        let pi_node = XMLAstChild::Instruction(XMLAstInstruction { name, value });
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(pi_node);
        } else {
          root.children.push(pi_node);
        }
      }
      // --- XML Declaration <?xml ...?> ---
      Ok(Event::Decl(e)) => {
        let value = decode_bytes(e.as_ref(), &reader, arena)?;
        let cdata_node = XMLAstChild::Decl(XMLAstDecl { value });
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(cdata_node);
        } else {
          root.children.push(cdata_node);
        }
      }
      // --- 文件结束 ---
      Ok(Event::Eof) => break,
      Err(e) => return Err(Box::new(e)),
    }

    // 清空缓冲区为下一次读取事件做准备
    buf.clear();
  }

  Ok(root)
}
