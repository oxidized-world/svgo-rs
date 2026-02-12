mod optimizer;
mod parser;
mod plugins;

use bumpalo::Bump;
use napi_derive::napi;
use optimizer::{Plugin, SvgOptimizer};
use parser::parse_svg;
use plugins::add_attributes_to_svg_element::{
  AddAttributesToSVGElementPlugin, AddAttributesToSVGElementPluginConfig, AttributeSpec,
};
use plugins::add_classes_to_svg_element::{
  AddClassesToSVGElementPlugin, AddClassesToSVGElementPluginConfig,
};
use plugins::cleanup_attrs::{CleanupAttrsPlugin, CleanupAttrsPluginConfig};
use plugins::cleanup_enable_background::{
  CleanupEnableBackgroundPlugin, CleanupEnableBackgroundPluginConfig,
};
use plugins::cleanup_ids::{CleanupIdsPlugin, CleanupIdsPluginConfig};
use plugins::cleanup_list_of_values::{CleanupListOfValuesPlugin, CleanupListOfValuesPluginConfig};
use plugins::cleanup_numeric_values::{
  CleanupNumericValuesPlugin, CleanupNumericValuesPluginConfig,
};
use plugins::collapse_groups::{CollapseGroupsPlugin, CollapseGroupsPluginConfig};
use plugins::convert_colors::{ConvertColorsPlugin, ConvertColorsPluginConfig};
use plugins::convert_ellipse_to_circle::{
  ConvertEllipseToCirclePlugin, ConvertEllipseToCirclePluginConfig,
};
use plugins::convert_one_stop_gradients::{
  ConvertOneStopGradientsPlugin, ConvertOneStopGradientsPluginConfig,
};
use plugins::convert_path_data::{ConvertPathDataPlugin, ConvertPathDataPluginConfig};
use plugins::convert_shape_to_path::{ConvertShapeToPathPlugin, ConvertShapeToPathPluginConfig};
use plugins::convert_style_to_attrs::{ConvertStyleToAttrsPlugin, ConvertStyleToAttrsPluginConfig};
use plugins::convert_transform::{ConvertTransformPlugin, ConvertTransformPluginConfig};
use plugins::inline_styles::{InlineStylesPlugin, InlineStylesPluginConfig};
use plugins::merge_paths::{MergePathsPlugin, MergePathsPluginConfig};
use plugins::merge_styles::{MergeStylesPlugin, MergeStylesPluginConfig};
use plugins::minify_styles::{MinifyStylesPlugin, MinifyStylesPluginConfig};
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
  /// moveElemsAttrsToGroup/removeEditorsNSData/removeTitle/addAttributesToSVGElement
  pub plugins: Vec<String>,

  /// removeDesc: 对齐 svgo 的 removeAny
  pub remove_desc_remove_any: Option<bool>,

  /// removeComments: preservePatterns（每个字符串会作为正则表达式编译）
  pub remove_comments_preserve_patterns: Option<Vec<String>>,
  /// removeComments: preservePatterns = false
  pub remove_comments_preserve_patterns_disabled: Option<bool>,

  /// removeEditorsNSData: additionalNamespaces
  pub remove_editors_ns_data_additional_namespaces: Option<Vec<String>>,

  /// addAttributesToSVGElement: attribute（字符串）
  /// 支持两种格式：
  /// 1) name            -> 输出为 name=""
  /// 2) key=value       -> 输出为 key="value"
  pub add_attributes_to_svg_element_attribute: Option<String>,

  /// addAttributesToSVGElement: attributes（字符串数组）
  /// 每项支持格式：name 或 key=value
  pub add_attributes_to_svg_element_attributes: Option<Vec<String>>,

  /// addClassesToSVGElement: className（字符串）
  pub add_classes_to_svg_element_class_name: Option<String>,
  /// addClassesToSVGElement: classNames（字符串数组）
  pub add_classes_to_svg_element_class_names: Option<Vec<String>>,

  /// cleanupAttrs
  pub cleanup_attrs_newlines: Option<bool>,
  pub cleanup_attrs_trim: Option<bool>,
  pub cleanup_attrs_spaces: Option<bool>,

  /// cleanupNumericValues
  pub cleanup_numeric_values_float_precision: Option<i32>,
  pub cleanup_numeric_values_leading_zero: Option<bool>,
  pub cleanup_numeric_values_default_px: Option<bool>,
  pub cleanup_numeric_values_convert_to_px: Option<bool>,

  /// cleanupListOfValues
  pub cleanup_list_of_values_float_precision: Option<i32>,
  pub cleanup_list_of_values_leading_zero: Option<bool>,
  pub cleanup_list_of_values_default_px: Option<bool>,
  pub cleanup_list_of_values_convert_to_px: Option<bool>,

  /// convertColors
  pub convert_colors_current_color: Option<String>,
  pub convert_colors_current_color_enabled: Option<bool>,
  pub convert_colors_names2hex: Option<bool>,
  pub convert_colors_rgb2hex: Option<bool>,
  pub convert_colors_convert_case: Option<String>,
  pub convert_colors_shorthex: Option<bool>,
  pub convert_colors_shortname: Option<bool>,

  /// cleanupIds
  pub cleanup_ids_remove: Option<bool>,
  pub cleanup_ids_minify: Option<bool>,
  pub cleanup_ids_preserve: Option<Vec<String>>,
  pub cleanup_ids_preserve_prefixes: Option<Vec<String>>,
  pub cleanup_ids_force: Option<bool>,

  /// convertShapeToPath
  pub convert_shape_to_path_convert_arcs: Option<bool>,

  /// convertStyleToAttrs
  pub convert_style_to_attrs_keep_important: Option<bool>,

  /// mergePaths
  pub merge_paths_force: Option<bool>,
}

#[napi(js_name = "optimizeWithPlugins")]
pub fn optimize_with_plugins(
  input_xml: String,
  options: OptimizeWithPluginsOptions,
) -> napi::Result<String> {
  let arena = Bump::new();
  let mut root = parse_svg(&input_xml, &arena)
    .map_err(|e| napi::Error::from_reason(format!("parse svg failed: {e}")))?;

  let remove_desc_remove_any = options.remove_desc_remove_any.unwrap_or(false);

  let remove_comments_preserve_patterns =
    if options.remove_comments_preserve_patterns_disabled.unwrap_or(false) {
      Some(Vec::<Regex>::new())
    } else if let Some(patterns) = options.remove_comments_preserve_patterns {
      let mut compiled = Vec::with_capacity(patterns.len());
      for pat in patterns {
        compiled.push(Regex::new(&pat).map_err(|e| {
          napi::Error::from_reason(format!("invalid preserve pattern '{pat}': {e}"))
        })?);
      }
      Some(compiled)
    } else {
      None
    };

  let remove_editors_additional =
    options.remove_editors_ns_data_additional_namespaces.map(|items| {
      items
        .into_iter()
        .map(|s| {
          let allocated: &mut str = arena.alloc_str(&s);
          &*allocated
        })
        .collect::<Vec<&str>>()
    });

  let mut add_attributes_specs: Vec<AttributeSpec<'_>> = Vec::new();

  if let Some(attribute) = options.add_attributes_to_svg_element_attribute {
    let mut parts = attribute.splitn(2, '=');
    let raw_key = parts.next().unwrap_or("").trim();
    let raw_value = parts.next().map(str::trim);
    if !raw_key.is_empty() {
      let key = {
        let allocated: &mut str = arena.alloc_str(raw_key);
        &*allocated
      };
      if let Some(value) = raw_value {
        let value_alloc = {
          let allocated: &mut str = arena.alloc_str(value);
          &*allocated
        };
        add_attributes_specs.push(AttributeSpec::KeyValue(key, value_alloc));
      } else {
        add_attributes_specs.push(AttributeSpec::Name(key));
      }
    }
  }

  if let Some(attributes) = options.add_attributes_to_svg_element_attributes {
    for attribute in attributes {
      let mut parts = attribute.splitn(2, '=');
      let raw_key = parts.next().unwrap_or("").trim();
      let raw_value = parts.next().map(str::trim);
      if raw_key.is_empty() {
        continue;
      }
      let key = {
        let allocated: &mut str = arena.alloc_str(raw_key);
        &*allocated
      };
      if let Some(value) = raw_value {
        let value_alloc = {
          let allocated: &mut str = arena.alloc_str(value);
          &*allocated
        };
        add_attributes_specs.push(AttributeSpec::KeyValue(key, value_alloc));
      } else {
        add_attributes_specs.push(AttributeSpec::Name(key));
      }
    }
  }

  let mut add_classes: Vec<&str> = Vec::new();
  if let Some(class_name) = options.add_classes_to_svg_element_class_name {
    let class_name = class_name.trim();
    if !class_name.is_empty() {
      add_classes.push(arena.alloc_str(class_name));
    }
  }
  if let Some(class_names) = options.add_classes_to_svg_element_class_names {
    for class_name in class_names {
      let class_name = class_name.trim();
      if !class_name.is_empty() {
        add_classes.push(arena.alloc_str(class_name));
      }
    }
  }

  let cleanup_attrs_config = CleanupAttrsPluginConfig {
    newlines: options.cleanup_attrs_newlines.unwrap_or(true),
    trim: options.cleanup_attrs_trim.unwrap_or(true),
    spaces: options.cleanup_attrs_spaces.unwrap_or(true),
  };

  let cleanup_numeric_values_config = CleanupNumericValuesPluginConfig {
    float_precision: options.cleanup_numeric_values_float_precision.unwrap_or(3),
    leading_zero: options.cleanup_numeric_values_leading_zero.unwrap_or(true),
    default_px: options.cleanup_numeric_values_default_px.unwrap_or(true),
    convert_to_px: options.cleanup_numeric_values_convert_to_px.unwrap_or(true),
  };

  let cleanup_list_of_values_config = CleanupListOfValuesPluginConfig {
    float_precision: options.cleanup_list_of_values_float_precision.unwrap_or(3),
    leading_zero: options.cleanup_list_of_values_leading_zero.unwrap_or(true),
    default_px: options.cleanup_list_of_values_default_px.unwrap_or(true),
    convert_to_px: options.cleanup_list_of_values_convert_to_px.unwrap_or(true),
  };

  let convert_colors_current_color =
    if options.convert_colors_current_color_enabled.unwrap_or(false) {
      Some(&*arena.alloc_str("*"))
    } else {
      options
        .convert_colors_current_color
        .as_ref()
        .map(|value| {
          let allocated: &mut str = arena.alloc_str(value.trim());
          &*allocated
        })
        .filter(|value| !value.is_empty())
    };

  let convert_case_value = options
    .convert_colors_convert_case
    .as_ref()
    .map(|value| value.trim())
    .filter(|value| !value.is_empty())
    .unwrap_or("lower");
  let convert_case_value = arena.alloc_str(convert_case_value);

  let convert_colors_config = ConvertColorsPluginConfig {
    current_color: convert_colors_current_color,
    names2hex: options.convert_colors_names2hex.unwrap_or(true),
    rgb2hex: options.convert_colors_rgb2hex.unwrap_or(true),
    convert_case: convert_case_value,
    shorthex: options.convert_colors_shorthex.unwrap_or(true),
    shortname: options.convert_colors_shortname.unwrap_or(true),
  };

  let cleanup_ids_preserve = options
    .cleanup_ids_preserve
    .unwrap_or_default()
    .into_iter()
    .map(|s| {
      let allocated: &mut str = arena.alloc_str(s.trim());
      &*allocated
    })
    .filter(|s| !s.is_empty())
    .collect::<Vec<&str>>();

  let cleanup_ids_preserve_prefixes = options
    .cleanup_ids_preserve_prefixes
    .unwrap_or_default()
    .into_iter()
    .map(|s| {
      let allocated: &mut str = arena.alloc_str(s.trim());
      &*allocated
    })
    .filter(|s| !s.is_empty())
    .collect::<Vec<&str>>();

  let cleanup_ids_config = CleanupIdsPluginConfig {
    remove: options.cleanup_ids_remove.unwrap_or(true),
    minify: options.cleanup_ids_minify.unwrap_or(true),
    preserve: cleanup_ids_preserve,
    preserve_prefixes: cleanup_ids_preserve_prefixes,
    force: options.cleanup_ids_force.unwrap_or(false),
  };

  let convert_shape_to_path_config = ConvertShapeToPathPluginConfig {
    convert_arcs: options.convert_shape_to_path_convert_arcs.unwrap_or(false),
  };

  let convert_style_to_attrs_config = ConvertStyleToAttrsPluginConfig {
    keep_important: options.convert_style_to_attrs_keep_important.unwrap_or(false),
  };

  let merge_paths_config = MergePathsPluginConfig {
    force: options.merge_paths_force.unwrap_or(false),
  };

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
      "addAttributesToSVGElement" => {
        if add_attributes_specs.is_empty() {
          eprintln!(
            "Error in plugin \"addAttributesToSVGElement\": absent parameters. addAttributesToSvgElementAttribute or addAttributesToSvgElementAttributes is required."
          );
          continue;
        }
        let specs = std::mem::take(&mut add_attributes_specs);
        plugins.push(Box::new(AddAttributesToSVGElementPlugin::new(
          AddAttributesToSVGElementPluginConfig { attributes: specs },
          &arena,
        )));
      }
      "addClassesToSVGElement" => {
        if add_classes.is_empty() {
          eprintln!(
            "Error in plugin \"addClassesToSVGElement\": absent parameters. addClassesToSvgElementClassName or addClassesToSvgElementClassNames is required."
          );
          continue;
        }
        let class_names = std::mem::take(&mut add_classes);
        plugins.push(Box::new(AddClassesToSVGElementPlugin::new(
          AddClassesToSVGElementPluginConfig { class_names },
          &arena,
        )));
      }
      "cleanupAttrs" => plugins.push(Box::new(CleanupAttrsPlugin::new(
        CleanupAttrsPluginConfig {
          newlines: cleanup_attrs_config.newlines,
          trim: cleanup_attrs_config.trim,
          spaces: cleanup_attrs_config.spaces,
        },
        &arena,
      ))),
      "cleanupNumericValues" => plugins.push(Box::new(CleanupNumericValuesPlugin::new(
        CleanupNumericValuesPluginConfig {
          float_precision: cleanup_numeric_values_config.float_precision,
          leading_zero: cleanup_numeric_values_config.leading_zero,
          default_px: cleanup_numeric_values_config.default_px,
          convert_to_px: cleanup_numeric_values_config.convert_to_px,
        },
        &arena,
      ))),
      "cleanupListOfValues" => plugins.push(Box::new(CleanupListOfValuesPlugin::new(
        CleanupListOfValuesPluginConfig {
          float_precision: cleanup_list_of_values_config.float_precision,
          leading_zero: cleanup_list_of_values_config.leading_zero,
          default_px: cleanup_list_of_values_config.default_px,
          convert_to_px: cleanup_list_of_values_config.convert_to_px,
        },
        &arena,
      ))),
      "convertColors" => plugins.push(Box::new(ConvertColorsPlugin::new(
        ConvertColorsPluginConfig {
          current_color: convert_colors_config.current_color,
          names2hex: convert_colors_config.names2hex,
          rgb2hex: convert_colors_config.rgb2hex,
          convert_case: convert_colors_config.convert_case,
          shorthex: convert_colors_config.shorthex,
          shortname: convert_colors_config.shortname,
        },
        &arena,
      ))),
      "cleanupEnableBackground" => plugins.push(Box::new(CleanupEnableBackgroundPlugin::new(
        CleanupEnableBackgroundPluginConfig {},
        &arena,
      ))),
      "cleanupIds" => plugins.push(Box::new(CleanupIdsPlugin::new(
        CleanupIdsPluginConfig {
          remove: cleanup_ids_config.remove,
          minify: cleanup_ids_config.minify,
          preserve: cleanup_ids_config.preserve.clone(),
          preserve_prefixes: cleanup_ids_config.preserve_prefixes.clone(),
          force: cleanup_ids_config.force,
        },
        &arena,
      ))),
      "collapseGroups" => plugins.push(Box::new(CollapseGroupsPlugin::new(
        CollapseGroupsPluginConfig {},
        &arena,
      ))),
      "convertEllipseToCircle" => plugins.push(Box::new(ConvertEllipseToCirclePlugin::new(
        ConvertEllipseToCirclePluginConfig {},
        &arena,
      ))),
      "convertShapeToPath" => plugins.push(Box::new(ConvertShapeToPathPlugin::new(
        ConvertShapeToPathPluginConfig {
          convert_arcs: convert_shape_to_path_config.convert_arcs,
        },
        &arena,
      ))),
      "convertOneStopGradients" => plugins.push(Box::new(ConvertOneStopGradientsPlugin::new(
        ConvertOneStopGradientsPluginConfig {},
        &arena,
      ))),
      "convertPathData" => plugins.push(Box::new(ConvertPathDataPlugin::new(
        ConvertPathDataPluginConfig {},
        &arena,
      ))),
      "convertStyleToAttrs" => plugins.push(Box::new(ConvertStyleToAttrsPlugin::new(
        ConvertStyleToAttrsPluginConfig {
          keep_important: convert_style_to_attrs_config.keep_important,
        },
        &arena,
      ))),
      "convertTransform" => plugins.push(Box::new(ConvertTransformPlugin::new(
        ConvertTransformPluginConfig {},
        &arena,
      ))),
      "inlineStyles" => plugins.push(Box::new(InlineStylesPlugin::new(
        InlineStylesPluginConfig {},
        &arena,
      ))),
      "mergePaths" => plugins.push(Box::new(MergePathsPlugin::new(
        MergePathsPluginConfig {
          force: merge_paths_config.force,
        },
        &arena,
      ))),
      "mergeStyles" => plugins.push(Box::new(MergeStylesPlugin::new(
        MergeStylesPluginConfig {},
        &arena,
      ))),
      "minifyStyles" => plugins.push(Box::new(MinifyStylesPlugin::new(
        MinifyStylesPluginConfig {},
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
