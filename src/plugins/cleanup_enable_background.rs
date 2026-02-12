use bumpalo::Bump;
use regex::Regex;
use std::cell::Cell;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct CleanupEnableBackgroundPlugin<'a> {
  pub arena: &'a Bump,
  has_filter: Cell<bool>,
  reg_enable_background: Regex,
}

pub struct CleanupEnableBackgroundPluginConfig {}

impl<'a> CleanupEnableBackgroundPlugin<'a> {
  pub fn new(_config: CleanupEnableBackgroundPluginConfig, arena: &'a Bump) -> Self {
    CleanupEnableBackgroundPlugin {
      arena,
      has_filter: Cell::new(false),
      reg_enable_background: Regex::new(
        r"^new\s0\s0\s([-+]?\d*\.?\d+(?:[eE][-+]?\d+)?)\s([-+]?\d*\.?\d+(?:[eE][-+]?\d+)?)$",
      )
      .unwrap(),
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

  fn remove_attr(el: &mut XMLAstElement<'a>, name: &str) {
    el.attributes.retain(|(k, _)| *k != name);
  }

  fn cleanup_value(
    &self,
    value: &str,
    node_name: &str,
    width: &str,
    height: &str,
  ) -> Option<String> {
    if let Some(caps) = self.reg_enable_background.captures(value) {
      let w = caps.get(1).map(|m| m.as_str()).unwrap_or("");
      let h = caps.get(2).map(|m| m.as_str()).unwrap_or("");
      if width == w && height == h {
        if node_name == "svg" {
          return None;
        }
        return Some("new".to_string());
      }
    }
    Some(value.to_string())
  }
}

impl<'a> Plugin<'a> for CleanupEnableBackgroundPlugin<'a> {
  fn root_enter(&self, root: &mut XMLAstRoot<'a>) {
    fn walk<'a>(children: &[XMLAstChild<'a>], has_filter: &Cell<bool>) {
      for child in children {
        if let XMLAstChild::Element(el) = child {
          if el.name == "filter" {
            has_filter.set(true);
            return;
          }
          walk(&el.children, has_filter);
          if has_filter.get() {
            return;
          }
        }
      }
    }
    self.has_filter.set(false);
    walk(&root.children, &self.has_filter);
  }

  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    let current = Self::get_attr(el, "enable-background");
    if current.is_none() {
      return VisitAction::Keep;
    }

    if !self.has_filter.get() {
      Self::remove_attr(el, "enable-background");
      return VisitAction::Keep;
    }

    let has_dimensions =
      Self::get_attr(el, "width").is_some() && Self::get_attr(el, "height").is_some();
    if matches!(el.name, "svg" | "mask" | "pattern") && has_dimensions {
      let value = current.unwrap_or("");
      let width = Self::get_attr(el, "width").unwrap_or("");
      let height = Self::get_attr(el, "height").unwrap_or("");
      if let Some(cleaned) = self.cleanup_value(value, el.name, width, height) {
        let allocated = self.arena.alloc_str(&cleaned);
        Self::set_attr(el, "enable-background", allocated);
      } else {
        Self::remove_attr(el, "enable-background");
      }
    }

    VisitAction::Keep
  }
}
