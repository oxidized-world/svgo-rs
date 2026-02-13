use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveElementsByAttrPlugin<'a> {
  pub ids: Vec<&'a str>,
  pub classes: Vec<&'a str>,
}

pub struct RemoveElementsByAttrPluginConfig<'a> {
  pub ids: Vec<&'a str>,
  pub classes: Vec<&'a str>,
}

impl<'a> RemoveElementsByAttrPlugin<'a> {
  pub fn new(config: RemoveElementsByAttrPluginConfig<'a>, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveElementsByAttrPlugin {
      ids: config.ids,
      classes: config.classes,
    }
  }
}

impl<'a> Plugin<'a> for RemoveElementsByAttrPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if !self.ids.is_empty() {
      let id = el.attributes.iter().find(|(k, _)| *k == "id").map(|(_, v)| *v);
      if let Some(id) = id {
        if self.ids.iter().any(|target| *target == id) {
          return VisitAction::Remove;
        }
      }
    }

    if !self.classes.is_empty() {
      let classes = el.attributes.iter().find(|(k, _)| *k == "class").map(|(_, v)| *v);
      if let Some(classes) = classes {
        let class_list = classes.split_whitespace().collect::<Vec<&str>>();
        if self
          .classes
          .iter()
          .any(|target| class_list.iter().any(|class_name| class_name == target))
        {
          return VisitAction::Remove;
        }
      }
    }

    VisitAction::Keep
  }
}
