// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptimizerError {
  #[error("XML parsing error: {0}")]
  XmlError(#[from] quick_xml::Error),

  #[error("Plugin error: {0}")]
  PluginError(String),
}

pub type Result<T> = std::result::Result<T, OptimizerError>;
