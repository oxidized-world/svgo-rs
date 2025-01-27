#![deny(clippy::all)]
use plugins::class_adder::ClassAdderPlugin;
use process_xml::{process_xml, XmlPlugin};

use napi_derive::napi;

mod plugins;
mod process_xml;

#[napi]
pub fn optimize(input_xml: String) -> String {
  let mut plugins: Vec<Box<dyn XmlPlugin>> = vec![Box::new(ClassAdderPlugin {
    target_element: "div".to_string(),
    class_name: "container".to_string(),
  })];

  let processed = process_xml(&input_xml, &mut plugins);

  match processed {
    Ok(output) => output,
    Err(e) => format!("Error: {}", e),
  }
}
