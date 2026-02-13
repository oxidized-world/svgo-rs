use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveOffCanvasPathsPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
  view_box: Option<(f64, f64, f64, f64)>,
  reg_token: Regex,
}

pub struct RemoveOffCanvasPathsPluginConfig {}

impl<'a> RemoveOffCanvasPathsPlugin<'a> {
  pub fn new(_config: RemoveOffCanvasPathsPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveOffCanvasPathsPlugin {
      _marker: std::marker::PhantomData,
      view_box: None,
      reg_token: Regex::new(r"[A-Za-z]|[-+]?(?:\d*\.\d+|\d+\.?)(?:[eE][-+]?\d+)?").unwrap(),
    }
  }

  fn parse_view_box(value: &str) -> Option<(f64, f64, f64, f64)> {
    let cleaned = value
      .replace(',', " ")
      .replace('+', " ")
      .replace("px", " ")
      .split_whitespace()
      .map(|s| s.to_string())
      .collect::<Vec<String>>();
    if cleaned.len() != 4 {
      return None;
    }
    let left = cleaned[0].parse::<f64>().ok()?;
    let top = cleaned[1].parse::<f64>().ok()?;
    let width = cleaned[2].parse::<f64>().ok()?;
    let height = cleaned[3].parse::<f64>().ok()?;
    Some((left, top, left + width, top + height))
  }

  fn bbox_from_path(&self, d: &str) -> Option<(f64, f64, f64, f64)> {
    let tokens = self
      .reg_token
      .find_iter(d)
      .map(|m| m.as_str().to_string())
      .collect::<Vec<String>>();
    if tokens.is_empty() {
      return None;
    }

    let mut i = 0usize;
    let mut current_cmd = 'M';
    let mut cx = 0.0f64;
    let mut cy = 0.0f64;
    let mut sx = 0.0f64;
    let mut sy = 0.0f64;

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    let update_bbox =
      |x: f64, y: f64, min_x: &mut f64, min_y: &mut f64, max_x: &mut f64, max_y: &mut f64| {
        if x < *min_x {
          *min_x = x;
        }
        if y < *min_y {
          *min_y = y;
        }
        if x > *max_x {
          *max_x = x;
        }
        if y > *max_y {
          *max_y = y;
        }
      };

    while i < tokens.len() {
      let token = &tokens[i];
      if token.len() == 1 {
        let ch = token.chars().next().unwrap_or(' ');
        if ch.is_ascii_alphabetic() {
          current_cmd = ch;
          i += 1;
          if current_cmd == 'Z' || current_cmd == 'z' {
            cx = sx;
            cy = sy;
            update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          }
          continue;
        }
      }

      let read_num =
        |idx: usize, tokens: &[String]| -> Option<f64> { tokens.get(idx)?.parse::<f64>().ok() };

      match current_cmd {
        'M' | 'L' | 'T' => {
          let (Some(x), Some(y)) = (read_num(i, &tokens), read_num(i + 1, &tokens)) else {
            break;
          };
          cx = x;
          cy = y;
          if current_cmd == 'M' {
            sx = cx;
            sy = cy;
            current_cmd = 'L';
          }
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 2;
        }
        'm' | 'l' | 't' => {
          let (Some(dx), Some(dy)) = (read_num(i, &tokens), read_num(i + 1, &tokens)) else {
            break;
          };
          cx += dx;
          cy += dy;
          if current_cmd == 'm' {
            sx = cx;
            sy = cy;
            current_cmd = 'l';
          }
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 2;
        }
        'H' => {
          let Some(x) = read_num(i, &tokens) else {
            break;
          };
          cx = x;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 1;
        }
        'h' => {
          let Some(dx) = read_num(i, &tokens) else {
            break;
          };
          cx += dx;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 1;
        }
        'V' => {
          let Some(y) = read_num(i, &tokens) else {
            break;
          };
          cy = y;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 1;
        }
        'v' => {
          let Some(dy) = read_num(i, &tokens) else {
            break;
          };
          cy += dy;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 1;
        }
        'C' => {
          let (Some(x), Some(y)) = (read_num(i + 4, &tokens), read_num(i + 5, &tokens)) else {
            break;
          };
          cx = x;
          cy = y;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 6;
        }
        'c' => {
          let (Some(dx), Some(dy)) = (read_num(i + 4, &tokens), read_num(i + 5, &tokens)) else {
            break;
          };
          cx += dx;
          cy += dy;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 6;
        }
        'S' | 'Q' => {
          let (Some(x), Some(y)) = (read_num(i + 2, &tokens), read_num(i + 3, &tokens)) else {
            break;
          };
          cx = x;
          cy = y;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 4;
        }
        's' | 'q' => {
          let (Some(dx), Some(dy)) = (read_num(i + 2, &tokens), read_num(i + 3, &tokens)) else {
            break;
          };
          cx += dx;
          cy += dy;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 4;
        }
        'A' => {
          let (Some(x), Some(y)) = (read_num(i + 5, &tokens), read_num(i + 6, &tokens)) else {
            break;
          };
          cx = x;
          cy = y;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 7;
        }
        'a' => {
          let (Some(dx), Some(dy)) = (read_num(i + 5, &tokens), read_num(i + 6, &tokens)) else {
            break;
          };
          cx += dx;
          cy += dy;
          update_bbox(cx, cy, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
          i += 7;
        }
        _ => {
          i += 1;
        }
      }
    }

    if !min_x.is_finite() || !min_y.is_finite() || !max_x.is_finite() || !max_y.is_finite() {
      None
    } else {
      Some((min_x, min_y, max_x, max_y))
    }
  }

  fn bbox_intersects(bbox: (f64, f64, f64, f64), view_box: (f64, f64, f64, f64)) -> bool {
    let (bx1, by1, bx2, by2) = bbox;
    let (vx1, vy1, vx2, vy2) = view_box;
    bx1 <= vx2 && bx2 >= vx1 && by1 <= vy2 && by2 >= vy1
  }
}

impl<'a> Plugin<'a> for RemoveOffCanvasPathsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "svg" && self.view_box.is_none() {
      if let Some(vb) = el.attributes.iter().find(|(k, _)| *k == "viewBox").map(|(_, v)| *v) {
        self.view_box = Self::parse_view_box(vb);
      } else {
        let w = el
          .attributes
          .iter()
          .find(|(k, _)| *k == "width")
          .and_then(|(_, v)| v.parse::<f64>().ok());
        let h = el
          .attributes
          .iter()
          .find(|(k, _)| *k == "height")
          .and_then(|(_, v)| v.parse::<f64>().ok());
        if let (Some(w), Some(h)) = (w, h) {
          self.view_box = Some((0.0, 0.0, w, h));
        }
      }
      return VisitAction::Keep;
    }

    if el.name != "path" {
      return VisitAction::Keep;
    }
    if el.attributes.iter().any(|(k, _)| *k == "transform") {
      return VisitAction::Keep;
    }

    let Some(view_box) = self.view_box else {
      return VisitAction::Keep;
    };
    let Some((_, d)) = el.attributes.iter().find(|(k, _)| *k == "d") else {
      return VisitAction::Keep;
    };

    let Some(bbox) = self.bbox_from_path(d) else {
      return VisitAction::Keep;
    };

    if !Self::bbox_intersects(bbox, view_box) {
      return VisitAction::Remove;
    }

    VisitAction::Keep
  }
}
