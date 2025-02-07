use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

pub struct Group {
  pub r#unsafe: Option<HashSet<&'static str>>,
}

pub struct ElemsGroups {
  pub animation: HashSet<&'static str>,
  pub descriptive: HashSet<&'static str>,
  pub shape: HashSet<&'static str>,
  pub structural: HashSet<&'static str>,
  pub paint_server: HashSet<&'static str>,
  pub non_rendering: HashSet<&'static str>,
  pub container: HashSet<&'static str>,
  pub text_content: HashSet<&'static str>,
  pub text_content_child: HashSet<&'static str>,
  pub light_source: HashSet<&'static str>,
  pub filter_primitive: HashSet<&'static str>,
}

lazy_static! {
  pub static ref ELEMS_GROUPS: ElemsGroups = ElemsGroups {
    animation: vec![
      "animate",
      "animateColor",
      "animateMotion",
      "animateTransform",
      "set",
    ]
    .into_iter()
    .collect(),
    descriptive: vec!["desc", "metadata", "title"].into_iter().collect(),
    shape: vec!["circle", "ellipse", "line", "path", "polygon", "polyline", "rect",]
      .into_iter()
      .collect(),
    structural: vec!["defs", "g", "svg", "symbol", "use"]
      .into_iter()
      .collect(),
    paint_server: vec![
      "hatch",
      "linearGradient",
      "meshGradient",
      "pattern",
      "radialGradient",
      "solidColor",
    ]
    .into_iter()
    .collect(),
    non_rendering: vec![
      "clipPath",
      "filter",
      "linearGradient",
      "marker",
      "mask",
      "pattern",
      "radialGradient",
      "solidColor",
      "symbol",
    ]
    .into_iter()
    .collect(),
    container: vec![
      "a",
      "defs",
      "foreignObject",
      "g",
      "marker",
      "mask",
      "missing-glyph",
      "pattern",
      "svg",
      "switch",
      "symbol",
    ]
    .into_iter()
    .collect(),
    text_content: vec![
      "a",
      "altGlyph",
      "altGlyphDef",
      "altGlyphItem",
      "glyph",
      "glyphRef",
      "text",
      "textPath",
      "tref",
      "tspan",
    ]
    .into_iter()
    .collect(),
    text_content_child: vec!["altGlyph", "textPath", "tref", "tspan"]
      .into_iter()
      .collect(),
    light_source: vec![
      "feDiffuseLighting",
      "feDistantLight",
      "fePointLight",
      "feSpecularLighting",
      "feSpotLight",
    ]
    .into_iter()
    .collect(),
    filter_primitive: vec![
      "feBlend",
      "feColorMatrix",
      "feComponentTransfer",
      "feComposite",
      "feConvolveMatrix",
      "feDiffuseLighting",
      "feDisplacementMap",
      "feDropShadow",
      "feFlood",
      "feFuncA",
      "feFuncB",
      "feFuncG",
      "feFuncR",
      "feGaussianBlur",
      "feImage",
      "feMerge",
      "feMergeNode",
      "feMorphology",
      "feOffset",
      "feSpecularLighting",
      "feTile",
      "feTurbulence",
    ]
    .into_iter()
    .collect(),
  };
}

lazy_static! {
  pub static ref ATTRS_GROUPS_DEPRECATED: HashMap<&'static str, Group> = {
    let mut map = HashMap::new();

    map.insert(
      "animationAttributeTarget",
      Group {
        r#unsafe: Some(vec!["attributeType"].into_iter().collect()),
      },
    );

    map.insert(
      "conditionalProcessing",
      Group {
        r#unsafe: Some(vec!["requiredFeatures"].into_iter().collect()),
      },
    );

    map.insert(
      "core",
      Group {
        r#unsafe: Some(
          vec!["xml:base", "xml:lang", "xml:space"]
            .into_iter()
            .collect(),
        ),
      },
    );

    map.insert(
      "presentation",
      Group {
        r#unsafe: Some(
          vec![
            "clip",
            "color-profile",
            "enable-background",
            "glyph-orientation-horizontal",
            "glyph-orientation-vertical",
            "kerning",
          ]
          .into_iter()
          .collect(),
        ),
      },
    );

    map
  };
}
