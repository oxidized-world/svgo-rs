use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::{XMLAstChild, XMLAstElement};

/// Removes `<desc>`.
///
/// 仅删除空的或以标准编辑器生成内容开头的 `<desc>`，以保留可访问性描述。
/// 设置 `remove_any = true` 则删除所有 `<desc>`.
#[allow(dead_code)]
pub struct RemoveDescPlugin<'a> {
  pub remove_any: bool,
  pub arena: &'a Bump,
}

pub struct RemoveDescPluginConfig {
  pub remove_any: bool,
}

impl<'a> RemoveDescPlugin<'a> {
  pub fn new(config: RemoveDescPluginConfig, arena: &'a Bump) -> Self {
    RemoveDescPlugin {
      arena: arena,
      remove_any: config.remove_any,
    }
  }
}

impl<'a> Plugin<'a> for RemoveDescPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
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
