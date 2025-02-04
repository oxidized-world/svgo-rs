use crate::dom::{SvgElement, SvgNode};
use crate::error::Result;

/// Remove empty Text elements.
///
/// @see https://www.w3.org/TR/SVG11/text.html
///
/// Remove empty text element:
/// <text/>
///
/// Remove empty tspan element:
/// <tspan/>
///
/// Remove tref with empty xlink:href attribute:
/// <tref xlink:href=""/>
pub struct RemoveEmptyTextPlugin;

impl super::Plugin for RemoveEmptyTextPlugin {
  fn process_element(&self, element: &mut SvgElement) -> Result<()> {
    fn delete_empty_elements(
      current_node: &SvgNode,
      parent_element: &mut SvgElement,
    ) -> Result<()> {
      match current_node {
        SvgNode::Element(cur_element) => {
          if (cur_element.name == "text" || cur_element.name == "tspan")
            && cur_element.children.len() == 0
          {
            parent_element
              .children
              .retain(|child| child != current_node);
          }
          Ok(())
        }
        _ => Ok(()),
      }
    }
    let parent = element as *mut SvgElement;
    element.children.iter_mut().for_each(|child| {
      delete_empty_elements(child, unsafe { &mut *parent }).unwrap();
    });
    Ok(())
  }
}
