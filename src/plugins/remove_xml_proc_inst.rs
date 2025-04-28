use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};

/// Remove XML Processing Instruction.
#[allow(dead_code)]
pub struct RemoveXMLProcInstPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct RemoveXMLProcInstPluginConfig {}

impl<'a> RemoveXMLProcInstPlugin<'a> {
  pub fn new(_config: RemoveXMLProcInstPluginConfig, arena: &'a Bump) -> Self {
    RemoveXMLProcInstPlugin { arena: arena }
  }
}

impl<'a> Plugin<'a> for RemoveXMLProcInstPlugin<'a> {
  fn decl_enter(&self, _el: &mut crate::parser::XMLAstDecl<'a>) -> VisitAction {
    VisitAction::Remove
  }
}
