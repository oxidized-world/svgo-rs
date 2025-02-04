use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
  static ref ELEM_GROUPS: HashMap<&'static str, HashSet<&'static str>> = {
    let data: &[(&str, &[&str])] = &[
      (
        "animation",
        &[
          "animate",
          "animateColor",
          "animateMotion",
          "animateTransform",
          "set",
        ],
      ),
      ("descriptive", &["desc", "metadata", "title"]),
      (
        "shape",
        &[
          "circle", "ellipse", "line", "path", "polygon", "polyline", "rect",
        ],
      ),
      ("structural", &["defs", "g", "svg", "symbol", "use"]),
      (
        "paintServer",
        &[
          "hatch",
          "linearGradient",
          "meshGradient",
          "pattern",
          "radialGradient",
          "solidColor",
        ],
      ),
      (
        "nonRendering",
        &[
          "clipPath",
          "filter",
          "linearGradient",
          "marker",
          "mask",
          "pattern",
          "radialGradient",
          "solidColor",
          "symbol",
        ],
      ),
      (
        "container",
        &[
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
        ],
      ),
      (
        "textContent",
        &[
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
        ],
      ),
      (
        "textContentChild",
        &["altGlyph", "textPath", "tref", "tspan"],
      ),
      (
        "lightSource",
        &[
          "feDiffuseLighting",
          "feDistantLight",
          "fePointLight",
          "feSpecularLighting",
          "feSpotLight",
        ],
      ),
      (
        "filterPrimitive",
        &[
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
        ],
      ),
    ];
    data
      .iter()
      .map(|(key, values)| (*key, values.iter().cloned().collect::<HashSet<_>>()))
      .collect()
  };
}

pub struct Group {
  pub r#unsafe: Option<HashSet<&'static str>>,
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
