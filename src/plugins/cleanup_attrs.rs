use bumpalo::collections::String as BumpString; // For allocating new strings into the arena
use bumpalo::Bump;
use derive_builder::Builder;
use regex::Regex;
use std::borrow::Cow; // Useful for avoiding allocations if string doesn't change

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement; // Assuming this is the correct path to your AST element

/// Configuration for the cleanupAttrs plugin.
#[derive(Debug, Builder, Default)]
pub struct CleanupAttrsConfig {
  /// Process newlines. (default: true)
  /// - Replaces newlines surrounded by non-whitespace with a single space.
  /// - Removes other newlines.
  #[builder(default = "true")]
  pub newlines: bool,
  /// Trim leading and trailing whitespace from attribute values. (default: true)
  #[builder(default = "true")]
  pub trim: bool,
  /// Collapse multiple whitespace characters into a single space. (default: true)
  #[builder(default = "true")]
  pub spaces: bool,
}

pub struct CleanupAttrs<'a> {
  arena: &'a Bump,
  config: CleanupAttrsConfig,
  // Compiled regexes for efficiency
  reg_newlines_need_space: Regex,
  reg_newlines: Regex,
  reg_spaces: Regex,
}

impl<'a> CleanupAttrs<'a> {
  pub fn new(config: CleanupAttrsConfig, arena: &'a Bump) -> Self {
    // It's generally good practice to compile regexes once.
    // unwrap() is used here for simplicity; in production, consider error handling.
    let reg_newlines_need_space = Regex::new(r"(\S)\r?\n(\S)").unwrap();
    let reg_newlines = Regex::new(r"\r?\n").unwrap();
    let reg_spaces = Regex::new(r"\s{2,}").unwrap();

    Self {
      arena,
      config,
      reg_newlines_need_space,
      reg_newlines,
      reg_spaces,
    }
  }
}

impl<'a> Plugin<'a> for CleanupAttrs<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    // Assuming el.attributes is Vec<(&'a str, &'a str)> or similar
    // where the second element (value) can be reassigned to a new &'a str
    // allocated in the arena.
    for (_attr_name, attr_value_ref) in el.attributes.iter_mut() {
      let original_value: &'a str = attr_value_ref;
      let mut current_value: Cow<'a, str> = Cow::Borrowed(original_value);

      let mut modified = false;

      if self.config.newlines {
        // 1. Newlines that need a space (e.g., text\ntext -> text text)
        let after_needed_space = self
          .reg_newlines_need_space
          .replace_all(&current_value, "$1 $2");
        if after_needed_space.as_ref() != current_value.as_ref() {
          current_value = Cow::Owned(after_needed_space.into_owned());
          modified = true;
        }

        // 2. Other newlines (remove them)
        let after_simple_newlines = self.reg_newlines.replace_all(&current_value, "");
        if after_simple_newlines.as_ref() != current_value.as_ref() {
          current_value = Cow::Owned(after_simple_newlines.into_owned());
          modified = true;
        }
      }

      if self.config.trim {
        let trimmed_value = current_value.trim();
        if trimmed_value != current_value.as_ref() {
          current_value = Cow::Owned(trimmed_value.to_string());
          modified = true;
        }
      }

      if self.config.spaces {
        // Collapse multiple spaces into one
        let after_spaces = self.reg_spaces.replace_all(&current_value, " ");
        if after_spaces.as_ref() != current_value.as_ref() {
          current_value = Cow::Owned(after_spaces.into_owned());
          modified = true;
        }
      }

      // If the string was modified (and is now Cow::Owned),
      // allocate it in the arena and update the attribute value.
      if modified {
        if let Cow::Owned(owned_str) = current_value {
          // Allocate the modified string in the bump arena
          *attr_value_ref = BumpString::from_str_in(&owned_str, self.arena).into_bump_str();
        }
        // If it's still Cow::Borrowed, it means either no modifications were made,
        // or the modifications resulted in the original string, so no need to reassign.
      }
    }

    VisitAction::Keep // Keep the element, we only modified its attributes
  }
}
