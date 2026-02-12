mod optimizer;
mod parser;
mod plugins;

use bumpalo::Bump;
use napi_derive::napi;
use optimizer::{Plugin, SvgOptimizer};
use parser::parse_svg;
use plugins::move_elems_attrs_to_group::{
  MoveElemsAttrsToGroupPlugin, MoveElemsAttrsToGroupPluginConfig,
};
use plugins::remove_comments::{RemoveCommentsConfig, RemoveCommentsPlugin};
use plugins::remove_desc::{RemoveDescPlugin, RemoveDescPluginConfig};
use plugins::remove_doctype::{RemoveDoctypePlugin, RemoveDoctypePluginConfig};
use plugins::remove_editors_ns_data::{RemoveEditorsNSData, RemoveEditorsNSDataConfig};
use plugins::remove_metadata::{RemoveMetadataPlugin, RemoveMetadataPluginConfig};
use plugins::remove_title::{RemoveTitlePlugin, RemoveTitlePluginConfig};
use plugins::remove_xml_proc_inst::{RemoveXMLProcInstPlugin, RemoveXMLProcInstPluginConfig};
use regex::Regex;

#[napi]
pub fn optimize(input_xml: String) -> String {
  // 只有在 debug build 时才初始化 env_logger
  if cfg!(debug_assertions) {
    let _ = env_logger::try_init();
  }
  let arena = Bump::new();
  let mut root = parse_svg(&input_xml, &arena).unwrap();
  let mut optimizer = SvgOptimizer::new(vec![
    Box::new(RemoveDescPlugin::new(
      RemoveDescPluginConfig { remove_any: false },
      &arena,
    )),
    Box::new(RemoveDoctypePlugin::new(
      RemoveDoctypePluginConfig {},
      &arena,
    )),
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
    Box::new(RemoveEditorsNSData::new(
      RemoveEditorsNSDataConfig {
        additional_namespace: None,
      },
      &arena,
    )),
  ]);
  optimizer.optimize(&mut root)
}

#[napi(object)]
pub struct OptimizeWithPluginsOptions {
  /// 要运行的插件列表（顺序生效），使用 svgo 的插件名：
  /// removeDesc/removeDoctype/removeComments/removeXMLProcInst/removeMetadata/
  /// moveElemsAttrsToGroup/removeEditorsNSData/removeTitle
  pub plugins: Vec<String>,

  /// removeDesc: 对齐 svgo 的 removeAny
  pub remove_desc_remove_any: Option<bool>,

  /// removeComments: preservePatterns（每个字符串会作为正则表达式编译）
  pub remove_comments_preserve_patterns: Option<Vec<String>>,
  /// removeComments: preservePatterns = false
  pub remove_comments_preserve_patterns_disabled: Option<bool>,

  /// removeEditorsNSData: additionalNamespaces
  pub remove_editors_ns_data_additional_namespaces: Option<Vec<String>>,
}

#[napi(js_name = "optimizeWithPlugins")]
pub fn optimize_with_plugins(input_xml: String, options: OptimizeWithPluginsOptions) -> napi::Result<String> {
  let arena = Bump::new();
  let mut root = parse_svg(&input_xml, &arena)
    .map_err(|e| napi::Error::from_reason(format!("parse svg failed: {e}")))?;

  let remove_desc_remove_any = options.remove_desc_remove_any.unwrap_or(false);

  let remove_comments_preserve_patterns = if options
    .remove_comments_preserve_patterns_disabled
    .unwrap_or(false)
  {
    Some(Vec::<Regex>::new())
  } else if let Some(patterns) = options.remove_comments_preserve_patterns {
    let mut compiled = Vec::with_capacity(patterns.len());
    for pat in patterns {
      compiled.push(
        Regex::new(&pat)
          .map_err(|e| napi::Error::from_reason(format!("invalid preserve pattern '{pat}': {e}")))?,
      );
    }
    Some(compiled)
  } else {
    None
  };

  let remove_editors_additional = options
    .remove_editors_ns_data_additional_namespaces
    .map(|items| {
      items
        .into_iter()
        .map(|s| {
          let allocated: &mut str = arena.alloc_str(&s);
          &*allocated
        })
        .collect::<Vec<&str>>()
    });

  let mut plugins: Vec<Box<dyn Plugin<'_> + '_>> = Vec::new();
  for name in options.plugins {
    match name.as_str() {
      "removeDesc" => plugins.push(Box::new(RemoveDescPlugin::new(
        RemoveDescPluginConfig {
          remove_any: remove_desc_remove_any,
        },
        &arena,
      ))),
      "removeDoctype" => plugins.push(Box::new(RemoveDoctypePlugin::new(
        RemoveDoctypePluginConfig {},
        &arena,
      ))),
      "removeComments" => plugins.push(Box::new(RemoveCommentsPlugin::new(
        RemoveCommentsConfig {
          preserve_patterns: remove_comments_preserve_patterns.clone(),
        },
        &arena,
      ))),
      "removeXMLProcInst" => plugins.push(Box::new(RemoveXMLProcInstPlugin::new(
        RemoveXMLProcInstPluginConfig {},
        &arena,
      ))),
      "removeMetadata" => plugins.push(Box::new(RemoveMetadataPlugin::new(
        RemoveMetadataPluginConfig {},
        &arena,
      ))),
      "moveElemsAttrsToGroup" => plugins.push(Box::new(MoveElemsAttrsToGroupPlugin::new(
        MoveElemsAttrsToGroupPluginConfig {},
        &arena,
      ))),
      "removeEditorsNSData" => plugins.push(Box::new(RemoveEditorsNSData::new(
        RemoveEditorsNSDataConfig {
          additional_namespace: remove_editors_additional.clone(),
        },
        &arena,
      ))),
      "removeTitle" => plugins.push(Box::new(RemoveTitlePlugin::new(
        RemoveTitlePluginConfig {},
        &arena,
      ))),
      _ => {
        return Err(napi::Error::from_reason(format!(
          "unknown plugin name: {name}"
        )))
      }
    }
  }

  let mut optimizer = SvgOptimizer::new(plugins);
  Ok(optimizer.optimize(&mut root))
}
