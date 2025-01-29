use crate::dom::SvgElement;
use crate::error::Result;
use std::collections::HashMap;

pub struct CommonAttributesPlugin;

impl super::Plugin for CommonAttributesPlugin {
  fn process_element(&self, element: &mut SvgElement) -> Result<()> {
    if element.name != "g" {
      return Ok(());
    }

    let mut common_attrs = HashMap::new();
    let mut first = true;

    // Collect common attributes from all child elements
    for child in &element.children {
      if let crate::dom::SvgNode::Element(child_elem) = child {
        if first {
          common_attrs = child_elem.attributes.clone();
          first = false;
        } else {
          common_attrs.retain(|k, v| child_elem.attributes.get(k) == Some(v));
        }
      }
    }

    // Remove attributes already present in parent
    common_attrs.retain(|k, _| !element.attributes.contains_key(k));

    // Apply to parent and remove from children
    if !common_attrs.is_empty() {
      element.attributes.extend(common_attrs.clone());
    }
    for child in &mut element.children {
      if let crate::dom::SvgNode::Element(child_elem) = child {
        child_elem.attributes.clear();
        element.attributes.extend(common_attrs.clone());
      }
    }

    Ok(())
  }
}
