use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstInstruction;

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
  fn instruction_enter(&self, el: &mut XMLAstInstruction<'a>) -> VisitAction {
    if el.name == "xml" {
      VisitAction::Remove
    } else {
      VisitAction::Keep
    }
  }
}
