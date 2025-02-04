use crate::dom::SvgElement;
use crate::error::Result;

pub struct RemoveDoctypePlugin;

impl super::Plugin for RemoveDoctypePlugin {
  fn process_element(&self, element: &mut SvgElement) -> Result<()> {
    element.children.retain(|child| {
      if let crate::dom::SvgNode::DocType(_) = child {
        false
      } else {
        true
      }
    });
    Ok(())
  }
}
