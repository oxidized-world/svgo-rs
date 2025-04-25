mod optimizer;
mod parser;
mod plugins;

use bumpalo::Bump;
use napi_derive::napi;
use optimizer::SvgOptimizer;
use parser::parse_svg;
use plugins::move_elems_attrs_to_group::{
  MoveElemsAttrsToGroupPlugin, MoveElemsAttrsToGroupPluginConfig,
};
use plugins::remove_comments::{RemoveCommentsConfig, RemoveCommentsPlugin};
use plugins::remove_desc::{RemoveDescPlugin, RemoveDescPluginConfig};
use plugins::remove_doctype::{RemoveDoctypePlugin, RemoveDoctypePluginConfig};
use plugins::remove_metadata::{RemoveMetadataPlugin, RemoveMetadataPluginConfig};
use plugins::remove_title::{RemoveTitlePlugin, RemoveTitlePluginConfig};
use plugins::remove_xml_proc_inst::{RemoveXMLProcInstPlugin, RemoveXMLProcInstPluginConfig};

#[napi]
pub fn optimize(input_xml: String) -> String {
  // 只有在 debug build 时才初始化 env_logger
  if cfg!(debug_assertions) {
    let _ = env_logger::try_init();
  }
  let arena = Bump::new();
  let mut root = parse_svg(&input_xml, &arena).unwrap();

  let optimizer = SvgOptimizer::new(vec![
    Box::new(RemoveDescPlugin::new(
      RemoveDescPluginConfig { remove_any: true },
      &arena,
    )),
    Box::new(RemoveDoctypePlugin::new(
      RemoveDoctypePluginConfig {},
      &arena,
    )),
    Box::new(RemoveTitlePlugin::new(RemoveTitlePluginConfig {}, &arena)),
    Box::new(RemoveCommentsPlugin::new(
      RemoveCommentsConfig {
        preserve_patterns: None,
      },
      &arena,
    )),
    Box::new(RemoveXMLProcInstPlugin::new(
      RemoveXMLProcInstPluginConfig {},
      &arena,
    )),
    Box::new(RemoveMetadataPlugin::new(
      RemoveMetadataPluginConfig {},
      &arena,
    )),
    Box::new(MoveElemsAttrsToGroupPlugin::new(
      MoveElemsAttrsToGroupPluginConfig {},
      &arena,
    )),
  ]);
  optimizer.optimize(&mut root)
}
