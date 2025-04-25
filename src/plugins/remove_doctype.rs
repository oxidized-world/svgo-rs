use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstDoctype;

/// Remove DOCTYPE declaration.
pub struct RemoveDoctypePlugin<'a> {
  pub arena: &'a Bump,
}

impl<'a> Plugin<'a> for RemoveDoctypePlugin<'a> {
  fn doctype_enter(&self, _el: &mut XMLAstDoctype<'a>) -> VisitAction {
    VisitAction::Remove
  }
}
