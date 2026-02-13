use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct ConvertPathDataPlugin<'a> {
  pub arena: &'a Bump,
  reg_token: Regex,
}

pub struct ConvertPathDataPluginConfig {}

impl<'a> ConvertPathDataPlugin<'a> {
  pub fn new(_config: ConvertPathDataPluginConfig, arena: &'a Bump) -> Self {
    ConvertPathDataPlugin {
      arena,
      reg_token: Regex::new(r"[a-zA-Z]|[-+]?(?:\d*\.\d+|\d+\.?)(?:[eE][-+]?\d+)?").unwrap(),
    }
  }

  fn fmt_num(value: f64) -> String {
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

  fn token_to_f64(token: &str) -> Option<f64> {
    token.parse::<f64>().ok()
  }

  fn minify_d(&self, value: &str) -> String {
    let tokens: Vec<&str> = self.reg_token.find_iter(value).map(|m| m.as_str()).collect();

    if tokens.is_empty() {
      return value.trim().to_string();
    }

    let mut out = String::new();
    let mut i = 0usize;
    let mut current_x = 0.0f64;
    let mut current_y = 0.0f64;
    let mut wrote_first_command = false;
    let mut emitted_commands = 0usize;

    while i < tokens.len() {
      let token = tokens[i];
      let Some(cmd) = token.chars().next() else {
        i += 1;
        continue;
      };
      if !cmd.is_ascii_alphabetic() {
        i += 1;
        continue;
      }

      i += 1;
      match cmd {
        'M' | 'm' => {
          if i + 1 >= tokens.len() {
            break;
          }
          let x = Self::token_to_f64(tokens[i]);
          let y = Self::token_to_f64(tokens[i + 1]);
          let (Some(x), Some(y)) = (x, y) else {
            break;
          };

          if !wrote_first_command {
            out.push('M');
          } else {
            out.push(cmd);
          }
          out.push_str(&Self::fmt_num(x));
          out.push(' ');
          out.push_str(&Self::fmt_num(y));
          wrote_first_command = true;
          emitted_commands += 1;

          current_x = if cmd == 'm' { current_x + x } else { x };
          current_y = if cmd == 'm' { current_y + y } else { y };
          i += 2;
        }
        'L' => {
          let mut implicit_after_m = false;
          if emitted_commands == 1 && out.starts_with('M') {
            out.replace_range(0..1, "m");
            implicit_after_m = true;
          } else if emitted_commands == 1 && out.starts_with('m') {
            implicit_after_m = true;
          }

          let mut first = true;
          while i + 1 < tokens.len() {
            if tokens[i].chars().next().map(|ch| ch.is_ascii_alphabetic()).unwrap_or(false) {
              break;
            }
            let x = Self::token_to_f64(tokens[i]);
            let y = Self::token_to_f64(tokens[i + 1]);
            let (Some(x), Some(y)) = (x, y) else {
              break;
            };
            let dx = x - current_x;
            let dy = y - current_y;
            if first {
              if !implicit_after_m {
                out.push('l');
                emitted_commands += 1;
              } else {
                out.push(' ');
              }
              first = false;
            } else {
              out.push(' ');
            }
            out.push_str(&Self::fmt_num(dx));
            out.push(' ');
            out.push_str(&Self::fmt_num(dy));
            current_x = x;
            current_y = y;
            i += 2;
          }
        }
        'H' => {
          let mut first = true;
          while i < tokens.len() {
            if tokens[i].chars().next().map(|ch| ch.is_ascii_alphabetic()).unwrap_or(false) {
              break;
            }
            let x = Self::token_to_f64(tokens[i]);
            let Some(x) = x else {
              break;
            };
            let dx = x - current_x;
            if first {
              out.push('h');
              first = false;
            } else {
              out.push(' ');
            }
            out.push_str(&Self::fmt_num(dx));
            current_x = x;
            i += 1;
          }
        }
        'V' => {
          let mut first = true;
          while i < tokens.len() {
            if tokens[i].chars().next().map(|ch| ch.is_ascii_alphabetic()).unwrap_or(false) {
              break;
            }
            let y = Self::token_to_f64(tokens[i]);
            let Some(y) = y else {
              break;
            };
            let dy = y - current_y;
            if first {
              out.push('v');
              first = false;
            } else {
              out.push(' ');
            }
            out.push_str(&Self::fmt_num(dy));
            current_y = y;
            i += 1;
          }
        }
        'z' | 'Z' => out.push('z'),
        _ => {
          out.push(cmd);
          while i < tokens.len() {
            if tokens[i].chars().next().map(|ch| ch.is_ascii_alphabetic()).unwrap_or(false) {
              break;
            }
            out.push_str(tokens[i]);
            i += 1;
            if i < tokens.len()
              && !tokens[i].chars().next().map(|ch| ch.is_ascii_alphabetic()).unwrap_or(false)
            {
              out.push(' ');
            }
          }
        }
      }
    }

    if out.contains("Ml") {
      out = out.replacen("Ml", "m", 1);
    }

    out.trim().to_string()
  }
}

impl<'a> Plugin<'a> for ConvertPathDataPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name != "path" {
      return VisitAction::Keep;
    }

    for (name, value) in &mut el.attributes {
      if *name == "d" {
        let minified = self.minify_d(value);
        *value = self.arena.alloc_str(&minified);
      }
    }

    VisitAction::Keep
  }
}
