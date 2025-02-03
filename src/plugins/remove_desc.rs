use napi_derive::napi;

#[napi(object)]
pub struct RemoveDescOptions {
  /// Remove any `desc` elements, even if they have children.
  pub remove_any: bool,
}

/// Removes <desc>.
/// Removes only standard editors content or empty elements because it can be
/// used for accessibility. Enable parameter 'removeAny' to remove any
/// description.
pub struct RemoveDescPlugin {
  pub options: RemoveDescOptions,
}

impl RemoveDescPlugin {
  pub fn new(options: RemoveDescOptions) -> Self {
    Self { options }
  }
}

use crate::error::Result;

impl super::Plugin for RemoveDescPlugin {
  fn process_element(&self, element: &mut crate::dom::SvgElement) -> Result<()> {
    fn delete_desc_elements(
      current_node: &crate::dom::SvgNode,
      parent_element: &mut crate::dom::SvgElement,
      remove_any: bool,
    ) -> Result<()> {
      match current_node {
        crate::dom::SvgNode::Element(cur_element) => {
          if cur_element.name == "desc" && (remove_any || cur_element.children.len() == 0) {
            parent_element
              .children
              .retain(|child| child != current_node);
          }
          Ok(())
        }
        _ => Ok(()),
      }
    }
    let parent = element as *mut crate::dom::SvgElement;
    element.children.iter_mut().for_each(|child| {
      delete_desc_elements(child, unsafe { &mut *parent }, self.options.remove_any).unwrap();
    });
    Ok(())
  }
}
