use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::{XMLAstChild, XMLAstElement};

/// Removes `<desc>`.
///
/// 仅删除空的或以标准编辑器生成内容开头的 `<desc>`，以保留可访问性描述。
/// 设置 `remove_any = true` 则删除所有 `<desc>`.
pub struct RemoveDescPlugin<'a> {
  pub remove_any: bool,
  pub arena: &'a Bump,
}

impl<'a> Plugin<'a> for RemoveDescPlugin<'a> {
  fn element_enter(&self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "desc" {
      if self.remove_any {
        return VisitAction::Remove;
      }
      match el.children.get(0) {
        None => VisitAction::Remove,
        Some(XMLAstChild::Text(t)) if is_standard_desc(&t.value) => VisitAction::Remove,
        _ => VisitAction::Keep,
      }
    } else {
      VisitAction::Keep
    }
  }
}

fn is_standard_desc(value: &str) -> bool {
  value.starts_with("Created with") || value.starts_with("Created using")
}
