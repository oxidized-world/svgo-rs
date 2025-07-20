use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement};
use bumpalo::collections::Vec as BumpVec;
use phf::{phf_set, Set};

/// Move common attributes of group children to the group
///
/// Example:
///
/// ```svg
/// <g attr1="val1">
///     <g attr2="val2">
///         text
///     </g>
///     <circle attr2="val2" attr3="val3"/>
/// </g>
///              ⬇
/// <g attr1="val1" attr2="val2">
///     <g>
///         text
///     </g>
///    <circle attr3="val3"/>
/// </g>
/// ```
pub struct MoveElemsAttrsToGroupPlugin<'a> {
  has_style_element: bool,
  arena: &'a Bump,
}

pub struct MoveElemsAttrsToGroupPluginConfig {}

impl<'a> MoveElemsAttrsToGroupPlugin<'a> {
  pub fn new(_config: MoveElemsAttrsToGroupPluginConfig, arena: &'a Bump) -> Self {
    MoveElemsAttrsToGroupPlugin {
      has_style_element: false,
      arena: arena,
    }
  }
}

// Collection of inheritable attributes that can be moved up to parent groups
static INHERITABLE_ATTRS: Set<&'static str> = phf_set! {
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

static PATH_ELEMS: Set<&'static str> = phf_set! {
  "clip-path",
  "display",
  "filter",
  "mask",
  "opacity",
  "text-decoration",
  "transform",
  "unicode-bidi",
};

impl<'a> Plugin<'a> for MoveElemsAttrsToGroupPlugin<'a> {
  fn root_enter(&self, _el: &mut crate::parser::XMLAstRoot<'a>) {
    let mut element_stack: Vec<&XMLAstChild<'a>> = Vec::new();
    for child in _el.children.iter() {
      element_stack.push(child);
    }
    let mut i = 0;
    while i < element_stack.len() {
      let element = &element_stack[i];
      match element {
        XMLAstChild::Element(_el) => {
          if _el.name == "style" {
            let this = self as *const _ as *mut MoveElemsAttrsToGroupPlugin;
            unsafe {
              (*this).has_style_element = true;
            }
            break;
          } else {
            for child in _el.children.iter() {
              element_stack.push(child);
            }
            i += 1
          }
        }
        _ => i += 1,
      }
    }
  }

  fn element_exit(&self, el: &mut XMLAstElement<'a>) {
    // Process only groups with more than 1 child
    if el.name != "g" || el.children.len() <= 1 {
      return;
    }

    // deoptimize when <style> is present
    if self.has_style_element {
      return;
    }

    let mut common_attributes: Option<BumpVec<'a, (&str, &str)>> = None;
    let mut every_child_is_path = true;
    let mut initial = true;

    // Find common attributes in group children
    for child in &el.children {
      if let XMLAstChild::Element(child_el) = child {
        // Check if all children are path elements
        if !PATH_ELEMS.contains(child_el.name) {
          every_child_is_path = false;
        }

        // Initialize common attributes from first child or compare with existing
        if initial {
          initial = false;
          // First element child, collect all inheritable attributes
          let mut attrs: BumpVec<'a, (&str, &str)> = BumpVec::new_in(self.arena);
          for (name, value) in &child_el.attributes {
            if INHERITABLE_ATTRS.contains(name) {
              attrs.push((*name, *value));
            }
          }
          common_attributes = Some(attrs);
        } else if let Some(ref mut attrs) = common_attributes {
          // Remove attributes that aren't common
          let mut to_remove = Vec::new();
          for (index, (name, value)) in attrs.iter().enumerate() {
            let mut found = false;
            for (child_name, child_value) in &child_el.attributes {
              if *name == *child_name && *value == *child_value {
                found = true;
                break;
              }
            }
            if !found {
              to_remove.push(index);
            }
          }
          // Remove non-common attributes
          for remove_index in to_remove {
            attrs.remove(remove_index);
          }
        }
      }
    }

    if let Some(ref mut common_attrs) = common_attributes {
      // Find if the group has filter, clip-path or mask attributes
      let has_filter = el.attributes.iter().any(|(name, _)| *name == "filter");
      let has_clip_path = el.attributes.iter().any(|(name, _)| *name == "clip-path");
      let has_mask = el.attributes.iter().any(|(name, _)| *name == "mask");

      // Preserve transform on children when group has filter or clip-path or mask
      if has_filter || has_clip_path || has_mask {
        common_attrs.retain(|(name, _)| *name != "transform");
      }

      // Preserve transform when all children are paths
      if every_child_is_path {
        common_attrs.retain(|(name, _)| *name != "transform");
      }

      for (name, value) in common_attrs.iter() {
        if *name == "transform" {
          let mut found = false;
          for (attr_name, attr_value) in el.attributes.iter_mut() {
            if *attr_name == "transform" {
              // Combine transform values
              let new_value = format!("{} {}", attr_value, value);
              // Create a string slice in the correct arena (self.arena)
              *attr_value = bumpalo::format!(in self.arena, "{}", new_value).into_bump_str(); // Corrected arena usage
              found = true;
              break;
            }
          }
          if !found {
            // Allocate name and value in the arena before pushing
            let allocated_name = bumpalo::format!(in self.arena, "{}", name).into_bump_str();
            let allocated_value = bumpalo::format!(in self.arena, "{}", value).into_bump_str();
            el.attributes.push((allocated_name, allocated_value));
          }
        } else {
          // Check if attribute already exists in parent
          let mut exists = false;
          for (attr_name, _) in &el.attributes {
            if *attr_name == *name {
              // Dereference attr_name and name for comparison
              exists = true;
              break;
            }
          }
          // Add attribute if not already present
          if !exists {
            // Allocate name and value in the arena before pushing
            let allocated_name = bumpalo::format!(in self.arena, "{}", name).into_bump_str();
            let allocated_value = bumpalo::format!(in self.arena, "{}", value).into_bump_str();
            el.attributes.push((allocated_name, allocated_value));
          }
        }
      }
    }

    // Remove common attributes from children
    if let Some(common_attrs) = &common_attributes {
      if !common_attrs.is_empty() {
        for child in &mut el.children {
          // Need mutable access
          if let XMLAstChild::Element(child_el) = child {
            child_el.attributes.retain(|(name, value)| {
              // Keep the attribute if it's NOT present in common_attrs (matching both name and value)
              !common_attrs
                .iter()
                .any(|(common_name, common_value)| *common_name == *name && *common_value == *value)
            });
          }
        }
      }
    }
  }
}
