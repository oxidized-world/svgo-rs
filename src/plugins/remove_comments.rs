use crate::optimizer::{Plugin, VisitAction};
use regex::Regex;

/// Remove comments.
pub struct RemoveCommentsPlugin {
  pub preserve_patterns: Vec<Regex>,
}

impl<'a> Plugin<'a> for RemoveCommentsPlugin {
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
