use bumpalo::Bump;
use phf::phf_set;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct CollapseGroupsPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct CollapseGroupsPluginConfig {}

static ANIMATION_ELEMS: phf::Set<&'static str> = phf_set! {
  "animate", "animateColor", "animateMotion", "animateTransform", "set"
};

static INHERITABLE_ATTRS: phf::Set<&'static str> = phf_set! {
  "clip-rule",
  "color-interpolation-filters",
  "color-interpolation",
  "color-profile",
  "color-rendering",
  "color",
  "cursor",
  "direction",
  "dominant-baseline",
  "fill-opacity",
  "fill-rule",
  "fill",
  "font-family",
  "font-size-adjust",
  "font-size",
  "font-stretch",
  "font-style",
  "font-variant",
  "font-weight",
  "font",
  "glyph-orientation-horizontal",
  "glyph-orientation-vertical",
  "image-rendering",
  "letter-spacing",
  "marker-end",
  "marker-mid",
  "marker-start",
  "marker",
  "paint-order",
  "pointer-events",
  "shape-rendering",
  "stroke-dasharray",
  "stroke-dashoffset",
  "stroke-linecap",
  "stroke-linejoin",
  "stroke-miterlimit",
  "stroke-opacity",
  "stroke-width",
  "stroke",
  "text-anchor",
  "text-rendering",
  "transform",
  "visibility",
  "word-spacing",
  "writing-mode",
};

impl<'a> CollapseGroupsPlugin<'a> {
  pub fn new(_config: CollapseGroupsPluginConfig, arena: &'a Bump) -> Self {
    CollapseGroupsPlugin { arena }
  }

  fn process_children(
    &self,
    children: &mut bumpalo::collections::Vec<'a, XMLAstChild<'a>>,
    parent_name: Option<&str>,
    ancestor_has_filter: bool,
  ) {
    let mut i = 0;
    while i < children.len() {
      let mut node_has_filter = false;
      if let XMLAstChild::Element(el) = &mut children[i] {
        node_has_filter = Self::element_has_filter(el);
        self.process_children(
          &mut el.children,
          Some(el.name),
          ancestor_has_filter || node_has_filter,
        );
        if !matches!(parent_name, None | Some("switch"))
          && !(ancestor_has_filter || node_has_filter)
        {
          self.try_move_group_attrs_to_child(el);
        }
      }

      let mut should_collapse = false;
      let mut replacement_children: Vec<XMLAstChild<'a>> = Vec::new();

      if !matches!(parent_name, None | Some("switch")) && !(ancestor_has_filter || node_has_filter)
      {
        if let XMLAstChild::Element(el) = &children[i] {
          if el.name == "g" && el.attributes.is_empty() {
            let has_animation_child = el.children.iter().any(
              |child| matches!(child, XMLAstChild::Element(c) if ANIMATION_ELEMS.contains(c.name)),
            );
            if !has_animation_child {
              should_collapse = true;
              replacement_children = el.children.iter().cloned().collect();
            }
          }
        }
      }

      if should_collapse {
        children.remove(i);
        for (offset, child) in replacement_children.into_iter().enumerate() {
          children.insert(i + offset, child);
        }
        continue;
      }

      i += 1;
    }
  }

  fn try_move_group_attrs_to_child(&self, group: &mut XMLAstElement<'a>) {
    if group.name != "g" || group.attributes.is_empty() || group.children.len() != 1 {
      return;
    }

    if Self::element_has_filter(group) {
      return;
    }

    let XMLAstChild::Element(child) = &mut group.children[0] else {
      return;
    };

    if child.attributes.iter().any(|(k, _)| *k == "id") {
      return;
    }

    let mut new_child_attrs = child.attributes.clone();

    for (name, value) in &group.attributes {
      let mut found = false;
      for (child_name, child_value) in &mut new_child_attrs {
        if *child_name == *name {
          found = true;
          if *name == "transform" {
            let merged = self.arena.alloc_str(&format!("{} {}", *value, *child_value));
            *child_value = merged;
          } else if *child_value == "inherit" {
            *child_value = *value;
          } else if !INHERITABLE_ATTRS.contains(name) && *child_value != *value {
            return;
          }
        }
      }
      if !found {
        new_child_attrs.push((*name, *value));
      }
    }

    group.attributes.clear();
    child.attributes = new_child_attrs;
  }
}

impl<'a> CollapseGroupsPlugin<'a> {
  fn element_has_filter(el: &XMLAstElement<'a>) -> bool {
    let has_filter_attr = el.attributes.iter().any(|(k, _)| *k == "filter");
    let has_filter_in_style = el
      .attributes
      .iter()
      .find(|(k, _)| *k == "style")
      .map(|(_, v)| v.contains("filter"))
      .unwrap_or(false);
    has_filter_attr || has_filter_in_style
  }
}

impl<'a> Plugin<'a> for CollapseGroupsPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    self.process_children(&mut root.children, None, false);
  }
}
