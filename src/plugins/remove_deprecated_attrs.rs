use super::_collections::ATTRS_GROUPS_DEPRECATED;
use crate::error::Result;
use crate::utils::extract_css_selectors::extract_css_selectors;

pub struct RemoveDeprecatedAttrsPlugin;

impl super::Plugin for RemoveDeprecatedAttrsPlugin {
  fn process_element(&self, element: &mut crate::dom::SvgElement) -> Result<()> {
    let res = extract_css_selectors(
      r#"
      <style>
        @media (max-width: 600px) {
          .foo[name="bar"] {
            color: red;
          }
        }
        .name[name="bar"] {
          color: red;
          .b[test="bar"] {
            color: red;

            .c[bbbb="bar"] {
              color: red;
            }
          }
        }
      </style>
    "#,
    );
    println!("res: {:?}", res);
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
