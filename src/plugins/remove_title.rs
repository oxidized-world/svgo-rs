use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};

/// Remove <title>.
#[allow(dead_code)]
pub struct RemoveTitlePlugin<'a> {
  pub arena: &'a Bump,
}

pub struct RemoveTitlePluginConfig {}

impl<'a> RemoveTitlePlugin<'a> {
  pub fn new(_config: RemoveTitlePluginConfig, arena: &'a Bump) -> Self {
    RemoveTitlePlugin { arena: arena }
  }
}

impl<'a> Plugin<'a> for RemoveTitlePlugin<'a> {
  fn element_enter(&mut self, el: &mut crate::parser::XMLAstElement<'a>) -> VisitAction {
    if el.name == "title" {
      VisitAction::Remove
    } else {
      VisitAction::Keep
    }
  }
}
