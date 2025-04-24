use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstInstruction;

/// Remove XML Processing Instruction.
pub struct RemoveXMLProcInstPlugin<'a> {
  pub arena: &'a Bump,
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
