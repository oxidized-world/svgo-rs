use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};

/// Remove <metadata>.
#[allow(dead_code)]
pub struct RemoveMetadataPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct RemoveMetadataPluginConfig {}

impl<'a> RemoveMetadataPlugin<'a> {
  pub fn new(_config: RemoveMetadataPluginConfig, arena: &'a Bump) -> Self {
    RemoveMetadataPlugin { arena: arena }
  }
}

impl<'a> Plugin<'a> for RemoveMetadataPlugin<'a> {
  fn element_enter(&self, el: &mut crate::parser::XMLAstElement<'a>) -> VisitAction {
    if el.name == "metadata" {
      VisitAction::Remove
    } else {
      VisitAction::Keep
    }
  }
}
