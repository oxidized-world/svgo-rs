use crate::optimizer::{Plugin, VisitAction};

/// Remove comments.
pub struct RemoveCommentsPlugin {}

impl<'a> Plugin<'a> for RemoveCommentsPlugin {
  fn comment_enter(&self, _el: &mut crate::parser::XMLAstComment<'a>) -> VisitAction {
    VisitAction::Keep
  }
}
