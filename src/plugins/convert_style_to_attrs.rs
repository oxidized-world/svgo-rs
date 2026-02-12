use bumpalo::Bump;
use phf::phf_set;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct ConvertStyleToAttrsPlugin<'a> {
  pub arena: &'a Bump,
  pub keep_important: bool,
}

pub struct ConvertStyleToAttrsPluginConfig {
  pub keep_important: bool,
}

static PRESENTATION_ATTRS: phf::Set<&'static str> = phf_set! {
  "alignment-baseline", "baseline-shift", "clip", "clip-path", "clip-rule", "color",
  "color-interpolation", "color-interpolation-filters", "color-profile", "color-rendering",
  "cursor", "direction", "display", "dominant-baseline", "enable-background", "fill",
  "fill-opacity", "fill-rule", "filter", "flood-color", "flood-opacity", "font-family",
  "font-size", "font-size-adjust", "font-stretch", "font-style", "font-variant", "font-weight",
  "glyph-orientation-horizontal", "glyph-orientation-vertical", "image-rendering", "kerning",
  "letter-spacing", "lighting-color", "marker-end", "marker-mid", "marker-start", "mask",
  "opacity", "overflow", "pointer-events", "shape-rendering", "stop-color", "stop-opacity",
  "stroke", "stroke-dasharray", "stroke-dashoffset", "stroke-linecap", "stroke-linejoin",
  "stroke-miterlimit", "stroke-opacity", "stroke-width", "text-anchor", "text-decoration",
  "text-rendering", "unicode-bidi", "visibility", "word-spacing", "writing-mode"
};

impl<'a> ConvertStyleToAttrsPlugin<'a> {
  pub fn new(config: ConvertStyleToAttrsPluginConfig, arena: &'a Bump) -> Self {
    ConvertStyleToAttrsPlugin {
      arena,
      keep_important: config.keep_important,
    }
  }

  fn get_attr<'b>(el: &'b XMLAstElement<'a>, name: &str) -> Option<&'b str> {
    el.attributes.iter().find(|(k, _)| *k == name).map(|(_, v)| *v)
  }

  fn set_attr(el: &mut XMLAstElement<'a>, name: &'a str, value: &'a str) {
    for (k, v) in &mut el.attributes {
      if *k == name {
        *v = value;
        return;
      }
    }
    el.attributes.push((name, value));
  }

  fn strip_comments(value: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = value.chars().collect();
    let mut i = 0;
    let mut in_single = false;
    let mut in_double = false;

    while i < chars.len() {
      let ch = chars[i];
      if ch == '\\' {
        out.push(ch);
        if i + 1 < chars.len() {
          out.push(chars[i + 1]);
          i += 2;
          continue;
        }
      }

      if !in_double && ch == '\'' {
        in_single = !in_single;
      } else if !in_single && ch == '"' {
        in_double = !in_double;
      }

      if !in_single && !in_double && ch == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
        i += 2;
        while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
          i += 1;
        }
        i = (i + 2).min(chars.len());
        continue;
      }

      out.push(ch);
      i += 1;
    }

    out
  }

  fn split_declarations(style: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut buf = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut paren_level = 0i32;

    for ch in style.chars() {
      if ch == '\\' {
        buf.push(ch);
        continue;
      }
      if !in_double && ch == '\'' {
        in_single = !in_single;
      } else if !in_single && ch == '"' {
        in_double = !in_double;
      } else if !in_single && !in_double {
        if ch == '(' {
          paren_level += 1;
        } else if ch == ')' {
          paren_level = (paren_level - 1).max(0);
        }
      }

      if ch == ';' && !in_single && !in_double && paren_level == 0 {
        let trimmed = buf.trim();
        if !trimmed.is_empty() {
          parts.push(trimmed.to_string());
        }
        buf.clear();
      } else {
        buf.push(ch);
      }
    }

    let trimmed = buf.trim();
    if !trimmed.is_empty() {
      parts.push(trimmed.to_string());
    }

    parts
  }
}

impl<'a> Plugin<'a> for ConvertStyleToAttrsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    let Some(style_raw) = Self::get_attr(el, "style") else {
      return VisitAction::Keep;
    };

    let style_value = Self::strip_comments(style_raw);
    let declarations = Self::split_declarations(&style_value);
    if declarations.is_empty() {
      return VisitAction::Keep;
    }

    let mut remain: Vec<String> = Vec::new();

    for declaration in declarations {
      let Some((raw_name, raw_value)) = declaration.split_once(':') else {
        remain.push(declaration);
        continue;
      };

      let name = raw_name.trim().to_lowercase();
      if name.is_empty() {
        continue;
      }

      let mut value = raw_value.trim().to_string();
      let has_important = value.to_ascii_lowercase().contains("!important");
      if has_important && self.keep_important {
        continue;
      }

      if has_important {
        value = value.replace("!important", "").replace("!IMPORTANT", "").trim().to_string();
      }

      if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        value = value[1..value.len() - 1].to_string();
      } else if value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2 {
        value = value[1..value.len() - 1].to_string();
      }

      if PRESENTATION_ATTRS.contains(name.as_str()) {
        Self::set_attr(
          el,
          self.arena.alloc_str(&name),
          self.arena.alloc_str(value.trim()),
        );
      } else {
        remain.push(format!("{name}:{value}"));
      }
    }

    if remain.is_empty() {
      el.attributes.retain(|(k, _)| *k != "style");
    } else {
      Self::set_attr(
        el,
        self.arena.alloc_str("style"),
        self.arena.alloc_str(&remain.join(";")),
      );
    }

    VisitAction::Keep
  }
}
