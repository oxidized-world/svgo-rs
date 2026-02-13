use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveXMLNSPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
}

pub struct RemoveXMLNSPluginConfig {}

impl<'a> RemoveXMLNSPlugin<'a> {
  pub fn new(_config: RemoveXMLNSPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveXMLNSPlugin {
      _marker: std::marker::PhantomData,
    }
  }
}

impl<'a> Plugin<'a> for RemoveXMLNSPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "svg" {
      el.attributes.retain(|(k, _)| *k != "xmlns");
    }
    VisitAction::Keep
  }
}
