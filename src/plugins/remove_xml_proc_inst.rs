use crate::dom::SvgElement;
use crate::error::Result;

pub struct RemoveXMLProcInstPlugin;

impl super::Plugin for RemoveXMLProcInstPlugin {
  fn process_element(&self, element: &mut SvgElement) -> Result<()> {
    element.children.retain(|child| {
      if let crate::dom::SvgNode::Decl(_) = child {
        false
      } else {
        true
      }
    });
    Ok(())
  }
}
