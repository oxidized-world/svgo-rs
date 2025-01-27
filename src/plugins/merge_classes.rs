// src/plugins/merge_classes.rs
use crate::dom::SvgElement;
use crate::error::Result;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
}

pub struct MergeClassesPlugin;

impl super::Plugin for MergeClassesPlugin {
  fn process_element(&self, element: &mut SvgElement) -> Result<()> {
    let classes: Vec<_> = element
      .attributes
      .iter()
      .filter(|(k, _)| k.as_str().eq_ignore_ascii_case("class"))
      .map(|(k, _)| k.clone())
      .collect();

    if classes.len() > 1 {
      let merged = classes
        .iter()
        .filter_map(|k| element.attributes.remove(k))
        .collect::<Vec<_>>()
        .join(" ");

      let normalized = WHITESPACE.replace_all(&merged, " ").trim().to_string();

      if !normalized.is_empty() {
        element.attributes.insert("class".to_string(), normalized);
      }
    }

    Ok(())
  }
}
