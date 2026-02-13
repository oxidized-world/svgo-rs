use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveHiddenElemsPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
  pub is_hidden: bool,
  pub display_none: bool,
  pub opacity0: bool,
  pub circle_r0: bool,
  pub ellipse_rx0: bool,
  pub ellipse_ry0: bool,
  pub rect_width0: bool,
  pub rect_height0: bool,
  pub pattern_width0: bool,
  pub pattern_height0: bool,
  pub image_width0: bool,
  pub image_height0: bool,
  pub path_empty_d: bool,
  pub polyline_empty_points: bool,
  pub polygon_empty_points: bool,
}

pub struct RemoveHiddenElemsPluginConfig {
  pub is_hidden: bool,
  pub display_none: bool,
  pub opacity0: bool,
  pub circle_r0: bool,
  pub ellipse_rx0: bool,
  pub ellipse_ry0: bool,
  pub rect_width0: bool,
  pub rect_height0: bool,
  pub pattern_width0: bool,
  pub pattern_height0: bool,
  pub image_width0: bool,
  pub image_height0: bool,
  pub path_empty_d: bool,
  pub polyline_empty_points: bool,
  pub polygon_empty_points: bool,
}

impl<'a> RemoveHiddenElemsPlugin<'a> {
  pub fn new(config: RemoveHiddenElemsPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveHiddenElemsPlugin {
      _marker: std::marker::PhantomData,
      is_hidden: config.is_hidden,
      display_none: config.display_none,
      opacity0: config.opacity0,
      circle_r0: config.circle_r0,
      ellipse_rx0: config.ellipse_rx0,
      ellipse_ry0: config.ellipse_ry0,
      rect_width0: config.rect_width0,
      rect_height0: config.rect_height0,
      pattern_width0: config.pattern_width0,
      pattern_height0: config.pattern_height0,
      image_width0: config.image_width0,
      image_height0: config.image_height0,
      path_empty_d: config.path_empty_d,
      polyline_empty_points: config.polyline_empty_points,
      polygon_empty_points: config.polygon_empty_points,
    }
  }

  fn attr<'b>(el: &'b XMLAstElement<'a>, name: &str) -> Option<&'b str> {
    el.attributes.iter().find(|(k, _)| *k == name).map(|(_, v)| *v)
  }

  fn has_visible_descendant(el: &XMLAstElement<'a>) -> bool {
    for child in &el.children {
      if let crate::parser::XMLAstChild::Element(child_el) = child {
        if Self::attr(child_el, "visibility") == Some("visible") {
          return true;
        }
        if Self::has_visible_descendant(child_el) {
          return true;
        }
      }
    }
    false
  }
}

impl<'a> Plugin<'a> for RemoveHiddenElemsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if self.display_none && Self::attr(el, "display") == Some("none") && el.name != "marker" {
      return VisitAction::Remove;
    }

    if self.opacity0 && Self::attr(el, "opacity") == Some("0") {
      return VisitAction::Remove;
    }

    if self.is_hidden && Self::attr(el, "visibility") == Some("hidden") {
      let has_class = Self::attr(el, "class").is_some();
      let style_visibility_visible = Self::attr(el, "style")
        .map(|v| v.to_ascii_lowercase().contains("visibility:visible"))
        .unwrap_or(false);
      if !has_class && !style_visibility_visible && !Self::has_visible_descendant(el) {
        return VisitAction::Remove;
      }
    }

    if self.circle_r0 && el.name == "circle" && Self::attr(el, "r") == Some("0") {
      return VisitAction::Remove;
    }
    if self.ellipse_rx0 && el.name == "ellipse" && Self::attr(el, "rx") == Some("0") {
      return VisitAction::Remove;
    }
    if self.ellipse_ry0 && el.name == "ellipse" && Self::attr(el, "ry") == Some("0") {
      return VisitAction::Remove;
    }
    if self.rect_width0 && el.name == "rect" && Self::attr(el, "width") == Some("0") {
      return VisitAction::Remove;
    }
    if self.rect_height0 && el.name == "rect" && Self::attr(el, "height") == Some("0") {
      return VisitAction::Remove;
    }
    if self.pattern_width0 && el.name == "pattern" && Self::attr(el, "width") == Some("0") {
      return VisitAction::Remove;
    }
    if self.pattern_height0 && el.name == "pattern" && Self::attr(el, "height") == Some("0") {
      return VisitAction::Remove;
    }
    if self.image_width0 && el.name == "image" && Self::attr(el, "width") == Some("0") {
      return VisitAction::Remove;
    }
    if self.image_height0 && el.name == "image" && Self::attr(el, "height") == Some("0") {
      return VisitAction::Remove;
    }

    if self.path_empty_d && el.name == "path" {
      let d = Self::attr(el, "d");
      if d.map(|v| v.trim().is_empty()).unwrap_or(true) {
        return VisitAction::Remove;
      }
    }

    if self.polyline_empty_points
      && el.name == "polyline"
      && Self::attr(el, "points").map(|v| v.trim().is_empty()).unwrap_or(true)
    {
      return VisitAction::Remove;
    }

    if self.polygon_empty_points
      && el.name == "polygon"
      && Self::attr(el, "points").map(|v| v.trim().is_empty()).unwrap_or(true)
    {
      return VisitAction::Remove;
    }

    VisitAction::Keep
  }
}
