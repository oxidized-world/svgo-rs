use crate::dom::{SvgElement, SvgNode};
use crate::error::Result;

pub struct RemoveEmptyTextPlugin;

impl super::Plugin for RemoveEmptyTextPlugin {
  fn process_element(&self, element: &mut SvgElement) -> Result<()> {
    fn delete_empty_elements(
      current_node: &SvgNode,
      parent_element: &mut SvgElement,
    ) -> Result<()> {
      match current_node {
        SvgNode::Element(cur_element) => {
          println!(
            "Element: {:?}, children: {:?}, len: {:?}",
            cur_element.name,
            cur_element.children,
            cur_element.children.len()
          );
          if (cur_element.name == "text" || cur_element.name == "tspan")
            && cur_element.children.len() == 0
          {
            print!(
              "Deleting empty element: {:?}, children: {:?}",
              &cur_element.name, &parent_element.children
            );
            parent_element
              .children
              .retain(|child| child != current_node);
          }
          Ok(())
        }
        SvgNode::Text(_) => Ok(()),
      }
    }
    let parent = element as *mut SvgElement;
    element.children.iter_mut().for_each(|child| {
      delete_empty_elements(child, unsafe { &mut *parent }).unwrap();
    });
    Ok(())
  }
}
