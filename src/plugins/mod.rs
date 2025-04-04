mod _collections;
pub mod common_attributes;
pub mod remove_comments;
pub mod remove_deprecated_attrs;
pub mod remove_desc;
pub mod remove_doctype;
pub mod remove_empty_text;
pub mod remove_xml_proc_inst;

use crate::dom::SvgElement;
use crate::error::Result;

pub trait Plugin: Send + Sync {
  fn process_element(&self, element: &mut SvgElement) -> Result<()>;
}
