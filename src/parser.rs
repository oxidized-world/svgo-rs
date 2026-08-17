use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use quick_xml::escape::{resolve_predefined_entity, unescape};
use quick_xml::events::attributes::Attributes;
use quick_xml::events::BytesText;
use quick_xml::events::Event;
use quick_xml::Reader;
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

fn parse_attributes<'a>(
  attributes: Attributes<'_>,
  reader: &Reader<&[u8]>,
  arena: &'a Bump,
) -> Result<BumpVec<'a, (&'a str, &'a str)>, Box<dyn Error>> {
  let mut attrs_vec = BumpVec::new_in(arena);
  for attr_result in attributes {
    let attr = attr_result?;
    let cow = reader.decoder().decode(attr.key.as_ref())?;
    let key: &str = arena.alloc_str(&cow);
    // 这里不用 `Attribute::unescape_value()`：quick-xml 0.40 起它已被废弃，
    // 且行为改成了按 XML 规范做属性值归一化，会把值里的 \t \r \n 替换成空格。
    // 手动 decode + unescape 才能让属性值保持源文档里的原始样子。
    let decoded = reader.decoder().decode(&attr.value)?;
    let raw_val = unescape(&decoded)?;
    let value = arena.alloc_str(&raw_val);
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

/// 解码文本 / 注释内容并在 arena 中分配
///
/// quick-xml 0.38 起 `BytesText::unescape()` 已被移除：文本里的实体引用不再
/// 内联在 `Event::Text` 里，而是作为独立的 `Event::GeneralRef` 事件返回，
/// 因此这里只需要解码，反转义由 `Event::GeneralRef` 分支负责。
fn decode_text<'arena>(
  bytes_text: &BytesText,
  arena: &'arena Bump,
) -> Result<&'arena str, Box<dyn Error>> {
  let cow = bytes_text.decode()?;
  Ok(arena.alloc_str(&cow))
}

/// 把一段文本追加到当前父元素（栈顶）或根节点的子节点列表
///
/// 如果最后一个子节点已经是文本节点，则就地合并。这样一来，被
/// `Event::GeneralRef` 切分成多个事件的同一段文本，最终仍然只对应一个
/// 文本节点，AST 形状与 quick-xml 0.38 之前保持一致。
fn push_text<'arena>(
  parent_stack: &mut [XMLAstElement<'arena>],
  root: &mut XMLAstRoot<'arena>,
  value: &'arena str,
  arena: &'arena Bump,
) {
  let children = match parent_stack.last_mut() {
    Some(parent) => &mut parent.children,
    None => &mut root.children,
  };
  if let Some(XMLAstChild::Text(last)) = children.last_mut() {
    let merged = bumpalo::format!(in arena, "{}{}", last.value, value).into_bump_str();
    last.value = merged;
    return;
  }
  children.push(XMLAstChild::Text(XMLAstText { value }));
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
  // 使用栈来追踪父元素，Vec<XMLAstElement> 用于存储正在构建中的元素
  let mut parent_stack: Vec<XMLAstElement<'arena>> = Vec::new();
  // 缓冲区，用于 read_event_into
  let mut buf = Vec::new();

  loop {
    match reader.read_event_into(&mut buf) {
      Ok(Event::Start(e)) => {
        let name = decode_bytes(e.name().as_ref(), &reader, arena)?;
        let attributes = parse_attributes(e.attributes(), &reader, arena)?;
        let element = XMLAstElement {
          name,
          attributes,
          children: BumpVec::new_in(arena),
        };
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
        let attributes = parse_attributes(e.attributes(), &reader, arena)?;
        let element = XMLAstElement {
          name,
          attributes,
          children: BumpVec::new_in(arena),
        };
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
        let value = decode_text(&e, arena)?;
        push_text(&mut parent_stack, &mut root, value, arena);
      }
      // --- 实体 / 字符引用 &amp; &#65; ---
      // quick-xml 0.38 起，文本中的引用不再包含在 Text 事件里，而是单独作为
      // GeneralRef 事件返回，需要自己解析后拼回文本节点。
      Ok(Event::GeneralRef(e)) => {
        let raw = decode_bytes(e.as_ref(), &reader, arena)?;
        let resolved: &str = match e.resolve_char_ref()? {
          // 字符引用：&#65; / &#x41;
          Some(ch) => {
            let mut buf = [0u8; 4];
            arena.alloc_str(ch.encode_utf8(&mut buf))
          }
          None => match resolve_predefined_entity(raw) {
            // 预定义实体：&amp; &lt; &gt; &quot; &apos;
            Some(text) => text,
            // 未知实体（例如自定义 DTD 实体）原样保留。
            // 旧版本这里会直接报错，并在 lib.rs 的 unwrap 处 panic。
            None => bumpalo::format!(in arena, "&{};", raw).into_bump_str(),
          },
        };
        push_text(&mut parent_stack, &mut root, resolved, arena);
      }
      // --- 注释 ---
      Ok(Event::Comment(e)) => {
        // 注释内部不做实体展开，所以这里只解码、不反转义
        let value = decode_text(&e, arena)?;
        let comment_node = XMLAstChild::Comment(XMLAstComment { value });
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(comment_node);
        } else {
          root.children.push(comment_node);
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
