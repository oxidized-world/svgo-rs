pub mod common_attributes;
pub mod remove_desc;
pub mod remove_doctype;
pub mod remove_empty_text;

use crate::dom::SvgElement;
use crate::error::Result;

pub trait Plugin: Send + Sync {
  fn process_element(&self, element: &mut SvgElement) -> Result<()>;
}
