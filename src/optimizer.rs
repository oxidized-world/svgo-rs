use crate::parser::{
  XMLAstCdata, XMLAstChild, XMLAstComment, XMLAstDecl, XMLAstDoctype, XMLAstElement,
  XMLAstInstruction, XMLAstRoot, XMLAstText,
};
use bumpalo::collections::Vec as BumpVec;

pub type HookMask = u16;

pub const HOOK_ROOT_ENTER: HookMask = 1 << 0;
pub const HOOK_ROOT_EXIT: HookMask = 1 << 1;
pub const HOOK_ELEMENT_ENTER: HookMask = 1 << 2;
pub const HOOK_ELEMENT_EXIT: HookMask = 1 << 3;
pub const HOOK_TEXT_ENTER: HookMask = 1 << 4;
pub const HOOK_TEXT_EXIT: HookMask = 1 << 5;
pub const HOOK_COMMENT_ENTER: HookMask = 1 << 6;
pub const HOOK_COMMENT_EXIT: HookMask = 1 << 7;
pub const HOOK_DOCTYPE_ENTER: HookMask = 1 << 8;
pub const HOOK_DOCTYPE_EXIT: HookMask = 1 << 9;
pub const HOOK_INSTRUCTION_ENTER: HookMask = 1 << 10;
pub const HOOK_INSTRUCTION_EXIT: HookMask = 1 << 11;
pub const HOOK_CDATA_ENTER: HookMask = 1 << 12;
pub const HOOK_CDATA_EXIT: HookMask = 1 << 13;
pub const HOOK_DECL_ENTER: HookMask = 1 << 14;
pub const HOOK_DECL_EXIT: HookMask = 1 << 15;
pub const HOOK_ALL: HookMask = u16::MAX;

#[derive(PartialEq, Eq)]
pub enum VisitAction {
  /// 保留该元素
  Keep,
  /// 移除该元素
  Remove,
}

pub trait Plugin<'a> {
  fn root_enter(&self, _el: &mut XMLAstRoot<'a>) {}
  fn root_exit(&self, _el: &mut XMLAstRoot<'a>) {}

  /// return true 表示要把这个 element 从父节点里删掉
  fn element_enter(&mut self, _el: &mut XMLAstElement<'a>) -> VisitAction {
    VisitAction::Keep
  }
  fn element_exit(&self, _el: &mut XMLAstElement<'a>) {}

  fn text_enter(&self, _el: &mut XMLAstText<'a>) -> VisitAction {
    VisitAction::Keep
  }
  fn text_exit(&self, _el: &mut XMLAstText<'a>) {}

  fn comment_enter(&self, _el: &mut XMLAstComment<'a>) -> VisitAction {
    VisitAction::Keep
  }
  fn comment_exit(&self, _el: &mut XMLAstComment<'a>) {}

  fn doctype_enter(&self, _el: &mut XMLAstDoctype<'a>) -> VisitAction {
    VisitAction::Keep
  }
  fn doctype_exit(&self, _el: &mut XMLAstDoctype<'a>) {}

  fn instruction_enter(&self, _el: &mut XMLAstInstruction<'a>) -> VisitAction {
    VisitAction::Keep
  }
  fn instruction_exit(&self, _el: &mut XMLAstInstruction<'a>) {}

  fn cdata_enter(&self, _el: &mut XMLAstCdata<'a>) -> VisitAction {
    VisitAction::Keep
  }
  fn cdata_exit(&self, _el: &mut XMLAstCdata<'a>) {}

  fn decl_enter(&self, _el: &mut XMLAstDecl<'a>) -> VisitAction {
    VisitAction::Keep
  }
  fn decl_exit(&self, _el: &mut XMLAstDecl<'a>) {}
}

pub struct SvgOptimizer<'a> {
  plugins: Vec<PluginEntry<'a>>,
  root_enter_plugins: Vec<usize>,
  root_exit_plugins: Vec<usize>,
  element_enter_plugins: Vec<usize>,
  element_exit_plugins: Vec<usize>,
  text_enter_plugins: Vec<usize>,
  text_exit_plugins: Vec<usize>,
  comment_enter_plugins: Vec<usize>,
  comment_exit_plugins: Vec<usize>,
  doctype_enter_plugins: Vec<usize>,
  doctype_exit_plugins: Vec<usize>,
  instruction_enter_plugins: Vec<usize>,
  instruction_exit_plugins: Vec<usize>,
  cdata_enter_plugins: Vec<usize>,
  cdata_exit_plugins: Vec<usize>,
  decl_enter_plugins: Vec<usize>,
  decl_exit_plugins: Vec<usize>,
}

pub struct PluginEntry<'a> {
  pub plugin: Box<dyn Plugin<'a> + 'a>,
  pub hooks: HookMask,
}

impl<'a> PluginEntry<'a> {
  pub fn new(plugin: Box<dyn Plugin<'a> + 'a>) -> Self {
    Self {
      plugin,
      hooks: HOOK_ALL,
    }
  }

  pub fn with_hooks(plugin: Box<dyn Plugin<'a> + 'a>, hooks: HookMask) -> Self {
    Self { plugin, hooks }
  }

  #[inline]
  fn enabled(&self, hook: HookMask) -> bool {
    (self.hooks & hook) != 0
  }
}

fn write_escaped_text(buf: &mut String, input: &str) {
  if !input.as_bytes().iter().any(|b| matches!(b, b'&' | b'<' | b'>')) {
    buf.push_str(input);
    return;
  }

  for ch in input.chars() {
    match ch {
      '&' => buf.push_str("&amp;"),
      '<' => buf.push_str("&lt;"),
      '>' => buf.push_str("&gt;"),
      _ => buf.push(ch),
    }
  }
}

fn write_escaped_attr(buf: &mut String, input: &str) {
  if !input.as_bytes().iter().any(|b| matches!(b, b'&' | b'<' | b'>' | b'"')) {
    buf.push_str(input);
    return;
  }

  for ch in input.chars() {
    match ch {
      '&' => buf.push_str("&amp;"),
      '<' => buf.push_str("&lt;"),
      '>' => buf.push_str("&gt;"),
      '"' => buf.push_str("&quot;"),
      _ => buf.push(ch),
    }
  }
}

impl<'a> SvgOptimizer<'a> {
  pub fn new(plugins: Vec<Box<dyn Plugin<'a> + 'a>>) -> Self {
    Self::new_with_entries(plugins.into_iter().map(PluginEntry::new).collect())
  }

  pub fn new_with_entries(plugins: Vec<PluginEntry<'a>>) -> Self {
    let mut root_enter_plugins = Vec::new();
    let mut root_exit_plugins = Vec::new();
    let mut element_enter_plugins = Vec::new();
    let mut element_exit_plugins = Vec::new();
    let mut text_enter_plugins = Vec::new();
    let mut text_exit_plugins = Vec::new();
    let mut comment_enter_plugins = Vec::new();
    let mut comment_exit_plugins = Vec::new();
    let mut doctype_enter_plugins = Vec::new();
    let mut doctype_exit_plugins = Vec::new();
    let mut instruction_enter_plugins = Vec::new();
    let mut instruction_exit_plugins = Vec::new();
    let mut cdata_enter_plugins = Vec::new();
    let mut cdata_exit_plugins = Vec::new();
    let mut decl_enter_plugins = Vec::new();
    let mut decl_exit_plugins = Vec::new();

    for (index, entry) in plugins.iter().enumerate() {
      if entry.enabled(HOOK_ROOT_ENTER) {
        root_enter_plugins.push(index);
      }
      if entry.enabled(HOOK_ROOT_EXIT) {
        root_exit_plugins.push(index);
      }
      if entry.enabled(HOOK_ELEMENT_ENTER) {
        element_enter_plugins.push(index);
      }
      if entry.enabled(HOOK_ELEMENT_EXIT) {
        element_exit_plugins.push(index);
      }
      if entry.enabled(HOOK_TEXT_ENTER) {
        text_enter_plugins.push(index);
      }
      if entry.enabled(HOOK_TEXT_EXIT) {
        text_exit_plugins.push(index);
      }
      if entry.enabled(HOOK_COMMENT_ENTER) {
        comment_enter_plugins.push(index);
      }
      if entry.enabled(HOOK_COMMENT_EXIT) {
        comment_exit_plugins.push(index);
      }
      if entry.enabled(HOOK_DOCTYPE_ENTER) {
        doctype_enter_plugins.push(index);
      }
      if entry.enabled(HOOK_DOCTYPE_EXIT) {
        doctype_exit_plugins.push(index);
      }
      if entry.enabled(HOOK_INSTRUCTION_ENTER) {
        instruction_enter_plugins.push(index);
      }
      if entry.enabled(HOOK_INSTRUCTION_EXIT) {
        instruction_exit_plugins.push(index);
      }
      if entry.enabled(HOOK_CDATA_ENTER) {
        cdata_enter_plugins.push(index);
      }
      if entry.enabled(HOOK_CDATA_EXIT) {
        cdata_exit_plugins.push(index);
      }
      if entry.enabled(HOOK_DECL_ENTER) {
        decl_enter_plugins.push(index);
      }
      if entry.enabled(HOOK_DECL_EXIT) {
        decl_exit_plugins.push(index);
      }
    }

    Self {
      plugins,
      root_enter_plugins,
      root_exit_plugins,
      element_enter_plugins,
      element_exit_plugins,
      text_enter_plugins,
      text_exit_plugins,
      comment_enter_plugins,
      comment_exit_plugins,
      doctype_enter_plugins,
      doctype_exit_plugins,
      instruction_enter_plugins,
      instruction_exit_plugins,
      cdata_enter_plugins,
      cdata_exit_plugins,
      decl_enter_plugins,
      decl_exit_plugins,
    }
  }

  pub fn optimize(&mut self, root: &mut XMLAstRoot<'a>) -> String {
    for &index in &self.root_enter_plugins {
      self.plugins[index].plugin.root_enter(root);
    }
    // 对根节点的 children 启动遍历
    self.traverse_children(&mut root.children);
    for &index in &self.root_exit_plugins {
      self.plugins[index].plugin.root_exit(root);
    }
    self.generate_svg(root)
  }

  fn traverse_children(&mut self, children: &mut BumpVec<'a, XMLAstChild<'a>>) {
    let len = children.len();
    let mut write_index = 0;

    for read_index in 0..len {
      // Check if any plugin wants to remove this node via the enter hook
      let should_remove = {
        let child = &mut children[read_index];
        match child {
          XMLAstChild::Doctype(el) => self
            .doctype_enter_plugins
            .iter()
            .any(|&index| self.plugins[index].plugin.doctype_enter(el) == VisitAction::Remove),
          XMLAstChild::Instruction(el) => self
            .instruction_enter_plugins
            .iter()
            .any(|&index| self.plugins[index].plugin.instruction_enter(el) == VisitAction::Remove),
          XMLAstChild::Comment(el) => self
            .comment_enter_plugins
            .iter()
            .any(|&index| self.plugins[index].plugin.comment_enter(el) == VisitAction::Remove),
          XMLAstChild::Cdata(el) => self
            .cdata_enter_plugins
            .iter()
            .any(|&index| self.plugins[index].plugin.cdata_enter(el) == VisitAction::Remove),
          XMLAstChild::Text(el) => self
            .text_enter_plugins
            .iter()
            .any(|&index| self.plugins[index].plugin.text_enter(el) == VisitAction::Remove),
          XMLAstChild::Element(el) => {
            let mut should_remove = false;
            for &index in &self.element_enter_plugins {
              if self.plugins[index].plugin.element_enter(el) == VisitAction::Remove {
                should_remove = true;
                break;
              }
            }
            should_remove
          }
          XMLAstChild::Decl(el) => self
            .decl_enter_plugins
            .iter()
            .any(|&index| self.plugins[index].plugin.decl_enter(el) == VisitAction::Remove),
        }
      };

      if should_remove {
        continue;
      }

      // If not removed, traverse deeper (for elements) and call exit hooks
      {
        let child = &mut children[read_index];
        match child {
          XMLAstChild::Element(el) => {
            self.traverse_children(&mut el.children);
            for &index in &self.element_exit_plugins {
              self.plugins[index].plugin.element_exit(el);
            }
          }
          XMLAstChild::Text(t) => {
            for &index in &self.text_exit_plugins {
              self.plugins[index].plugin.text_exit(t);
            }
          }
          XMLAstChild::Comment(c) => {
            for &index in &self.comment_exit_plugins {
              self.plugins[index].plugin.comment_exit(c);
            }
          }
          XMLAstChild::Doctype(d) => {
            for &index in &self.doctype_exit_plugins {
              self.plugins[index].plugin.doctype_exit(d);
            }
          }
          XMLAstChild::Instruction(ins) => {
            for &index in &self.instruction_exit_plugins {
              self.plugins[index].plugin.instruction_exit(ins);
            }
          }
          XMLAstChild::Cdata(cd) => {
            for &index in &self.cdata_exit_plugins {
              self.plugins[index].plugin.cdata_exit(cd);
            }
          }
          XMLAstChild::Decl(decl) => {
            for &index in &self.decl_exit_plugins {
              self.plugins[index].plugin.decl_exit(decl);
            }
          }
        }
      }

      if write_index != read_index {
        children.swap(write_index, read_index);
      }
      write_index += 1;
    }

    children.truncate(write_index);
  }

  /// 根据 AST 生成 SVG 字符串
  pub fn generate_svg(&self, root: &XMLAstRoot<'a>) -> String {
    let mut output = String::new();
    self.write_children(&root.children, &mut output);
    output
  }

  fn write_children(&self, children: &BumpVec<'a, XMLAstChild<'a>>, buf: &mut String) {
    for child in children {
      self.write_child(child, buf);
    }
  }

  fn write_child(&self, child: &XMLAstChild<'a>, buf: &mut String) {
    match child {
      XMLAstChild::Element(el) => {
        // 开始标签
        buf.push('<');
        buf.push_str(el.name);
        // 输出属性
        for (k, v) in &el.attributes {
          buf.push(' ');
          buf.push_str(k);
          buf.push_str("=\"");
          write_escaped_attr(buf, v);
          buf.push('"');
        }
        if el.children.is_empty() {
          // 自闭合标签
          buf.push_str("/>");
        } else {
          buf.push('>');
          // 递归子节点
          self.write_children(&el.children, buf);
          // 结束标签
          buf.push_str("</");
          buf.push_str(el.name);
          buf.push('>');
        }
      }
      XMLAstChild::Text(t) => {
        // 文本节点
        write_escaped_text(buf, t.value);
      }
      XMLAstChild::Comment(c) => {
        // 注释
        buf.push_str("<!--");
        buf.push_str(c.value);
        buf.push_str("-->");
      }
      XMLAstChild::Doctype(d) => {
        // <!DOCTYPE ...>
        buf.push_str("<!DOCTYPE ");
        buf.push_str(d.data.doctype);
        buf.push('>');
      }
      XMLAstChild::Instruction(i) => {
        // <?name value?>
        buf.push_str("<?");
        buf.push_str(i.name);
        if i.value.is_empty() {
          buf.push_str("?>");
        } else {
          buf.push(' ');
          buf.push_str(i.value);
          buf.push_str("?>");
        }
      }
      XMLAstChild::Cdata(cd) => {
        // <![CDATA[...]]>
        buf.push_str("<![CDATA[");
        buf.push_str(cd.value);
        buf.push_str("]]>");
      }
      XMLAstChild::Decl(decl) => {
        // <?xml ...?>
        buf.push_str("<?");
        buf.push_str(decl.value);
        buf.push_str("?>");
      }
    }
  }
}
