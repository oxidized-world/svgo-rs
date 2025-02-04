use crate::dom::SvgElement;
use crate::error::Result;

/// 移除注释。
///
/// 示例
/// ```svg
/// <!-- Generator: Adobe Illustrator 15.0.0, SVG Export
/// Plug-In . SVG Version: 6.00 Build 0)  -->
/// ```
pub struct RemoveCommentsPlugin;

impl super::Plugin for RemoveCommentsPlugin {
  fn process_element(&self, element: &mut SvgElement) -> Result<()> {
    element.children.retain(|child| {
      if let crate::dom::SvgNode::Comment(_) = child {
        false
      } else {
        true
      }
    });
    Ok(())
  }
}
