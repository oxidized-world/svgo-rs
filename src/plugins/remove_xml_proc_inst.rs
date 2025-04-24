use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstInstruction;

/// Remove XML Processing Instruction.
pub struct RemoveXMLProcInstPlugin {}

impl<'a> Plugin<'a> for RemoveXMLProcInstPlugin {
  fn instruction_enter(&self, el: &mut XMLAstInstruction<'a>) -> VisitAction {
    if el.name == "xml" {
      VisitAction::Remove
    } else {
      VisitAction::Keep
    }
  }
}
