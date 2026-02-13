use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::BytesText;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;

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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstInstruction<'arena> {
  pub name: &'arena str,
  pub value: &'arena str,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstComment<'arena> {
  pub value: &'arena str,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstCdata<'arena> {
  pub value: &'arena str,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct XMLAstDecl<'arena> {
  pub value: &'arena str,
}

#[derive(Debug, Clone)]
pub struct XMLAstText<'arena> {
  pub value: &'arena str,
}

#[derive(Debug, Clone)]
pub struct XMLAstElement<'arena> {
  pub name: &'arena str,
  pub attributes: BumpVec<'arena, (&'arena str, &'arena str)>,
  pub children: BumpVec<'arena, XMLAstChild<'arena>>,
}

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
static ENTITY_DECLARATION_RE: OnceLock<Regex> = OnceLock::new();

fn is_text_elem(name: &str) -> bool {
  TEXT_ELEMS.contains(&name)
}

fn decode_xml_entities(
  value: &str,
  entities: &HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
  if !value.contains('&') {
    return Ok(value.to_string());
  }

  let mut out = String::with_capacity(value.len());
  let mut i = 0;
  while i < value.len() {
    let Some(amp_rel) = value[i..].find('&') else {
      out.push_str(&value[i..]);
      break;
    };

    let amp = i + amp_rel;
    if amp > i {
      out.push_str(&value[i..amp]);
    }

    let tail = &value[amp + 1..];
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
    i = amp + end_rel + 2;
  }

  Ok(out)
}

fn parse_doctype_entities(doctype: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
  let re = ENTITY_DECLARATION_RE.get_or_init(|| {
    Regex::new(ENTITY_DECLARATION_PATTERN).expect("ENTITY_DECLARATION_PATTERN must be valid")
  });

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

fn intern_string<'a>(cache: &mut HashMap<String, &'a str>, raw: &str, arena: &'a Bump) -> &'a str {
  if let Some(existing) = cache.get(raw) {
    return existing;
  }
  let interned = arena.alloc_str(raw);
  cache.insert(raw.to_string(), interned);
  interned
}

fn parse_attributes<'a>(
  attributes: Attributes<'_>,
  reader: &Reader<&[u8]>,
  entities: &HashMap<String, String>,
  interned_attr_names: &mut HashMap<String, &'a str>,
  arena: &'a Bump,
) -> Result<BumpVec<'a, (&'a str, &'a str)>, Box<dyn Error>> {
  let mut attrs_vec = BumpVec::new_in(arena);
  for attr_result in attributes {
    let attr = attr_result?;
    let cow = reader.decoder().decode(attr.key.as_ref())?;
    let key = intern_string(interned_attr_names, &cow, arena);

    let decoded_attr = reader.decoder().decode(attr.value.as_ref())?;
    if decoded_attr.contains('&') {
      let decoded_value = decode_xml_entities(&decoded_attr, entities)?;
      let value = arena.alloc_str(&decoded_value);
      attrs_vec.push((key, &*value));
    } else {
      let value = arena.alloc_str(&decoded_attr);
      attrs_vec.push((key, &*value));
    }
  }
  Ok(attrs_vec)
}

fn decode_bytes<'arena>(
  bytes: &[u8],
  reader: &Reader<&[u8]>,
  arena: &'arena Bump,
) -> Result<&'arena str, Box<dyn Error>> {
  let cow = reader.decoder().decode(bytes)?;
  Ok(arena.alloc_str(&cow))
}

fn decode_escaped<'arena>(
  bytes_text: &BytesText,
  reader: &Reader<&[u8]>,
  entities: &HashMap<String, String>,
  arena: &'arena Bump,
) -> Result<&'arena str, Box<dyn Error>> {
  let cow = reader.decoder().decode(bytes_text.as_ref())?;
  if cow.contains('&') {
    let decoded = decode_xml_entities(&cow, entities)?;
    Ok(arena.alloc_str(&decoded))
  } else {
    Ok(arena.alloc_str(&cow))
  }
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
  let mut interned_element_names: HashMap<String, &'arena str> = HashMap::new();
  let mut interned_attr_names: HashMap<String, &'arena str> = HashMap::new();
  let mut seen_root_element = false;
  let mut parent_stack: Vec<XMLAstElement<'arena>> = Vec::new();
  let mut buf = Vec::new();

  loop {
    match reader.read_event_into(&mut buf) {
      Ok(Event::Start(e)) => {
        let name = {
          let qname = e.name();
          let cow = reader.decoder().decode(qname.as_ref())?;
          intern_string(&mut interned_element_names, &cow, arena)
        };
        let attributes = parse_attributes(
          e.attributes(),
          &reader,
          &entities,
          &mut interned_attr_names,
          arena,
        )?;
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
      Ok(Event::End(_)) => {
        if let Some(finished_element) = parent_stack.pop() {
          let child_node = XMLAstChild::Element(finished_element);
          if let Some(parent) = parent_stack.last_mut() {
            parent.children.push(child_node);
          } else {
            root.children.push(child_node);
          }
        } else {
          return Err(
            format!(
              "Unexpected closing tag near position {}",
              reader.buffer_position()
            )
            .into(),
          );
        }
      }
      Ok(Event::Empty(e)) => {
        let name = {
          let qname = e.name();
          let cow = reader.decoder().decode(qname.as_ref())?;
          intern_string(&mut interned_element_names, &cow, arena)
        };
        let attributes = parse_attributes(
          e.attributes(),
          &reader,
          &entities,
          &mut interned_attr_names,
          arena,
        )?;
        let element = XMLAstElement {
          name,
          attributes,
          children: BumpVec::new_in(arena),
        };
        if parent_stack.is_empty() {
          seen_root_element = true;
        }
        let child_node = XMLAstChild::Element(element);
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(child_node);
        } else {
          root.children.push(child_node);
        }
      }
      Ok(Event::Text(e)) => {
        if let Some(parent) = parent_stack.last_mut() {
          if is_text_elem(parent.name) {
            let value = decode_escaped(&e, &reader, &entities, arena)?;
            parent.children.push(XMLAstChild::Text(XMLAstText { value }));
          } else {
            if e.as_ref().iter().all(|b| b.is_ascii_whitespace()) {
              buf.clear();
              continue;
            }
            let value = decode_escaped(&e, &reader, &entities, arena)?;
            let trimmed = value.trim();
            if !trimmed.is_empty() {
              parent.children.push(XMLAstChild::Text(XMLAstText {
                value: arena.alloc_str(trimmed),
              }));
            }
          }
        } else {
          if e.as_ref().iter().all(|b| b.is_ascii_whitespace()) {
            buf.clear();
            continue;
          }
          let value = decode_escaped(&e, &reader, &entities, arena)?;
          if !value.trim().is_empty() {
            if seen_root_element {
              return Err("Text data outside of root node.".into());
            }
            return Err("Non-whitespace before first tag.".into());
          }
        }
      }
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
      Ok(Event::CData(e)) => {
        let value = decode_bytes(e.as_ref(), &reader, arena)?;
        let cdata_node = XMLAstChild::Cdata(XMLAstCdata { value });
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(cdata_node);
        } else {
          root.children.push(cdata_node);
        }
      }
      Ok(Event::DocType(e)) => {
        let content = decode_bytes(e.as_ref(), &reader, arena)?;
        for (name, value) in parse_doctype_entities(content)? {
          entities.insert(name, value);
        }
        let name = content.split_whitespace().next().unwrap_or("");
        let doctype_node = XMLAstChild::Doctype(XMLAstDoctype {
          name,
          data: XMLAstDoctypeData { doctype: content },
        });
        if parent_stack.is_empty() {
          root.children.push(doctype_node);
        } else {
          eprintln!(
            "Warning: Found DOCTYPE inside an element near position {}",
            reader.buffer_position()
          );
          parent_stack.last_mut().unwrap().children.push(doctype_node);
        }
      }
      Ok(Event::PI(e)) => {
        let content = decode_bytes(e.as_ref(), &reader, arena)?;
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
      Ok(Event::Decl(e)) => {
        let value = decode_bytes(e.as_ref(), &reader, arena)?;
        let decl_node = XMLAstChild::Decl(XMLAstDecl { value });
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(decl_node);
        } else {
          root.children.push(decl_node);
        }
      }
      Ok(Event::Eof) => break,
      Err(e) => return Err(Box::new(e)),
    }

    buf.clear();
  }

  Ok(root)
}
