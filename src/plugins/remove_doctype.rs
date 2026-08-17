use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstDoctype;

/// Remove DOCTYPE declaration.
#[allow(dead_code)]
pub struct RemoveDoctypePlugin<'a> {
  pub arena: &'a Bump,
}

pub struct RemoveDoctypePluginConfig {}

impl<'a> RemoveDoctypePlugin<'a> {
  pub fn new(_config: RemoveDoctypePluginConfig, arena: &'a Bump) -> Self {
    RemoveDoctypePlugin { arena }
  }
}

impl<'a> Plugin<'a> for RemoveDoctypePlugin<'a> {
  fn doctype_enter(&self, _el: &mut XMLAstDoctype<'a>) -> VisitAction {
    VisitAction::Remove
  }
}
