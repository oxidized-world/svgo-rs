use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::BytesText;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::error::Error;

/// <!DOCTYPE ...>
#[derive(Debug, Clone)]
pub struct XMLAstDoctype<'arena> {
  pub name: &'arena str,
  pub data: XMLAstDoctypeData<'arena>,
}

#[derive(Debug, Clone)]
pub struct XMLAstDoctypeData<'arena> {
  pub doctype: &'arena str,
}

/// <?instruction ...?>
#[derive(Debug, Clone)]
pub struct XMLAstInstruction<'arena> {
  pub name: &'arena str,
  pub value: &'arena str,
}

/// <!-- comment -->
#[derive(Debug, Clone)]
pub struct XMLAstComment<'arena> {
  pub value: &'arena str,
}

/// <![CDATA[ ... ]]>
#[derive(Debug, Clone)]
pub struct XMLAstCdata<'arena> {
  pub value: &'arena str,
}

/// <?xml ... ?>
#[derive(Debug, Clone)]
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

/// 从 quick_xml 的 Attributes 迭代器解析属性到 HashMap
fn parse_attributes<'a>(
  attributes: Attributes<'_>,
  reader: &Reader<&[u8]>, // 需要 reader 来访问解码器
  arena: &'a Bump,        // 添加 arena 参数
) -> Result<BumpVec<'a, (&'a str, &'a str)>, Box<dyn Error>> {
  let mut attrs_vec = BumpVec::new_in(arena);
  for attr_result in attributes {
    let attr = attr_result?;
    let cow = reader.decoder().decode(attr.key.as_ref())?;
    let key: &str = arena.alloc_str(&cow);
    let raw_val = attr.unescape_value()?;
    let value = arena.alloc_str(&raw_val);
    // 直接 push 元组
    attrs_vec.push((key, &*value));
  }
  Ok(attrs_vec)
}

/// 解码 quick_xml 的字节切片并在 arena 中分配
fn decode_bytes<'arena>(
  bytes: &[u8],
  reader: &Reader<&[u8]>,
  arena: &'arena Bump, // 添加 arena 参数
) -> Result<&'arena str, Box<dyn Error>> {
  let cow = reader.decoder().decode(bytes)?;
  Ok(arena.alloc_str(&cow)) // 在 arena 中分配
}

/// 解码包含转义字符的文本内容并在 arena 中分配
fn decode_escaped<'arena>(
  bytes_text: &BytesText,
  _reader: &Reader<&[u8]>, // reader 可能不再需要直接用于此函数内部解码
  arena: &'arena Bump,     // 添加 arena 参数
) -> Result<&'arena str, Box<dyn Error>> {
  let cow = bytes_text.unescape()?; // 处理 XML 实体
  Ok(arena.alloc_str(&cow))
}

pub fn parse_svg<'arena>(
  svg_string: &'arena str,
  arena: &'arena Bump, // 传入 arena 引用
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
        // 从栈顶弹出一个完成的元素
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
        let value = decode_escaped(&e, &reader, arena)?;
        let text_node = XMLAstChild::Text(XMLAstText { value });
        // 添加到当前父元素（栈顶）或根节点（如果文本在顶层，虽然不常见于格式良好的XML）
        if let Some(parent) = parent_stack.last_mut() {
          parent.children.push(text_node);
        } else {
          root.children.push(text_node);
        }
      }
      // --- 注释 ---
      Ok(Event::Comment(e)) => {
        let value = decode_escaped(&e, &reader, arena)?;
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
          // 在元素内部发现 Doctype? 这通常是无效的 XML，但我们还是处理一下
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
        let name = parts.next().unwrap_or(""); // &'arena str
        let value = parts.next().unwrap_or("").trim_start(); // &'arena str

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
      Ok(Event::Eof) => break,           // 成功解析到文件末尾
      Err(e) => return Err(Box::new(e)), // 将 quick-xml 的错误传递出去
    }

    // 清空缓冲区为下一次读取事件做准备
    buf.clear();
  }

  Ok(root)
}
