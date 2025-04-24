mod optimizer;
mod parser;
mod plugins;

use bumpalo::Bump;
use napi_derive::napi;
use optimizer::SvgOptimizer;
use parser::parse_svg;
use plugins::remove_desc::RemoveDescPlugin;
use plugins::remove_doctype::RemoveDoctypePlugin;
use plugins::remove_title::RemoveTitlePlugin;

#[napi]
pub fn optimize(input_xml: String) -> String {
  // 只有在 debug build 时才初始化 env_logger
  if cfg!(debug_assertions) {
    let _ = env_logger::try_init();
  }
  let arena = Bump::new();
  let mut root = parse_svg(&input_xml, &arena).unwrap();

  let optimizer = SvgOptimizer::new(vec![
    Box::new(RemoveDescPlugin { remove_any: true }),
    Box::new(RemoveDoctypePlugin {}),
    Box::new(RemoveTitlePlugin {}),
  ]);
  optimizer.optimize(&mut root)
}
