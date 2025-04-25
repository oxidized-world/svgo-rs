use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};

/// Remove <title>.
pub struct RemoveTitlePlugin<'a> {
  pub arena: &'a Bump,
}

impl<'a> Plugin<'a> for RemoveTitlePlugin<'a> {
  fn element_enter(&self, el: &mut crate::parser::XMLAstElement<'a>) -> VisitAction {
    if el.name == "title" {
      VisitAction::Remove
    } else {
      VisitAction::Keep
    }
  }
}
