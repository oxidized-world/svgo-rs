use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::{XMLAstChild, XMLAstElement};

pub struct MoveGroupAttrsToElemsPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct MoveGroupAttrsToElemsPluginConfig {}

impl<'a> MoveGroupAttrsToElemsPlugin<'a> {
  pub fn new(_config: MoveGroupAttrsToElemsPluginConfig, arena: &'a Bump) -> Self {
    MoveGroupAttrsToElemsPlugin { arena }
  }

  fn is_allowed_child(name: &str) -> bool {
    matches!(
      name,
      "path"
        | "g"
        | "text"
        | "glyph"
        | "missing-glyph"
        | "line"
        | "polyline"
        | "polygon"
        | "circle"
        | "ellipse"
        | "rect"
    )
  }

  fn is_reference_attr(name: &str) -> bool {
    matches!(
      name,
      "clip-path"
        | "color-profile"
        | "fill"
        | "filter"
        | "marker-start"
        | "marker-mid"
        | "marker-end"
        | "mask"
        | "stroke"
    )
  }

  fn has_url_reference(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("url(") && lower.contains('#')
  }
}

impl<'a> Plugin<'a> for MoveGroupAttrsToElemsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name != "g" || el.children.is_empty() {
      return VisitAction::Keep;
    }

    let transform = el.attributes.iter().find(|(k, _)| *k == "transform").map(|(_, v)| *v);
    let Some(group_transform) = transform else {
      return VisitAction::Keep;
    };

    let has_reference = el
      .attributes
      .iter()
      .any(|(name, value)| Self::is_reference_attr(name) && Self::has_url_reference(value));
    if has_reference {
      return VisitAction::Keep;
    }

    let can_move = el.children.iter().all(|child| match child {
      XMLAstChild::Element(child_el) => {
        Self::is_allowed_child(child_el.name)
          && child_el.attributes.iter().all(|(name, _)| *name != "id")
      }
      _ => false,
    });

    if !can_move {
      return VisitAction::Keep;
    }

    for child in &mut el.children {
      if let XMLAstChild::Element(child_el) = child {
        let existing = child_el
          .attributes
          .iter()
          .find(|(k, _)| *k == "transform")
          .map(|(_, v)| *v)
          .unwrap_or("");
        let next_transform = if existing.is_empty() {
          group_transform.to_string()
        } else {
          format!("{group_transform} {existing}")
        };

        let mut found = false;
        for (k, v) in &mut child_el.attributes {
          if *k == "transform" {
            *v = self.arena.alloc_str(&next_transform);
            found = true;
            break;
          }
        }
        if !found {
          child_el.attributes.push((
            self.arena.alloc_str("transform"),
            self.arena.alloc_str(&next_transform),
          ));
        }
      }
    }

    el.attributes.retain(|(k, _)| *k != "transform");
    VisitAction::Keep
  }
}
