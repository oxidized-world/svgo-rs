use bumpalo::Bump;
use phf::{phf_map, phf_set};
use regex::Regex;
use std::cell::Cell;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct ConvertColorsPlugin<'a> {
  pub arena: &'a Bump,
  pub current_color: Option<&'a str>,
  pub names2hex: bool,
  pub rgb2hex: bool,
  pub convert_case: &'a str,
  pub shorthex: bool,
  pub shortname: bool,
  mask_counter: Cell<i32>,
  reg_rgb: Regex,
}

pub struct ConvertColorsPluginConfig<'a> {
  pub current_color: Option<&'a str>,
  pub names2hex: bool,
  pub rgb2hex: bool,
  pub convert_case: &'a str,
  pub shorthex: bool,
  pub shortname: bool,
}

static COLORS_PROPS: phf::Set<&'static str> = phf_set! {
  "color",
  "fill",
  "flood-color",
  "lighting-color",
  "stop-color",
  "stroke",
};

static COLORS_NAMES: phf::Map<&'static str, &'static str> = phf_map! {
  "black" => "#000",
  "white" => "#fff",
  "red" => "#f00",
  "green" => "#008000",
  "blue" => "#00f",
  "yellow" => "#ff0",
  "cyan" => "#0ff",
  "magenta" => "#f0f",
  "fuchsia" => "#f0f",
  "aqua" => "#0ff",
  "navy" => "#000080",
  "gray" => "#808080",
  "grey" => "#808080",
  "silver" => "#c0c0c0",
  "maroon" => "#800000",
  "purple" => "#800080",
  "teal" => "#008080",
  "olive" => "#808000",
  "orange" => "#ffa500",
};

static COLORS_SHORT_NAMES: phf::Map<&'static str, &'static str> = phf_map! {
  "#000080" => "navy",
  "#808080" => "gray",
  "#008000" => "green",
  "#800000" => "maroon",
  "#800080" => "purple",
  "#008080" => "teal",
  "#808000" => "olive",
  "#ffa500" => "orange",
  "#ff0000" => "red",
  "#f00" => "red",
  "#c0c0c0" => "silver",
};

impl<'a> ConvertColorsPlugin<'a> {
  pub fn new(config: ConvertColorsPluginConfig<'a>, arena: &'a Bump) -> Self {
    ConvertColorsPlugin {
      arena,
      current_color: config.current_color,
      names2hex: config.names2hex,
      rgb2hex: config.rgb2hex,
      convert_case: config.convert_case,
      shorthex: config.shorthex,
      shortname: config.shortname,
      mask_counter: Cell::new(0),
      reg_rgb: Regex::new(r"^rgb\(\s*([+-]?(?:\d*\.?\d+|\d+\.?)%?)\s*(?:,\s*|\s+)([+-]?(?:\d*\.?\d+|\d+\.?)%?)\s*(?:,\s*|\s+)([+-]?(?:\d*\.?\d+|\d+\.?)%?)\s*\)$").unwrap(),
    }
  }

  fn includes_url_reference(value: &str) -> bool {
    value.contains("url(#") || value.contains("url('#") || value.contains("url(\"")
  }

  fn convert_rgb_to_hex(numbers: [i32; 3]) -> String {
    format!("#{:02X}{:02X}{:02X}", numbers[0], numbers[1], numbers[2])
  }

  fn parse_rgb_component(input: &str) -> Option<i32> {
    if input.ends_with('%') {
      let n = input.trim_end_matches('%').parse::<f64>().ok()?;
      let v = (n * 2.55).round() as i32;
      Some(v.clamp(0, 255))
    } else {
      let n = input.parse::<f64>().ok()? as i32;
      Some(n.clamp(0, 255))
    }
  }

  fn is_shortenable_hex_color(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 7 || bytes[0] != b'#' {
      return false;
    }
    let is_hex = |b: u8| b.is_ascii_hexdigit();
    is_hex(bytes[1])
      && is_hex(bytes[2])
      && is_hex(bytes[3])
      && is_hex(bytes[4])
      && is_hex(bytes[5])
      && is_hex(bytes[6])
      && bytes[1].eq_ignore_ascii_case(&bytes[2])
      && bytes[3].eq_ignore_ascii_case(&bytes[4])
      && bytes[5].eq_ignore_ascii_case(&bytes[6])
  }
}

impl<'a> Plugin<'a> for ConvertColorsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "mask" {
      self.mask_counter.set(self.mask_counter.get() + 1);
    }

    for (name, value) in &mut el.attributes {
      if !COLORS_PROPS.contains(name) {
        continue;
      }

      let mut val = (*value).to_string();

      if let Some(current_color) = self.current_color {
        if self.mask_counter.get() == 0 && (current_color == "*" || val == current_color) {
          val = "currentColor".to_string();
        }
      }

      if self.names2hex {
        let lowered = val.to_lowercase();
        if let Some(hex) = COLORS_NAMES.get(lowered.as_str()) {
          val = (*hex).to_string();
        }
      }

      if self.rgb2hex {
        if let Some(caps) = self.reg_rgb.captures(&val) {
          let r = Self::parse_rgb_component(caps.get(1).map(|m| m.as_str()).unwrap_or(""));
          let g = Self::parse_rgb_component(caps.get(2).map(|m| m.as_str()).unwrap_or(""));
          let b = Self::parse_rgb_component(caps.get(3).map(|m| m.as_str()).unwrap_or(""));
          if let (Some(r), Some(g), Some(b)) = (r, g, b) {
            val = Self::convert_rgb_to_hex([r, g, b]);
          }
        }
      }

      if !self.convert_case.is_empty()
        && !Self::includes_url_reference(&val)
        && val != "currentColor"
      {
        if self.convert_case == "lower" {
          val = val.to_lowercase();
        } else if self.convert_case == "upper" {
          val = val.to_uppercase();
        }
      }

      if self.shorthex {
        if Self::is_shortenable_hex_color(&val) {
          let chars: Vec<char> = val.chars().collect();
          val = format!("#{}{}{}", chars[1], chars[3], chars[5]);
        }
      }

      if self.shortname {
        let lowered = val.to_lowercase();
        if let Some(short_name) = COLORS_SHORT_NAMES.get(lowered.as_str()) {
          val = (*short_name).to_string();
        }
      }

      *value = self.arena.alloc_str(&val);
    }

    VisitAction::Keep
  }

  fn element_exit(&self, el: &mut XMLAstElement<'a>) {
    if el.name == "mask" {
      self.mask_counter.set(self.mask_counter.get() - 1);
    }
  }
}
