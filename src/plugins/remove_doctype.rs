use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstDoctype;

/// Remove DOCTYPE declaration.
pub struct RemoveDoctypePlugin {}

impl<'a> Plugin<'a> for RemoveDoctypePlugin {
  fn doctype_enter(&self, _el: &mut XMLAstDoctype<'a>) -> VisitAction {
    VisitAction::Remove
  }
}
