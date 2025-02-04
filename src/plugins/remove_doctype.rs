use crate::dom::SvgElement;
use crate::error::Result;

/// Remove DOCTYPE declaration.
/// "Unfortunately the SVG DTDs are a source of so many
/// issues that the SVG WG has decided not to write one
/// for the upcoming SVG 1.2 standard. In fact SVG WG
/// members are even telling people not to use a DOCTYPE
/// declaration in SVG 1.0 and 1.1 documents"
/// https://jwatt.org/svg/authoring/#doctype-declaration
///
/// Examples:
///
/// ```svg
/// <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN"
/// q"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
///
/// <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN"
/// "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd" [
///     <!-- an internal subset can be embedded here -->
/// ]>
/// ```
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
