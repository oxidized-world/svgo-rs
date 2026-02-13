use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveStyleElementPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
}

pub struct RemoveStyleElementPluginConfig {}

impl<'a> RemoveStyleElementPlugin<'a> {
  pub fn new(_config: RemoveStyleElementPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveStyleElementPlugin {
      _marker: std::marker::PhantomData,
    }
  }
}

impl<'a> Plugin<'a> for RemoveStyleElementPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "style" {
      VisitAction::Remove
    } else {
      VisitAction::Keep
    }
  }
}
