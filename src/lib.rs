mod parser;

use bumpalo::Bump;
use napi_derive::napi;
use parser::parse_svg;

#[napi]
pub fn optimize(input_xml: String) -> String {
  let arena = Bump::new();
  let res = parse_svg(&input_xml, &arena);

  format!("{:?}", res)
}
