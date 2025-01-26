#![deny(clippy::all)]
use plugin_pipeline::PluginPipeline;
use plugins::class_style::ClassStylePlugin;
use plugins::uppercase_id::UppercaseIdPlugin;

use napi_derive::napi;

mod plugin_pipeline;
mod plugins;
mod xml_ast;

#[napi]
pub fn main() -> String {
  let input_xml = r#"
      <div class="container" id="main">
          <p class="text">Hello World</p>
          <span id="subtitle">Rust XML Plugin System</span>
      </div>
  "#;

  let mut pipeline = PluginPipeline::new();
  pipeline.add_plugin(Box::new(ClassStylePlugin));
  pipeline.add_plugin(Box::new(UppercaseIdPlugin));

  let output = pipeline.process(input_xml);
  match output {
    Ok(output) => return output,
    Err(err) => return err.to_string(),
  }
}
