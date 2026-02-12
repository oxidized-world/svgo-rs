use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct CleanupListOfValuesPlugin<'a> {
  pub arena: &'a Bump,
  pub float_precision: i32,
  pub leading_zero: bool,
  pub default_px: bool,
  pub convert_to_px: bool,
  reg_numeric_values: Regex,
}

pub struct CleanupListOfValuesPluginConfig {
  pub float_precision: i32,
  pub leading_zero: bool,
  pub default_px: bool,
  pub convert_to_px: bool,
}

impl<'a> CleanupListOfValuesPlugin<'a> {
  pub fn new(config: CleanupListOfValuesPluginConfig, arena: &'a Bump) -> Self {
    CleanupListOfValuesPlugin {
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

  fn round_values(&self, input: &str) -> String {
    let mut rounded_list: Vec<String> = Vec::new();

    for elem in input
      .split(|ch: char| ch == ',' || ch.is_whitespace())
      .filter(|item| !item.is_empty())
    {
      if elem == "new" {
        rounded_list.push("new".to_string());
        continue;
      }

      if let Some(cleaned) = self.transform_numeric_token(elem) {
        rounded_list.push(cleaned);
      } else {
        rounded_list.push(elem.to_string());
      }
    }

    rounded_list.join(" ")
  }
}

impl<'a> Plugin<'a> for CleanupListOfValuesPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    for (name, value) in &mut el.attributes {
      let should_round = matches!(
        *name,
        "points" | "enable-background" | "viewBox" | "stroke-dasharray" | "dx" | "dy" | "x" | "y"
      );

      if should_round {
        let rounded = self.round_values(value);
        *value = self.arena.alloc_str(&rounded);
      }
    }

    VisitAction::Keep
  }
}
