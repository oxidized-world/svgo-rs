use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveDimensionsPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct RemoveDimensionsPluginConfig {}

impl<'a> RemoveDimensionsPlugin<'a> {
  pub fn new(_config: RemoveDimensionsPluginConfig, arena: &'a Bump) -> Self {
    RemoveDimensionsPlugin { arena }
  }
}

impl<'a> Plugin<'a> for RemoveDimensionsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name != "svg" {
      return VisitAction::Keep;
    }

    let view_box = el.attributes.iter().find(|(k, _)| *k == "viewBox").map(|(_, v)| *v);
    if view_box.is_some() {
      el.attributes.retain(|(k, _)| *k != "width" && *k != "height");
      return VisitAction::Keep;
    }

    let width = el.attributes.iter().find(|(k, _)| *k == "width").map(|(_, v)| *v);
    let height = el.attributes.iter().find(|(k, _)| *k == "height").map(|(_, v)| *v);
    let (Some(width), Some(height)) = (width, height) else {
      return VisitAction::Keep;
    };

    let width_num = width.parse::<f64>().ok();
    let height_num = height.parse::<f64>().ok();
    let (Some(width_num), Some(height_num)) = (width_num, height_num) else {
      return VisitAction::Keep;
    };

    let view_box_value = format!("0 0 {} {}", width_num, height_num);
    let mut has_view_box = false;
    for (k, v) in &mut el.attributes {
      if *k == "viewBox" {
        *v = self.arena.alloc_str(&view_box_value);
        has_view_box = true;
      }
    }
    if !has_view_box {
      el.attributes.push((
        self.arena.alloc_str("viewBox"),
        self.arena.alloc_str(&view_box_value),
      ));
    }

    el.attributes.retain(|(k, _)| *k != "width" && *k != "height");
    VisitAction::Keep
  }
}
