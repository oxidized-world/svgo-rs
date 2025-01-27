// src/plugins/mod.rs
mod common_attributes;
mod merge_classes;

pub use common_attributes::CommonAttributesPlugin;
pub use merge_classes::MergeClassesPlugin;

use crate::dom::SvgElement;
use crate::error::Result;

pub trait Plugin: Send + Sync {
    fn process_element(&self, element: &mut SvgElement) -> Result<()>;
}