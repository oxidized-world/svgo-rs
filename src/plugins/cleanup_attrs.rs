use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct CleanupAttrsPlugin<'a> {
  pub arena: &'a Bump,
  pub newlines: bool,
  pub trim: bool,
  pub spaces: bool,
  reg_newlines_need_space: Regex,
  reg_newlines: Regex,
  reg_spaces: Regex,
}

pub struct CleanupAttrsPluginConfig {
  pub newlines: bool,
  pub trim: bool,
  pub spaces: bool,
}

impl<'a> CleanupAttrsPlugin<'a> {
  pub fn new(config: CleanupAttrsPluginConfig, arena: &'a Bump) -> Self {
    CleanupAttrsPlugin {
      arena,
      newlines: config.newlines,
      trim: config.trim,
      spaces: config.spaces,
      reg_newlines_need_space: Regex::new(r"(\S)\r?\n(\S)").unwrap(),
      reg_newlines: Regex::new(r"\r?\n").unwrap(),
      reg_spaces: Regex::new(r"\s{2,}").unwrap(),
    }
  }
}

impl<'a> Plugin<'a> for CleanupAttrsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    for (_, attr_value) in &mut el.attributes {
      let mut value = (*attr_value).to_string();

      if self.newlines {
        value = self.reg_newlines_need_space.replace_all(&value, "$1 $2").to_string();
        value = self.reg_newlines.replace_all(&value, "").to_string();
      }

      if self.trim {
        value = value.trim().to_string();
      }

      if self.spaces {
        value = self.reg_spaces.replace_all(&value, " ").to_string();
      }

      *attr_value = self.arena.alloc_str(&value);
    }

    VisitAction::Keep
  }
}
