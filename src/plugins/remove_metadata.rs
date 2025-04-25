use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};

/// Remove <metadata>.
pub struct RemoveMetadataPlugin<'a> {
  pub arena: &'a Bump,
}

impl<'a> Plugin<'a> for RemoveMetadataPlugin<'a> {
  fn element_enter(&self, el: &mut crate::parser::XMLAstElement<'a>) -> VisitAction {
    if el.name == "metadata" {
      VisitAction::Remove
    } else {
      VisitAction::Keep
    }
  }
}
