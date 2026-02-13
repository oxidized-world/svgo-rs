use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveEmptyAttrsPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
}

pub struct RemoveEmptyAttrsPluginConfig {}

impl<'a> RemoveEmptyAttrsPlugin<'a> {
  pub fn new(_config: RemoveEmptyAttrsPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveEmptyAttrsPlugin {
      _marker: std::marker::PhantomData,
    }
  }

  fn is_conditional_processing_attr(name: &str) -> bool {
    matches!(
      name,
      "requiredFeatures" | "requiredExtensions" | "systemLanguage"
    )
  }
}

impl<'a> Plugin<'a> for RemoveEmptyAttrsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    el.attributes
      .retain(|(name, value)| !(value.is_empty() && !Self::is_conditional_processing_attr(name)));
    VisitAction::Keep
  }
}
