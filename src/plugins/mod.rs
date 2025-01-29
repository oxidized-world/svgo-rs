mod common_attributes;
mod merge_classes;
mod remove_empty_text;

pub use common_attributes::CommonAttributesPlugin;
pub use merge_classes::MergeClassesPlugin;
pub use remove_empty_text::RemoveEmptyTextPlugin;

use crate::dom::SvgElement;
use crate::error::Result;

pub trait Plugin: Send + Sync {
  fn process_element(&self, element: &mut SvgElement) -> Result<()>;
}
