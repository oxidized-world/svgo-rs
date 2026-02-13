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
use plugins::move_group_attrs_to_elems::{
  MoveGroupAttrsToElemsPlugin, MoveGroupAttrsToElemsPluginConfig,
};
use plugins::prefix_ids::{PrefixIdsPlugin, PrefixIdsPluginConfig};
use plugins::remove_attributes_by_selector::{
  RemoveAttributesBySelectorPlugin, RemoveAttributesBySelectorPluginConfig,
};
use plugins::remove_attrs::{RemoveAttrsPlugin, RemoveAttrsPluginConfig};
use plugins::remove_comments::{RemoveCommentsConfig, RemoveCommentsPlugin};
use plugins::remove_deprecated_attrs::{
  RemoveDeprecatedAttrsPlugin, RemoveDeprecatedAttrsPluginConfig,
};
use plugins::remove_desc::{RemoveDescPlugin, RemoveDescPluginConfig};
use plugins::remove_dimensions::{RemoveDimensionsPlugin, RemoveDimensionsPluginConfig};
use plugins::remove_doctype::{RemoveDoctypePlugin, RemoveDoctypePluginConfig};
use plugins::remove_editors_ns_data::{RemoveEditorsNSData, RemoveEditorsNSDataConfig};
use plugins::remove_elements_by_attr::{
  RemoveElementsByAttrPlugin, RemoveElementsByAttrPluginConfig,
};
use plugins::remove_empty_attrs::{RemoveEmptyAttrsPlugin, RemoveEmptyAttrsPluginConfig};
use plugins::remove_empty_containers::{
  RemoveEmptyContainersPlugin, RemoveEmptyContainersPluginConfig,
};
use plugins::remove_empty_text::{RemoveEmptyTextPlugin, RemoveEmptyTextPluginConfig};
use plugins::remove_hidden_elems::{RemoveHiddenElemsPlugin, RemoveHiddenElemsPluginConfig};
use plugins::remove_metadata::{RemoveMetadataPlugin, RemoveMetadataPluginConfig};
use plugins::remove_non_inheritable_group_attrs::{
  RemoveNonInheritableGroupAttrsPlugin, RemoveNonInheritableGroupAttrsPluginConfig,
};
use plugins::remove_off_canvas_paths::{
  RemoveOffCanvasPathsPlugin, RemoveOffCanvasPathsPluginConfig,
};
use plugins::remove_raster_images::{RemoveRasterImagesPlugin, RemoveRasterImagesPluginConfig};
use plugins::remove_scripts::{RemoveScriptsPlugin, RemoveScriptsPluginConfig};
use plugins::remove_style_element::{RemoveStyleElementPlugin, RemoveStyleElementPluginConfig};
use plugins::remove_title::{RemoveTitlePlugin, RemoveTitlePluginConfig};
use plugins::remove_unknowns_and_defaults::{
  RemoveUnknownsAndDefaultsPlugin, RemoveUnknownsAndDefaultsPluginConfig,
};
use plugins::remove_unused_ns::{RemoveUnusedNSPlugin, RemoveUnusedNSPluginConfig};
use plugins::remove_useless_defs::{RemoveUselessDefsPlugin, RemoveUselessDefsPluginConfig};
use plugins::remove_useless_stroke_and_fill::{
  RemoveUselessStrokeAndFillPlugin, RemoveUselessStrokeAndFillPluginConfig,
};
use plugins::remove_view_box::{RemoveViewBoxPlugin, RemoveViewBoxPluginConfig};
use plugins::remove_xlink::{RemoveXlinkPlugin, RemoveXlinkPluginConfig};
use plugins::remove_xml_proc_inst::{RemoveXMLProcInstPlugin, RemoveXMLProcInstPluginConfig};
use plugins::remove_xmlns::{RemoveXMLNSPlugin, RemoveXMLNSPluginConfig};
use plugins::reuse_paths::{ReusePathsPlugin, ReusePathsPluginConfig};
use plugins::sort_attrs::{SortAttrsPlugin, SortAttrsPluginConfig};
use plugins::sort_defs_children::{SortDefsChildrenPlugin, SortDefsChildrenPluginConfig};
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

  /// prefixIds
  pub prefix_ids_prefix: Option<String>,
  pub prefix_ids_delim: Option<String>,
  pub prefix_ids_prefix_ids: Option<bool>,
  pub prefix_ids_prefix_class_names: Option<bool>,

  /// removeAttributesBySelector
  pub remove_attributes_by_selector_selector: Option<String>,
  pub remove_attributes_by_selector_attributes: Option<Vec<String>>,

  /// removeAttrs
  pub remove_attrs_attrs: Option<Vec<String>>,
  pub remove_attrs_elem_separator: Option<String>,
  pub remove_attrs_preserve_current_color: Option<bool>,

  /// removeDeprecatedAttrs
  pub remove_deprecated_attrs_remove_unsafe: Option<bool>,

  /// removeElementsByAttr
  pub remove_elements_by_attr_id: Option<Vec<String>>,
  pub remove_elements_by_attr_class: Option<Vec<String>>,

  /// removeEmptyText
  pub remove_empty_text_text: Option<bool>,
  pub remove_empty_text_tspan: Option<bool>,
  pub remove_empty_text_tref: Option<bool>,

  /// removeHiddenElems
  pub remove_hidden_elems_is_hidden: Option<bool>,
  pub remove_hidden_elems_display_none: Option<bool>,
  pub remove_hidden_elems_opacity0: Option<bool>,
  pub remove_hidden_elems_circle_r0: Option<bool>,
  pub remove_hidden_elems_ellipse_rx0: Option<bool>,
  pub remove_hidden_elems_ellipse_ry0: Option<bool>,
  pub remove_hidden_elems_rect_width0: Option<bool>,
  pub remove_hidden_elems_rect_height0: Option<bool>,
  pub remove_hidden_elems_pattern_width0: Option<bool>,
  pub remove_hidden_elems_pattern_height0: Option<bool>,
  pub remove_hidden_elems_image_width0: Option<bool>,
  pub remove_hidden_elems_image_height0: Option<bool>,
  pub remove_hidden_elems_path_empty_d: Option<bool>,
  pub remove_hidden_elems_polyline_empty_points: Option<bool>,
  pub remove_hidden_elems_polygon_empty_points: Option<bool>,

  /// removeUnknownsAndDefaults
  pub remove_unknowns_and_defaults_unknown_content: Option<bool>,
  pub remove_unknowns_and_defaults_unknown_attrs: Option<bool>,
  pub remove_unknowns_and_defaults_default_attrs: Option<bool>,
  pub remove_unknowns_and_defaults_default_markup_declarations: Option<bool>,
  pub remove_unknowns_and_defaults_useless_overrides: Option<bool>,
  pub remove_unknowns_and_defaults_keep_data_attrs: Option<bool>,
  pub remove_unknowns_and_defaults_keep_aria_attrs: Option<bool>,
  pub remove_unknowns_and_defaults_keep_role_attr: Option<bool>,

  /// removeUselessStrokeAndFill
  pub remove_useless_stroke_and_fill_stroke: Option<bool>,
  pub remove_useless_stroke_and_fill_fill: Option<bool>,
  pub remove_useless_stroke_and_fill_remove_none: Option<bool>,

  /// removeXlink
  pub remove_xlink_include_legacy: Option<bool>,

  /// sortAttrs
  pub sort_attrs_order: Option<Vec<String>>,
  pub sort_attrs_xmlns_order: Option<String>,
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

  let prefix_ids_prefix = options
    .prefix_ids_prefix
    .as_ref()
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|s| {
      let allocated: &mut str = arena.alloc_str(s);
      &*allocated
    });
  let prefix_ids_delim = options
    .prefix_ids_delim
    .as_ref()
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .unwrap_or("__");
  let prefix_ids_delim = {
    let allocated: &mut str = arena.alloc_str(prefix_ids_delim);
    &*allocated
  };
  let prefix_ids_config = PrefixIdsPluginConfig {
    prefix: prefix_ids_prefix,
    delim: prefix_ids_delim,
    prefix_ids: options.prefix_ids_prefix_ids.unwrap_or(true),
    prefix_class_names: options.prefix_ids_prefix_class_names.unwrap_or(true),
  };

  let remove_attributes_by_selector_selector = options
    .remove_attributes_by_selector_selector
    .as_ref()
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|s| {
      let allocated: &mut str = arena.alloc_str(s);
      &*allocated
    });
  let remove_attributes_by_selector_attributes = options
    .remove_attributes_by_selector_attributes
    .unwrap_or_default()
    .into_iter()
    .map(|s| {
      let allocated: &mut str = arena.alloc_str(s.trim());
      &*allocated
    })
    .filter(|s| !s.is_empty())
    .collect::<Vec<&str>>();
  let remove_attributes_by_selector_config = RemoveAttributesBySelectorPluginConfig {
    selector: remove_attributes_by_selector_selector,
    attributes: remove_attributes_by_selector_attributes,
  };

  let remove_attrs_attrs = options
    .remove_attrs_attrs
    .unwrap_or_default()
    .into_iter()
    .map(|s| {
      let allocated: &mut str = arena.alloc_str(s.trim());
      &*allocated
    })
    .filter(|s| !s.is_empty())
    .collect::<Vec<&str>>();
  let remove_attrs_elem_separator = options
    .remove_attrs_elem_separator
    .as_ref()
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .unwrap_or(":");
  let remove_attrs_elem_separator = {
    let allocated: &mut str = arena.alloc_str(remove_attrs_elem_separator);
    &*allocated
  };
  let remove_attrs_config = RemoveAttrsPluginConfig {
    attrs: remove_attrs_attrs,
    elem_separator: remove_attrs_elem_separator,
    preserve_current_color: options.remove_attrs_preserve_current_color.unwrap_or(false),
  };

  let remove_deprecated_attrs_config = RemoveDeprecatedAttrsPluginConfig {
    remove_unsafe: options.remove_deprecated_attrs_remove_unsafe.unwrap_or(false),
  };

  let remove_elements_by_attr_ids = options
    .remove_elements_by_attr_id
    .unwrap_or_default()
    .into_iter()
    .map(|s| {
      let allocated: &mut str = arena.alloc_str(s.trim());
      &*allocated
    })
    .filter(|s| !s.is_empty())
    .collect::<Vec<&str>>();
  let remove_elements_by_attr_class = options
    .remove_elements_by_attr_class
    .unwrap_or_default()
    .into_iter()
    .map(|s| {
      let allocated: &mut str = arena.alloc_str(s.trim());
      &*allocated
    })
    .filter(|s| !s.is_empty())
    .collect::<Vec<&str>>();
  let remove_elements_by_attr_config = RemoveElementsByAttrPluginConfig {
    ids: remove_elements_by_attr_ids,
    classes: remove_elements_by_attr_class,
  };

  let remove_empty_text_config = RemoveEmptyTextPluginConfig {
    text: options.remove_empty_text_text.unwrap_or(true),
    tspan: options.remove_empty_text_tspan.unwrap_or(true),
    tref: options.remove_empty_text_tref.unwrap_or(true),
  };

  let remove_hidden_elems_config = RemoveHiddenElemsPluginConfig {
    is_hidden: options.remove_hidden_elems_is_hidden.unwrap_or(true),
    display_none: options.remove_hidden_elems_display_none.unwrap_or(true),
    opacity0: options.remove_hidden_elems_opacity0.unwrap_or(true),
    circle_r0: options.remove_hidden_elems_circle_r0.unwrap_or(true),
    ellipse_rx0: options.remove_hidden_elems_ellipse_rx0.unwrap_or(true),
    ellipse_ry0: options.remove_hidden_elems_ellipse_ry0.unwrap_or(true),
    rect_width0: options.remove_hidden_elems_rect_width0.unwrap_or(true),
    rect_height0: options.remove_hidden_elems_rect_height0.unwrap_or(true),
    pattern_width0: options.remove_hidden_elems_pattern_width0.unwrap_or(true),
    pattern_height0: options.remove_hidden_elems_pattern_height0.unwrap_or(true),
    image_width0: options.remove_hidden_elems_image_width0.unwrap_or(true),
    image_height0: options.remove_hidden_elems_image_height0.unwrap_or(true),
    path_empty_d: options.remove_hidden_elems_path_empty_d.unwrap_or(true),
    polyline_empty_points: options.remove_hidden_elems_polyline_empty_points.unwrap_or(true),
    polygon_empty_points: options.remove_hidden_elems_polygon_empty_points.unwrap_or(true),
  };

  let remove_unknowns_and_defaults_config = RemoveUnknownsAndDefaultsPluginConfig {
    unknown_content: options.remove_unknowns_and_defaults_unknown_content.unwrap_or(true),
    unknown_attrs: options.remove_unknowns_and_defaults_unknown_attrs.unwrap_or(true),
    default_attrs: options.remove_unknowns_and_defaults_default_attrs.unwrap_or(true),
    default_markup_declarations: options
      .remove_unknowns_and_defaults_default_markup_declarations
      .unwrap_or(true),
    useless_overrides: options.remove_unknowns_and_defaults_useless_overrides.unwrap_or(true),
    keep_data_attrs: options.remove_unknowns_and_defaults_keep_data_attrs.unwrap_or(true),
    keep_aria_attrs: options.remove_unknowns_and_defaults_keep_aria_attrs.unwrap_or(true),
    keep_role_attr: options.remove_unknowns_and_defaults_keep_role_attr.unwrap_or(false),
  };

  let remove_useless_stroke_and_fill_config = RemoveUselessStrokeAndFillPluginConfig {
    stroke: options.remove_useless_stroke_and_fill_stroke.unwrap_or(true),
    fill: options.remove_useless_stroke_and_fill_fill.unwrap_or(true),
    remove_none: options.remove_useless_stroke_and_fill_remove_none.unwrap_or(false),
  };

  let remove_xlink_config = RemoveXlinkPluginConfig {
    include_legacy: options.remove_xlink_include_legacy.unwrap_or(false),
  };

  let sort_attrs_default_order = vec![
    "id", "width", "height", "x", "x1", "x2", "y", "y1", "y2", "cx", "cy", "r", "fill", "stroke",
    "marker", "d", "points",
  ];
  let sort_attrs_order = options
    .sort_attrs_order
    .unwrap_or(sort_attrs_default_order.into_iter().map(String::from).collect())
    .into_iter()
    .map(|s| {
      let allocated: &mut str = arena.alloc_str(s.trim());
      &*allocated
    })
    .filter(|s| !s.is_empty())
    .collect::<Vec<&str>>();
  let sort_attrs_xmlns_order = options
    .sort_attrs_xmlns_order
    .as_ref()
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .unwrap_or("front");
  let sort_attrs_xmlns_order = {
    let allocated: &mut str = arena.alloc_str(sort_attrs_xmlns_order);
    &*allocated
  };
  let sort_attrs_config = SortAttrsPluginConfig {
    order: sort_attrs_order,
    xmlns_order: sort_attrs_xmlns_order,
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
      "moveGroupAttrsToElems" => plugins.push(Box::new(MoveGroupAttrsToElemsPlugin::new(
        MoveGroupAttrsToElemsPluginConfig {},
        &arena,
      ))),
      "prefixIds" => plugins.push(Box::new(PrefixIdsPlugin::new(
        PrefixIdsPluginConfig {
          prefix: prefix_ids_config.prefix,
          delim: prefix_ids_config.delim,
          prefix_ids: prefix_ids_config.prefix_ids,
          prefix_class_names: prefix_ids_config.prefix_class_names,
        },
        &arena,
      ))),
      "removeAttributesBySelector" => {
        plugins.push(Box::new(RemoveAttributesBySelectorPlugin::new(
          RemoveAttributesBySelectorPluginConfig {
            selector: remove_attributes_by_selector_config.selector,
            attributes: remove_attributes_by_selector_config.attributes.clone(),
          },
          &arena,
        )))
      }
      "removeAttrs" => plugins.push(Box::new(RemoveAttrsPlugin::new(
        RemoveAttrsPluginConfig {
          attrs: remove_attrs_config.attrs.clone(),
          elem_separator: remove_attrs_config.elem_separator,
          preserve_current_color: remove_attrs_config.preserve_current_color,
        },
        &arena,
      ))),
      "removeEditorsNSData" => plugins.push(Box::new(RemoveEditorsNSData::new(
        RemoveEditorsNSDataConfig {
          additional_namespace: remove_editors_additional.clone(),
        },
        &arena,
      ))),
      "removeDeprecatedAttrs" => plugins.push(Box::new(RemoveDeprecatedAttrsPlugin::new(
        RemoveDeprecatedAttrsPluginConfig {
          remove_unsafe: remove_deprecated_attrs_config.remove_unsafe,
        },
        &arena,
      ))),
      "removeTitle" => plugins.push(Box::new(RemoveTitlePlugin::new(
        RemoveTitlePluginConfig {},
        &arena,
      ))),
      "removeDimensions" => plugins.push(Box::new(RemoveDimensionsPlugin::new(
        RemoveDimensionsPluginConfig {},
        &arena,
      ))),
      "removeElementsByAttr" => plugins.push(Box::new(RemoveElementsByAttrPlugin::new(
        RemoveElementsByAttrPluginConfig {
          ids: remove_elements_by_attr_config.ids.clone(),
          classes: remove_elements_by_attr_config.classes.clone(),
        },
        &arena,
      ))),
      "removeEmptyAttrs" => plugins.push(Box::new(RemoveEmptyAttrsPlugin::new(
        RemoveEmptyAttrsPluginConfig {},
        &arena,
      ))),
      "removeEmptyContainers" => plugins.push(Box::new(RemoveEmptyContainersPlugin::new(
        RemoveEmptyContainersPluginConfig {},
        &arena,
      ))),
      "removeEmptyText" => plugins.push(Box::new(RemoveEmptyTextPlugin::new(
        RemoveEmptyTextPluginConfig {
          text: remove_empty_text_config.text,
          tspan: remove_empty_text_config.tspan,
          tref: remove_empty_text_config.tref,
        },
        &arena,
      ))),
      "removeHiddenElems" => plugins.push(Box::new(RemoveHiddenElemsPlugin::new(
        RemoveHiddenElemsPluginConfig {
          is_hidden: remove_hidden_elems_config.is_hidden,
          display_none: remove_hidden_elems_config.display_none,
          opacity0: remove_hidden_elems_config.opacity0,
          circle_r0: remove_hidden_elems_config.circle_r0,
          ellipse_rx0: remove_hidden_elems_config.ellipse_rx0,
          ellipse_ry0: remove_hidden_elems_config.ellipse_ry0,
          rect_width0: remove_hidden_elems_config.rect_width0,
          rect_height0: remove_hidden_elems_config.rect_height0,
          pattern_width0: remove_hidden_elems_config.pattern_width0,
          pattern_height0: remove_hidden_elems_config.pattern_height0,
          image_width0: remove_hidden_elems_config.image_width0,
          image_height0: remove_hidden_elems_config.image_height0,
          path_empty_d: remove_hidden_elems_config.path_empty_d,
          polyline_empty_points: remove_hidden_elems_config.polyline_empty_points,
          polygon_empty_points: remove_hidden_elems_config.polygon_empty_points,
        },
        &arena,
      ))),
      "removeNonInheritableGroupAttrs" => {
        plugins.push(Box::new(RemoveNonInheritableGroupAttrsPlugin::new(
          RemoveNonInheritableGroupAttrsPluginConfig {},
          &arena,
        )))
      }
      "removeOffCanvasPaths" => plugins.push(Box::new(RemoveOffCanvasPathsPlugin::new(
        RemoveOffCanvasPathsPluginConfig {},
        &arena,
      ))),
      "removeRasterImages" => plugins.push(Box::new(RemoveRasterImagesPlugin::new(
        RemoveRasterImagesPluginConfig {},
        &arena,
      ))),
      "removeScripts" => plugins.push(Box::new(RemoveScriptsPlugin::new(
        RemoveScriptsPluginConfig {},
        &arena,
      ))),
      "removeStyleElement" => plugins.push(Box::new(RemoveStyleElementPlugin::new(
        RemoveStyleElementPluginConfig {},
        &arena,
      ))),
      "removeUnknownsAndDefaults" => plugins.push(Box::new(RemoveUnknownsAndDefaultsPlugin::new(
        RemoveUnknownsAndDefaultsPluginConfig {
          unknown_content: remove_unknowns_and_defaults_config.unknown_content,
          unknown_attrs: remove_unknowns_and_defaults_config.unknown_attrs,
          default_attrs: remove_unknowns_and_defaults_config.default_attrs,
          default_markup_declarations: remove_unknowns_and_defaults_config
            .default_markup_declarations,
          useless_overrides: remove_unknowns_and_defaults_config.useless_overrides,
          keep_data_attrs: remove_unknowns_and_defaults_config.keep_data_attrs,
          keep_aria_attrs: remove_unknowns_and_defaults_config.keep_aria_attrs,
          keep_role_attr: remove_unknowns_and_defaults_config.keep_role_attr,
        },
        &arena,
      ))),
      "removeUnusedNS" => plugins.push(Box::new(RemoveUnusedNSPlugin::new(
        RemoveUnusedNSPluginConfig {},
        &arena,
      ))),
      "removeUselessDefs" => plugins.push(Box::new(RemoveUselessDefsPlugin::new(
        RemoveUselessDefsPluginConfig {},
        &arena,
      ))),
      "removeUselessStrokeAndFill" => {
        plugins.push(Box::new(RemoveUselessStrokeAndFillPlugin::new(
          RemoveUselessStrokeAndFillPluginConfig {
            stroke: remove_useless_stroke_and_fill_config.stroke,
            fill: remove_useless_stroke_and_fill_config.fill,
            remove_none: remove_useless_stroke_and_fill_config.remove_none,
          },
          &arena,
        )))
      }
      "removeViewBox" => plugins.push(Box::new(RemoveViewBoxPlugin::new(
        RemoveViewBoxPluginConfig {},
        &arena,
      ))),
      "removeXMLNS" => plugins.push(Box::new(RemoveXMLNSPlugin::new(
        RemoveXMLNSPluginConfig {},
        &arena,
      ))),
      "removeXlink" => plugins.push(Box::new(RemoveXlinkPlugin::new(
        RemoveXlinkPluginConfig {
          include_legacy: remove_xlink_config.include_legacy,
        },
        &arena,
      ))),
      "reusePaths" => plugins.push(Box::new(ReusePathsPlugin::new(
        ReusePathsPluginConfig {},
        &arena,
      ))),
      "sortAttrs" => plugins.push(Box::new(SortAttrsPlugin::new(
        SortAttrsPluginConfig {
          order: sort_attrs_config.order.clone(),
          xmlns_order: sort_attrs_config.xmlns_order,
        },
        &arena,
      ))),
      "sortDefsChildren" => plugins.push(Box::new(SortDefsChildrenPlugin::new(
        SortDefsChildrenPluginConfig {},
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
