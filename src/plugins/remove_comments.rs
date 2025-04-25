use crate::optimizer::{Plugin, VisitAction};
use bumpalo::Bump;
use regex::Regex;

/// Remove comments.
#[allow(dead_code)]
pub struct RemoveCommentsPlugin<'a> {
  preserve_patterns: Vec<Regex>,
  arena: &'a Bump,
}

/// Configuration for RemoveCommentsPlugin.
pub struct RemoveCommentsConfig {
  pub preserve_patterns: Option<Vec<Regex>>,
}

impl<'a> RemoveCommentsPlugin<'a> {
  pub fn new(config: RemoveCommentsConfig, arena: &'a Bump) -> Self {
    RemoveCommentsPlugin {
      arena: arena,
      preserve_patterns: config
        .preserve_patterns
        .unwrap_or_else(|| vec![Regex::new(r"^!").unwrap()]),
    }
  }
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
