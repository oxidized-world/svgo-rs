use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveUselessStrokeAndFillPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
  pub remove_stroke: bool,
  pub remove_fill: bool,
  pub remove_none: bool,
}

pub struct RemoveUselessStrokeAndFillPluginConfig {
  pub stroke: bool,
  pub fill: bool,
  pub remove_none: bool,
}

impl<'a> RemoveUselessStrokeAndFillPlugin<'a> {
  pub fn new(config: RemoveUselessStrokeAndFillPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveUselessStrokeAndFillPlugin {
      _marker: std::marker::PhantomData,
      remove_stroke: config.stroke,
      remove_fill: config.fill,
      remove_none: config.remove_none,
    }
  }

  fn is_shape(name: &str) -> bool {
    matches!(
      name,
      "path" | "rect" | "circle" | "ellipse" | "line" | "polyline" | "polygon"
    )
  }
}

impl<'a> Plugin<'a> for RemoveUselessStrokeAndFillPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if !Self::is_shape(el.name) {
      return VisitAction::Keep;
    }

    if self.remove_stroke {
      let stroke = el.attributes.iter().find(|(k, _)| *k == "stroke").map(|(_, v)| *v);
      let stroke_opacity =
        el.attributes.iter().find(|(k, _)| *k == "stroke-opacity").map(|(_, v)| *v);
      let stroke_width = el.attributes.iter().find(|(k, _)| *k == "stroke-width").map(|(_, v)| *v);

      if stroke == Some("none") || stroke_opacity == Some("0") || stroke_width == Some("0") {
        el.attributes.retain(|(name, _)| !name.starts_with("stroke"));
      }
    }

    if self.remove_fill {
      let fill = el.attributes.iter().find(|(k, _)| *k == "fill").map(|(_, v)| *v);
      let fill_opacity = el.attributes.iter().find(|(k, _)| *k == "fill-opacity").map(|(_, v)| *v);
      if fill == Some("none") || fill_opacity == Some("0") {
        el.attributes.retain(|(name, _)| !name.starts_with("fill-"));
        let has_fill = el.attributes.iter().any(|(k, _)| *k == "fill");
        if !has_fill {
          // keep explicit none
          el.attributes.push(("fill", "none"));
        }
      }
    }

    if self.remove_none {
      let stroke_none = el
        .attributes
        .iter()
        .find(|(k, _)| *k == "stroke")
        .map(|(_, v)| *v == "none")
        .unwrap_or(true);
      let fill_none = el
        .attributes
        .iter()
        .find(|(k, _)| *k == "fill")
        .map(|(_, v)| *v == "none")
        .unwrap_or(false);
      if stroke_none && fill_none {
        el.attributes.retain(|(name, _)| *name != "stroke");
      }
    }

    VisitAction::Keep
  }
}
