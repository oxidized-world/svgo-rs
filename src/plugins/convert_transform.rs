use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct ConvertTransformPlugin<'a> {
  pub arena: &'a Bump,
  reg_fn: Regex,
}

pub struct ConvertTransformPluginConfig {}

impl<'a> ConvertTransformPlugin<'a> {
  pub fn new(_config: ConvertTransformPluginConfig, arena: &'a Bump) -> Self {
    ConvertTransformPlugin {
      arena,
      reg_fn: Regex::new(r"([a-zA-Z]+)\(([^)]*)\)").unwrap(),
    }
  }

  fn parse_numbers(args: &str) -> Vec<String> {
    args
      .split(|c: char| c == ',' || c.is_whitespace())
      .filter(|s| !s.is_empty())
      .map(|s| s.to_string())
      .collect()
  }

  fn is_zero(value: &str) -> bool {
    value.parse::<f64>().ok().map(|v| v == 0.0).unwrap_or(false)
  }

  fn is_one(value: &str) -> bool {
    value.parse::<f64>().ok().map(|v| v == 1.0).unwrap_or(false)
  }

  fn normalize_transform(&self, value: &str) -> Option<String> {
    let mut out: Vec<String> = Vec::new();

    for cap in self.reg_fn.captures_iter(value) {
      let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
      let args = cap.get(2).map(|m| m.as_str()).unwrap_or("");
      let nums = Self::parse_numbers(args);

      match name {
        "translate" => {
          if nums.is_empty() || nums.iter().all(|n| Self::is_zero(n)) {
            continue;
          }
          if nums.len() >= 2 && Self::is_zero(&nums[1]) {
            out.push(format!("translate({})", nums[0]));
          } else {
            out.push(format!("translate({})", nums.join(" ")));
          }
        }
        "scale" => {
          if nums.is_empty() || nums.iter().all(|n| Self::is_one(n)) {
            continue;
          }
          if nums.len() >= 2 && nums[0] == nums[1] {
            out.push(format!("scale({})", nums[0]));
          } else {
            out.push(format!("scale({})", nums.join(" ")));
          }
        }
        "rotate" | "skewX" | "skewY" => {
          if nums.first().map(|n| Self::is_zero(n)).unwrap_or(true) {
            continue;
          }
          out.push(format!("{}({})", name, nums.join(" ")));
        }
        "matrix" => {
          if nums.len() == 6
            && Self::is_one(&nums[0])
            && Self::is_zero(&nums[1])
            && Self::is_zero(&nums[2])
            && Self::is_one(&nums[3])
            && Self::is_zero(&nums[4])
            && Self::is_zero(&nums[5])
          {
            continue;
          }
          out.push(format!("matrix({})", nums.join(" ")));
        }
        _ => out.push(format!("{}({})", name, nums.join(" "))),
      }
    }

    if out.is_empty() {
      None
    } else {
      Some(out.join(""))
    }
  }
}

impl<'a> Plugin<'a> for ConvertTransformPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    for key in ["transform", "gradientTransform", "patternTransform"] {
      for (name, value) in &mut el.attributes {
        if *name == key {
          if let Some(normalized) = self.normalize_transform(value) {
            *value = self.arena.alloc_str(&normalized);
          } else {
            *value = "";
          }
        }
      }
      el.attributes.retain(|(name, value)| !(*name == key && value.is_empty()));
    }

    VisitAction::Keep
  }
}
