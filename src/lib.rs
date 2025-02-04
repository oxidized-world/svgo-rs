pub mod dom;
pub mod error;
pub mod optimizer;
pub mod plugins;

use napi_derive::napi;
use optimizer::SvgOptimizer;
use plugins::common_attributes::CommonAttributesPlugin;
use plugins::remove_desc::{RemoveDescOptions, RemoveDescPlugin};
use plugins::remove_doctype::RemoveDoctypePlugin;
use plugins::remove_empty_text::RemoveEmptyTextPlugin;
use plugins::remove_xml_proc_inst::RemoveXMLProcInstPlugin;

#[napi(object)]
pub struct PluginConfig {
  pub remove_desc: RemoveDescOptions,
}

#[napi(object)]
pub struct OptimizeOptions {
  pub plugins: PluginConfig,
}

#[napi]
pub fn optimize(input_xml: String, options: OptimizeOptions) -> String {
  let optimizer = SvgOptimizer::new(vec![
    Box::new(CommonAttributesPlugin),
    Box::new(RemoveEmptyTextPlugin),
    Box::new(RemoveDescPlugin::new(options.plugins.remove_desc)),
    Box::new(RemoveDoctypePlugin),
    Box::new(RemoveXMLProcInstPlugin),
  ]);
  let output = optimizer.optimize(input_xml.as_bytes()).unwrap();
  String::from_utf8(output).unwrap()
}
