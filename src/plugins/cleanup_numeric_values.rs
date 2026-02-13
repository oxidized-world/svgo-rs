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

  fn split_numeric_and_unit(value: &str) -> Option<(&str, &str)> {
    let bytes = value.as_bytes();
    let len = bytes.len();
    if len == 0 {
      return None;
    }

    let mut i = 0;
    if matches!(bytes[i], b'+' | b'-') {
      i += 1;
      if i >= len {
        return None;
      }
    }

    let mut has_digit = false;
    while i < len && bytes[i].is_ascii_digit() {
      i += 1;
      has_digit = true;
    }

    if i < len && bytes[i] == b'.' {
      i += 1;
      while i < len && bytes[i].is_ascii_digit() {
        i += 1;
        has_digit = true;
      }
    }

    if !has_digit {
      return None;
    }

    if i < len && matches!(bytes[i], b'e' | b'E') {
      i += 1;
      if i < len && matches!(bytes[i], b'+' | b'-') {
        i += 1;
      }

      let exp_start = i;
      while i < len && bytes[i].is_ascii_digit() {
        i += 1;
      }
      if exp_start == i {
        return None;
      }
    }

    let unit = &value[i..];
    match unit {
      "" | "px" | "pt" | "pc" | "mm" | "cm" | "m" | "in" | "ft" | "em" | "ex" | "%" => {
        Some((&value[..i], unit))
      }
      _ => None,
    }
  }

  fn transform_numeric_token(&self, original: &str) -> Option<String> {
    let (num_raw, mut units) = if let Some((num, unit)) = Self::split_numeric_and_unit(original) {
      (num, unit)
    } else {
      let caps = self.reg_numeric_values.captures(original)?;
      (
        caps.get(1)?.as_str(),
        caps.get(2).map(|m| m.as_str()).unwrap_or(""),
      )
    };

    let parsed_num = num_raw.parse::<f64>().ok()?;
    let mut num = self.round_to_precision(parsed_num);

    if self.convert_to_px {
      if let Some(ratio) = Self::absolute_length_ratio(&units) {
        let px_num = self.round_to_precision(ratio * parsed_num);
        if px_num.to_string().len() < original.len() {
          num = px_num;
          units = "px";
        }
      }
    }

    let mut str_num = if self.leading_zero {
      self.remove_leading_zero(num)
    } else {
      num.to_string()
    };

    if self.default_px && units == "px" {
      units = "";
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
