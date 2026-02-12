use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct ConvertShapeToPathPlugin<'a> {
  pub arena: &'a Bump,
  pub convert_arcs: bool,
  reg_number: Regex,
}

pub struct ConvertShapeToPathPluginConfig {
  pub convert_arcs: bool,
}

impl<'a> ConvertShapeToPathPlugin<'a> {
  pub fn new(config: ConvertShapeToPathPluginConfig, arena: &'a Bump) -> Self {
    ConvertShapeToPathPlugin {
      arena,
      convert_arcs: config.convert_arcs,
      reg_number: Regex::new(r"[-+]?(?:\d*\.\d+|\d+\.?)(?:[eE][-+]?\d+)?").unwrap(),
    }
  }

  fn get_attr<'b>(el: &'b XMLAstElement<'a>, name: &str) -> Option<&'b str> {
    el.attributes.iter().find(|(k, _)| *k == name).map(|(_, v)| *v)
  }

  fn parse_num(value: Option<&str>, default: f64) -> Option<f64> {
    match value {
      Some(raw) => raw.parse::<f64>().ok(),
      None => Some(default),
    }
  }

  fn fmt_num(value: f64) -> String {
    if value.is_nan() || value.is_infinite() {
      return value.to_string();
    }
    let mut s = value.to_string();
    if s.contains('.') {
      while s.ends_with('0') {
        s.pop();
      }
      if s.ends_with('.') {
        s.pop();
      }
    }
    if s == "-0" {
      "0".to_string()
    } else {
      s
    }
  }

  fn set_path(el: &mut XMLAstElement<'a>, d: String, arena: &'a Bump) {
    el.name = arena.alloc_str("path");
    el.attributes.retain(|(k, _)| {
      !matches!(
        *k,
        "x"
          | "y"
          | "width"
          | "height"
          | "x1"
          | "y1"
          | "x2"
          | "y2"
          | "points"
          | "cx"
          | "cy"
          | "r"
          | "rx"
          | "ry"
      )
    });
    for (k, v) in &mut el.attributes {
      if *k == "d" {
        *v = arena.alloc_str(&d);
        return;
      }
    }
    el.attributes.push((arena.alloc_str("d"), arena.alloc_str(&d)));
  }
}

impl<'a> Plugin<'a> for ConvertShapeToPathPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "rect"
      && Self::get_attr(el, "width").is_some()
      && Self::get_attr(el, "height").is_some()
      && Self::get_attr(el, "rx").is_none()
      && Self::get_attr(el, "ry").is_none()
    {
      let x = Self::parse_num(Self::get_attr(el, "x"), 0.0);
      let y = Self::parse_num(Self::get_attr(el, "y"), 0.0);
      let width = Self::parse_num(Self::get_attr(el, "width"), 0.0);
      let height = Self::parse_num(Self::get_attr(el, "height"), 0.0);
      if let (Some(x), Some(y), Some(width), Some(height)) = (x, y, width, height) {
        if !(x - y + width - height).is_nan() {
          let d = format!(
            "M{} {}H{}V{}H{}z",
            Self::fmt_num(x),
            Self::fmt_num(y),
            Self::fmt_num(x + width),
            Self::fmt_num(y + height),
            Self::fmt_num(x)
          );
          Self::set_path(el, d, self.arena);
        }
      }
      return VisitAction::Keep;
    }

    if el.name == "line" {
      let x1 = Self::parse_num(Self::get_attr(el, "x1"), 0.0);
      let y1 = Self::parse_num(Self::get_attr(el, "y1"), 0.0);
      let x2 = Self::parse_num(Self::get_attr(el, "x2"), 0.0);
      let y2 = Self::parse_num(Self::get_attr(el, "y2"), 0.0);
      if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (x1, y1, x2, y2) {
        if !(x1 - y1 + x2 - y2).is_nan() {
          let d = format!(
            "M{} {} {} {}",
            Self::fmt_num(x1),
            Self::fmt_num(y1),
            Self::fmt_num(x2),
            Self::fmt_num(y2)
          );
          Self::set_path(el, d, self.arena);
        }
      }
      return VisitAction::Keep;
    }

    if (el.name == "polyline" || el.name == "polygon") && Self::get_attr(el, "points").is_some() {
      let points = Self::get_attr(el, "points").unwrap_or("");
      let coords: Vec<f64> = self
        .reg_number
        .find_iter(points)
        .filter_map(|m| m.as_str().parse::<f64>().ok())
        .collect();
      if coords.len() < 4 {
        return VisitAction::Remove;
      }

      let mut d = String::new();
      for i in (0..coords.len()).step_by(2) {
        if i + 1 >= coords.len() {
          break;
        }
        if i == 0 {
          d.push('M');
        }
        if i > 0 {
          d.push(' ');
        }
        d.push_str(&Self::fmt_num(coords[i]));
        d.push(' ');
        d.push_str(&Self::fmt_num(coords[i + 1]));
      }
      if el.name == "polygon" {
        d.push('z');
      }

      Self::set_path(el, d, self.arena);
      return VisitAction::Keep;
    }

    if self.convert_arcs && el.name == "circle" {
      let cx = Self::parse_num(Self::get_attr(el, "cx"), 0.0);
      let cy = Self::parse_num(Self::get_attr(el, "cy"), 0.0);
      let r = Self::parse_num(Self::get_attr(el, "r"), 0.0);
      if let (Some(cx), Some(cy), Some(r)) = (cx, cy, r) {
        if !(cx - cy + r).is_nan() {
          let d = format!(
            "M{} {}A{} {} 0 1 0 {} {} {} {} 0 1 0 {} {}z",
            Self::fmt_num(cx),
            Self::fmt_num(cy - r),
            Self::fmt_num(r),
            Self::fmt_num(r),
            Self::fmt_num(cx),
            Self::fmt_num(cy + r),
            Self::fmt_num(r),
            Self::fmt_num(r),
            Self::fmt_num(cx),
            Self::fmt_num(cy - r)
          );
          Self::set_path(el, d, self.arena);
        }
      }
      return VisitAction::Keep;
    }

    if self.convert_arcs && el.name == "ellipse" {
      let cx = Self::parse_num(Self::get_attr(el, "cx"), 0.0);
      let cy = Self::parse_num(Self::get_attr(el, "cy"), 0.0);
      let rx = Self::parse_num(Self::get_attr(el, "rx"), 0.0);
      let ry = Self::parse_num(Self::get_attr(el, "ry"), 0.0);
      if let (Some(cx), Some(cy), Some(rx), Some(ry)) = (cx, cy, rx, ry) {
        if !(cx - cy + rx - ry).is_nan() {
          let d = format!(
            "M{} {}A{} {} 0 1 0 {} {} {} {} 0 1 0 {} {}z",
            Self::fmt_num(cx),
            Self::fmt_num(cy - ry),
            Self::fmt_num(rx),
            Self::fmt_num(ry),
            Self::fmt_num(cx),
            Self::fmt_num(cy + ry),
            Self::fmt_num(rx),
            Self::fmt_num(ry),
            Self::fmt_num(cx),
            Self::fmt_num(cy - ry)
          );
          Self::set_path(el, d, self.arena);
        }
      }
      return VisitAction::Keep;
    }

    VisitAction::Keep
  }
}
