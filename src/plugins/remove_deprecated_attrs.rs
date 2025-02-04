use super::_collections::ATTRS_GROUPS_DEPRECATED;
use crate::error::Result;

pub struct RemoveDeprecatedAttrs;

impl super::Plugin for RemoveDeprecatedAttrs {
  fn process_element(&self, element: &mut crate::dom::SvgElement) -> Result<()> {
    element.attributes.retain(|key, _| {
      if let Some(deprecated_attrs) = ATTRS_GROUPS_DEPRECATED.get(element.name.as_str()) {
        if let Some(unsafe_group) = deprecated_attrs.r#unsafe.as_ref() {
          !unsafe_group.contains(&key.as_str())
        } else {
          true
        }
      } else {
        true
      }
    });
    Ok(())
  }
}
