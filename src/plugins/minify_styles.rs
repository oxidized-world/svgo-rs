use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct MinifyStylesPlugin<'a> {
  pub arena: &'a Bump,
  reg_spaces: Regex,
  reg_around_symbols_css: Regex,
  reg_around_symbols_block: Regex,
}

pub struct MinifyStylesPluginConfig {}

impl<'a> MinifyStylesPlugin<'a> {
  pub fn new(_config: MinifyStylesPluginConfig, arena: &'a Bump) -> Self {
    MinifyStylesPlugin {
      arena,
      reg_spaces: Regex::new(r"\s+").unwrap(),
      reg_around_symbols_css: Regex::new(r"\s*([{}:;,])\s*").unwrap(),
      reg_around_symbols_block: Regex::new(r"\s*([:;,])\s*").unwrap(),
    }
  }

  fn strip_comments(css: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = css.chars().collect();
    let mut i = 0usize;
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

  fn minify_css(&self, css: &str) -> String {
    let css = Self::strip_comments(css);
    let compact = self.reg_spaces.replace_all(&css, " ");
    let compact = self.reg_around_symbols_css.replace_all(&compact, "$1");
    compact.trim().to_string()
  }

  fn minify_block(&self, css: &str) -> String {
    let css = Self::strip_comments(css);
    let compact = self.reg_spaces.replace_all(&css, " ");
    let compact = self.reg_around_symbols_block.replace_all(&compact, "$1");
    compact.trim().to_string()
  }

  fn minify_element(&self, el: &mut XMLAstElement<'a>) {
    if let Some((_, style_value)) = el.attributes.iter_mut().find(|(k, _)| *k == "style") {
      let minified = self.minify_block(style_value);
      if minified.is_empty() {
        el.attributes.retain(|(k, _)| *k != "style");
      } else {
        *style_value = self.arena.alloc_str(&minified);
      }
    }

    for child in &mut el.children {
      if let XMLAstChild::Element(child_el) = child {
        self.minify_element(child_el);
      }
    }
  }

  fn minify_children(&self, children: &mut BumpVec<'a, XMLAstChild<'a>>) {
    let mut i = 0usize;
    while i < children.len() {
      let mut remove_current = false;

      if let Some(XMLAstChild::Element(el)) = children.get_mut(i) {
        self.minify_element(el);

        if el.name == "style" {
          let mut has_css = false;
          for child in &mut el.children {
            match child {
              XMLAstChild::Text(text) => {
                let minified = self.minify_css(text.value);
                if !minified.is_empty() {
                  has_css = true;
                }
                text.value = self.arena.alloc_str(&minified);
              }
              XMLAstChild::Cdata(cdata) => {
                let minified = self.minify_css(cdata.value);
                if !minified.is_empty() {
                  has_css = true;
                }
                cdata.value = self.arena.alloc_str(&minified);
              }
              _ => {}
            }
          }
          remove_current = !has_css;
        } else {
          self.minify_children(&mut el.children);
        }
      }

      if remove_current {
        children.remove(i);
      } else {
        i += 1;
      }
    }
  }
}

impl<'a> Plugin<'a> for MinifyStylesPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    self.minify_children(&mut root.children);
  }
}
