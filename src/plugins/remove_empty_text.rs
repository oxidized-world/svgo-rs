use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveEmptyTextPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
  pub text: bool,
  pub tspan: bool,
  pub tref: bool,
}

pub struct RemoveEmptyTextPluginConfig {
  pub text: bool,
  pub tspan: bool,
  pub tref: bool,
}

impl<'a> RemoveEmptyTextPlugin<'a> {
  pub fn new(config: RemoveEmptyTextPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveEmptyTextPlugin {
      _marker: std::marker::PhantomData,
      text: config.text,
      tspan: config.tspan,
      tref: config.tref,
    }
  }
}

impl<'a> Plugin<'a> for RemoveEmptyTextPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if self.text && el.name == "text" && el.children.is_empty() {
      return VisitAction::Remove;
    }
    if self.tspan && el.name == "tspan" && el.children.is_empty() {
      return VisitAction::Remove;
    }
    if self.tref
      && el.name == "tref"
      && !el.attributes.iter().any(|(k, _)| *k == "xlink:href" || *k == "href")
    {
      return VisitAction::Remove;
    }
    VisitAction::Keep
  }
}
