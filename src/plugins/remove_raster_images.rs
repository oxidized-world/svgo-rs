use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveRasterImagesPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
  reg_raster: Regex,
}

pub struct RemoveRasterImagesPluginConfig {}

impl<'a> RemoveRasterImagesPlugin<'a> {
  pub fn new(_config: RemoveRasterImagesPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveRasterImagesPlugin {
      _marker: std::marker::PhantomData,
      reg_raster: Regex::new(r"(?i)(\.|image/)(jpe?g|png|gif)").unwrap(),
    }
  }
}

impl<'a> Plugin<'a> for RemoveRasterImagesPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name != "image" {
      return VisitAction::Keep;
    }
    let href = el
      .attributes
      .iter()
      .find(|(k, _)| *k == "xlink:href")
      .map(|(_, v)| *v)
      .unwrap_or("");
    if self.reg_raster.is_match(href) {
      return VisitAction::Remove;
    }
    VisitAction::Keep
  }
}
