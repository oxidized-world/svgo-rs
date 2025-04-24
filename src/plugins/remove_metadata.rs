use crate::optimizer::{Plugin, VisitAction};

/// Remove <metadata>.
pub struct RemoveMetadataPlugin {}

impl<'a> Plugin<'a> for RemoveMetadataPlugin {
  fn element_enter(&self, el: &mut crate::parser::XMLAstElement<'a>) -> VisitAction {
    if el.name == "metadata" {
      VisitAction::Remove
    } else {
      VisitAction::Keep
    }
  }
}
