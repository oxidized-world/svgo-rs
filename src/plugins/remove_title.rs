use crate::optimizer::{Plugin, VisitAction};

/// Remove <title>.
pub struct RemoveTitlePlugin {}

impl<'a> Plugin<'a> for RemoveTitlePlugin {
  fn element_enter(&self, el: &mut crate::parser::XMLAstElement<'a>) -> VisitAction {
    if el.name == "title" {
      VisitAction::Remove
    } else {
      VisitAction::Keep
    }
  }
}
