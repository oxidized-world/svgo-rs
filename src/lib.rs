mod optimizer;
mod parser;
mod plugins;

use bumpalo::Bump;
use napi_derive::napi;
use optimizer::SvgOptimizer;
use parser::parse_svg;
use plugins::move_elems_attrs_to_group::MoveElemsAttrsToGroupPlugin;
use plugins::remove_comments::RemoveCommentsPlugin;
use plugins::remove_desc::RemoveDescPlugin;
use plugins::remove_doctype::RemoveDoctypePlugin;
use plugins::remove_metadata::RemoveMetadataPlugin;
use plugins::remove_title::RemoveTitlePlugin;
use plugins::remove_xml_proc_inst::RemoveXMLProcInstPlugin;
use regex::Regex;

#[napi]
pub fn optimize(input_xml: String) -> String {
  // 只有在 debug build 时才初始化 env_logger
  if cfg!(debug_assertions) {
    let _ = env_logger::try_init();
  }
  let arena = Bump::new();
  let mut root = parse_svg(&input_xml, &arena).unwrap();

  let optimizer = SvgOptimizer::new(vec![
    Box::new(RemoveDescPlugin {
      remove_any: true,
      arena: &arena,
    }),
    Box::new(RemoveDoctypePlugin { arena: &arena }),
    Box::new(RemoveTitlePlugin { arena: &arena }),
    Box::new(RemoveCommentsPlugin {
      preserve_patterns: vec![Regex::new(r"^!").unwrap()],
      arena: &arena,
    }),
    Box::new(RemoveXMLProcInstPlugin { arena: &arena }),
    Box::new(RemoveMetadataPlugin { arena: &arena }),
    Box::new(MoveElemsAttrsToGroupPlugin::new(&arena)),
  ]);
  optimizer.optimize(&mut root)
}
