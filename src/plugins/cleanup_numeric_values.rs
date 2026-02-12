use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct CleanupNumericValuesPlugin<'a> {
  pub arena: &'a Bump,
  pub float_precision: i32,
  pub leading_zero: bool,
  pub default_px: bool,
  pub convert_to_px: bool,
  reg_numeric_values: Regex,
}

pub struct CleanupNumericValuesPluginConfig {
  pub float_precision: i32,
  pub leading_zero: bool,
  pub default_px: bool,
  pub convert_to_px: bool,
}

impl<'a> CleanupNumericValuesPlugin<'a> {
  pub fn new(config: CleanupNumericValuesPluginConfig, arena: &'a Bump) -> Self {
    CleanupNumericValuesPlugin {
      arena,
      float_precision: config.float_precision,
      leading_zero: config.leading_zero,
      default_px: config.default_px,
      convert_to_px: config.convert_to_px,
      reg_numeric_values: Regex::new(
        r"^([-+]?\d*\.?\d+(?:[eE][-+]?\d+)?)(px|pt|pc|mm|cm|m|in|ft|em|ex|%)?$",
      )
      .unwrap(),
    }
  }

  fn round_to_precision(&self, value: f64) -> f64 {
    let p = 10f64.powi(self.float_precision);
    (value * p).round() / p
  }

  fn remove_leading_zero(&self, value: f64) -> String {
    let str_value = value.to_string();
    if 0.0 < value && value < 1.0 && str_value.starts_with('0') {
      return str_value[1..].to_string();
    }
    if -1.0 < value && value < 0.0 && str_value.len() > 1 && &str_value[1..2] == "0" {
      return format!("{}{}", &str_value[0..1], &str_value[2..]);
    }
    str_value
  }

  fn absolute_length_ratio(unit: &str) -> Option<f64> {
    match unit {
      "cm" => Some(96.0 / 2.54),
      "mm" => Some(96.0 / 25.4),
      "in" => Some(96.0),
      "pt" => Some(4.0 / 3.0),
      "pc" => Some(16.0),
      "px" => Some(1.0),
      _ => None,
    }
  }

  fn transform_numeric_token(&self, original: &str) -> Option<String> {
    let caps = self.reg_numeric_values.captures(original)?;
    let num_raw = caps.get(1)?.as_str();
    let mut units = caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
    let mut num = self.round_to_precision(num_raw.parse::<f64>().ok()?);

    if self.convert_to_px {
      if let Some(ratio) = Self::absolute_length_ratio(&units) {
        let px_num = self.round_to_precision(ratio * num_raw.parse::<f64>().ok()?);
        if px_num.to_string().len() < original.len() {
          num = px_num;
          units = "px".to_string();
        }
      }
    }

    let mut str_num = if self.leading_zero {
      self.remove_leading_zero(num)
    } else {
      num.to_string()
    };

    if self.default_px && units == "px" {
      units.clear();
    }

    str_num.push_str(&units);
    Some(str_num)
  }

  fn cleanup_view_box(&self, value: &str) -> String {
    value
      .split(|ch: char| ch == ',' || ch.is_whitespace())
      .filter(|item| !item.is_empty())
      .map(|token| {
        token
          .parse::<f64>()
          .ok()
          .map(|num| self.round_to_precision(num).to_string())
          .unwrap_or_else(|| token.to_string())
      })
      .collect::<Vec<_>>()
      .join(" ")
  }
}

impl<'a> Plugin<'a> for CleanupNumericValuesPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    for (attr_name, attr_value) in &mut el.attributes {
      if *attr_name == "viewBox" {
        let cleaned = self.cleanup_view_box(attr_value);
        *attr_value = self.arena.alloc_str(&cleaned);
        continue;
      }

      if *attr_name == "version" {
        continue;
      }

      if let Some(cleaned) = self.transform_numeric_token(attr_value) {
        *attr_value = self.arena.alloc_str(&cleaned);
      }
    }

    VisitAction::Keep
  }
}
