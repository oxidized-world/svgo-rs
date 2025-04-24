use crate::optimizer::{Plugin, VisitAction};
use bumpalo::Bump;
use regex::Regex;

/// Remove comments.
pub struct RemoveCommentsPlugin<'a> {
  pub preserve_patterns: Vec<Regex>,
  pub arena: &'a Bump,
}

impl<'a> Plugin<'a> for RemoveCommentsPlugin<'a> {
  fn comment_enter(&self, _el: &mut crate::parser::XMLAstComment<'a>) -> VisitAction {
    // Iterate through the patterns to preserve
    for pattern in &self.preserve_patterns {
      // Check if the comment text matches the current pattern
      // Assuming el.value contains the comment text
      if pattern.is_match(&_el.value) {
        // If it matches, keep the comment and stop checking
        return VisitAction::Keep;
      }
    }
    // If no pattern matched, remove the comment
    VisitAction::Remove
  }
}
