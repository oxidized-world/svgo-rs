use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveViewBoxPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
}

pub struct RemoveViewBoxPluginConfig {}

impl<'a> RemoveViewBoxPlugin<'a> {
  pub fn new(_config: RemoveViewBoxPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveViewBoxPlugin {
      _marker: std::marker::PhantomData,
    }
  }
}

impl<'a> Plugin<'a> for RemoveViewBoxPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if !(el.name == "svg" || el.name == "symbol" || el.name == "pattern") {
      return VisitAction::Keep;
    }

    let view_box = el.attributes.iter().find(|(k, _)| *k == "viewBox").map(|(_, v)| *v);
    let width = el.attributes.iter().find(|(k, _)| *k == "width").map(|(_, v)| *v);
    let height = el.attributes.iter().find(|(k, _)| *k == "height").map(|(_, v)| *v);
    let (Some(view_box), Some(width), Some(height)) = (view_box, width, height) else {
      return VisitAction::Keep;
    };

    let nums = view_box
      .split(|c: char| c == ' ' || c == ',')
      .filter(|s| !s.is_empty())
      .collect::<Vec<&str>>();
    if nums.len() != 4 {
      return VisitAction::Keep;
    }
    if nums[0] == "0"
      && nums[1] == "0"
      && width.trim_end_matches("px") == nums[2]
      && height.trim_end_matches("px") == nums[3]
    {
      el.attributes.retain(|(k, _)| *k != "viewBox");
    }

    VisitAction::Keep
  }
}
