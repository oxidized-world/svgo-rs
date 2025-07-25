use crate::parser::{
  XMLAstCdata, XMLAstChild, XMLAstComment, XMLAstDecl, XMLAstDoctype, XMLAstElement,
  XMLAstInstruction, XMLAstRoot, XMLAstText,
};
use bumpalo::collections::Vec as BumpVec;

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
  plugins: Vec<Box<dyn Plugin<'a> + 'a>>,
}

impl<'a> SvgOptimizer<'a> {
  pub fn new(plugins: Vec<Box<dyn Plugin<'a> + 'a>>) -> Self {
    Self { plugins }
  }

  pub fn optimize(&mut self, root: &mut XMLAstRoot<'a>) -> String {
    for plugin in &self.plugins {
      plugin.root_enter(root);
    }
    // 对根节点的 children 启动遍历
    self.traverse_children(&mut root.children);
    for plugin in &self.plugins {
      plugin.root_exit(root);
    }
    self.generate_svg(root)
  }

  fn traverse_children(&mut self, children: &mut BumpVec<'a, XMLAstChild<'a>>) {
    let mut i = 0;
    while i < children.len() {
      // Check if any plugin wants to remove this node via the enter hook
      let should_remove = match &mut children[i] {
        XMLAstChild::Doctype(el) => self
          .plugins
          .iter()
          .any(|plugin| plugin.doctype_enter(el) == VisitAction::Remove),
        XMLAstChild::Instruction(el) => self
          .plugins
          .iter()
          .any(|plugin| plugin.instruction_enter(el) == VisitAction::Remove),
        XMLAstChild::Comment(el) => self
          .plugins
          .iter()
          .any(|plugin| plugin.comment_enter(el) == VisitAction::Remove),
        XMLAstChild::Cdata(el) => {
          self.plugins.iter().any(|plugin| plugin.cdata_enter(el) == VisitAction::Remove)
        }
        XMLAstChild::Text(el) => {
          self.plugins.iter().any(|plugin| plugin.text_enter(el) == VisitAction::Remove)
        }
        XMLAstChild::Element(el) => self
          .plugins
          .iter_mut()
          .any(|plugin| plugin.element_enter(el) == VisitAction::Remove),
        // Assuming Decl nodes are never removed by plugins
        XMLAstChild::Decl(el) => {
          self.plugins.iter().any(|plugin| plugin.decl_enter(el) == VisitAction::Remove)
        }
      };

      if should_remove {
        children.remove(i);
        // Do not increment i, the next element shifts to the current index
        continue;
      }

      // If not removed, traverse deeper (for elements) and call exit hooks
      match &mut children[i] {
        XMLAstChild::Element(el) => {
          // Traverse children before calling exit hooks for the parent
          self.traverse_children(&mut el.children);
          for plugin in &self.plugins {
            plugin.element_exit(el);
          }
        }
        XMLAstChild::Text(t) => {
          // Call text_exit (enter hook was checked above)
          for plugin in &self.plugins {
            plugin.text_exit(t);
          }
        }
        XMLAstChild::Comment(c) => {
          // Call comment_exit
          for plugin in &self.plugins {
            plugin.comment_exit(c);
          }
        }
        XMLAstChild::Doctype(d) => {
          // Call doctype_exit
          for plugin in &self.plugins {
            plugin.doctype_exit(d);
          }
        }
        XMLAstChild::Instruction(ins) => {
          // Call instruction_exit
          for plugin in &self.plugins {
            plugin.instruction_exit(ins);
          }
        }
        XMLAstChild::Cdata(cd) => {
          // Call cdata_exit
          for plugin in &self.plugins {
            plugin.cdata_exit(cd);
          }
        }
        XMLAstChild::Decl(decl) => {
          for plugin in &self.plugins {
            plugin.decl_exit(decl);
          }
        }
      }
      // Increment index only if the element was not removed
      i += 1;
    }
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
    use std::fmt::Write;
    match child {
      XMLAstChild::Element(el) => {
        // 开始标签
        write!(buf, "<{}", el.name).unwrap();
        // 输出属性
        for (k, v) in &el.attributes {
          write!(buf, " {}=\"{}\"", k, v).unwrap();
        }
        if el.children.is_empty() {
          // 自闭合标签
          buf.push_str("/>");
        } else {
          buf.push('>');
          // 递归子节点
          self.write_children(&el.children, buf);
          // 结束标签
          write!(buf, "</{}>", el.name).unwrap();
        }
      }
      XMLAstChild::Text(t) => {
        // 文本节点
        buf.push_str(&t.value);
      }
      XMLAstChild::Comment(c) => {
        // 注释
        write!(buf, "<!--{}-->", c.value).unwrap();
      }
      _ => {}
    }
  }
}
