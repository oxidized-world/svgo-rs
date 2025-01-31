pub mod dom;
pub mod error;
pub mod optimizer;
pub mod plugins;

use napi_derive::napi;
use optimizer::SvgOptimizer;
use plugins::{
  CommonAttributesPlugin, MergeClassesPlugin, RemoveDescPlugin, RemoveEmptyTextPlugin,
};

#[napi]
pub fn optimize(input_xml: String) -> String {
  let optimizer = SvgOptimizer::new(vec![
    Box::new(CommonAttributesPlugin),
    Box::new(MergeClassesPlugin),
    Box::new(RemoveEmptyTextPlugin),       // 传递参数
    Box::new(RemoveDescPlugin::new(true)), // 传递参数
  ]);
  let output = optimizer.optimize(input_xml.as_bytes()).unwrap();
  String::from_utf8(output).unwrap()
}
