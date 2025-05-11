use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement};
use bumpalo::collections::Vec as BumpVec;
use std::collections::HashSet;

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
      arena,
    }
  }
}

// Collection of inheritable attributes that can be moved up to parent groups
lazy_static::lazy_static! {
    static ref INHERITABLE_ATTRS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("clip-rule");
        set.insert("color-interpolation-filters");
        set.insert("color-interpolation");
        set.insert("color-profile");
        set.insert("color-rendering");
        set.insert("color");
        set.insert("cursor");
        set.insert("direction");
        set.insert("dominant-baseline");
        set.insert("fill-opacity");
        set.insert("fill-rule");
        set.insert("fill");
        set.insert("font-family");
        set.insert("font-size-adjust");
        set.insert("font-size");
        set.insert("font-stretch");
        set.insert("font-style");
        set.insert("font-variant");
        set.insert("font-weight");
        set.insert("font");
        set.insert("glyph-orientation-horizontal");
        set.insert("glyph-orientation-vertical");
        set.insert("image-rendering");
        set.insert("letter-spacing");
        set.insert("marker-end");
        set.insert("marker-mid");
        set.insert("marker-start");
        set.insert("marker");
        set.insert("paint-order");
        set.insert("pointer-events");
        set.insert("shape-rendering");
        set.insert("stroke-dasharray");
        set.insert("stroke-dashoffset");
        set.insert("stroke-linecap");
        set.insert("stroke-linejoin");
        set.insert("stroke-miterlimit");
        set.insert("stroke-opacity");
        set.insert("stroke-width");
        set.insert("stroke");
        set.insert("text-anchor");
        set.insert("text-rendering");
        set.insert("transform");
        set.insert("visibility");
        set.insert("word-spacing");
        set.insert("writing-mode");
        set
    };

    static ref PATH_ELEMS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("clip-path");
        set.insert("display");
        set.insert("filter");
        set.insert("mask");
        set.insert("opacity");
        set.insert("text-decoration");
        set.insert("transform");
        set.insert("unicode-bidi");
        set
    };
}

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

  fn element_exit(&mut self, el: &mut XMLAstElement<'a>) {
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
