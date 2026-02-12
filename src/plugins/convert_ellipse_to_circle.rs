use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct ConvertEllipseToCirclePlugin<'a> {
  pub arena: &'a Bump,
}

pub struct ConvertEllipseToCirclePluginConfig {}

impl<'a> ConvertEllipseToCirclePlugin<'a> {
  pub fn new(_config: ConvertEllipseToCirclePluginConfig, arena: &'a Bump) -> Self {
    ConvertEllipseToCirclePlugin { arena }
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
}

impl<'a> Plugin<'a> for ConvertEllipseToCirclePlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name != "ellipse" {
      return VisitAction::Keep;
    }

    let rx = Self::get_attr(el, "rx").unwrap_or("0").to_string();
    let ry = Self::get_attr(el, "ry").unwrap_or("0").to_string();

    if rx == ry || rx == "auto" || ry == "auto" {
      let radius = if rx == "auto" { ry.clone() } else { rx.clone() };
      el.name = self.arena.alloc_str("circle");
      el.attributes.retain(|(k, _)| *k != "rx" && *k != "ry");
      Self::set_attr(el, self.arena.alloc_str("r"), self.arena.alloc_str(&radius));
    }

    VisitAction::Keep
  }
}
