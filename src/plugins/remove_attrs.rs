use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveAttrsPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
  pub preserve_current_color: bool,
  patterns: Vec<(Regex, Regex, Regex)>,
}

pub struct RemoveAttrsPluginConfig<'a> {
  pub attrs: Vec<&'a str>,
  pub elem_separator: &'a str,
  pub preserve_current_color: bool,
}

impl<'a> RemoveAttrsPlugin<'a> {
  pub fn new(config: RemoveAttrsPluginConfig<'a>, arena: &'a Bump) -> Self {
    let _ = arena;
    let mut patterns = Vec::new();
    for raw_pattern in config.attrs {
      let mut pattern = raw_pattern.to_string();
      if !pattern.contains(config.elem_separator) {
        pattern = format!(
          ".*{}{}{}.*",
          config.elem_separator, pattern, config.elem_separator
        );
      } else if pattern.split(config.elem_separator).count() < 3 {
        pattern = format!("{}{}.*", pattern, config.elem_separator);
      }

      let parts: Vec<String> = pattern
        .split(config.elem_separator)
        .map(|value| {
          if value == "*" {
            ".*".to_string()
          } else {
            value.to_string()
          }
        })
        .collect();
      if parts.len() != 3 {
        continue;
      }

      let elem = Regex::new(&format!("(?i)^{}$", parts[0]));
      let attr = Regex::new(&format!("(?i)^{}$", parts[1]));
      let value = Regex::new(&format!("(?i)^{}$", parts[2]));
      if let (Ok(elem), Ok(attr), Ok(value)) = (elem, attr, value) {
        patterns.push((elem, attr, value));
      }
    }

    RemoveAttrsPlugin {
      _marker: std::marker::PhantomData,
      preserve_current_color: config.preserve_current_color,
      patterns,
    }
  }
}

impl<'a> Plugin<'a> for RemoveAttrsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    for (elem_re, attr_re, val_re) in &self.patterns {
      if !elem_re.is_match(el.name) {
        continue;
      }
      el.attributes.retain(|(name, value)| {
        let is_current_color = value.eq_ignore_ascii_case("currentColor");
        let is_fill_or_stroke = *name == "fill" || *name == "stroke";
        if self.preserve_current_color && is_fill_or_stroke && is_current_color {
          return true;
        }
        !(attr_re.is_match(name) && val_re.is_match(value))
      });
    }
    VisitAction::Keep
  }
}
