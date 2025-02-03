use crate::dom::SvgElement;
use crate::error::Result;

pub struct RemoveDoctypePlugin;

impl super::Plugin for RemoveDoctypePlugin {
  fn process_element(&self, element: &mut SvgElement) -> Result<()> {
    if element.name == "!DOCTYPE" {}
    Ok(())
  }
}
