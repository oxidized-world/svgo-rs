use bumpalo::Bump;
use phf::phf_set;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveNonInheritableGroupAttrsPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
}

pub struct RemoveNonInheritableGroupAttrsPluginConfig {}

static PRESENTATION_ATTRS: phf::Set<&'static str> = phf_set! {
  "alignment-baseline", "baseline-shift", "clip", "clip-path", "clip-rule", "color",
  "color-interpolation", "color-interpolation-filters", "color-profile", "color-rendering",
  "cursor", "direction", "display", "dominant-baseline", "enable-background", "fill",
  "fill-opacity", "fill-rule", "filter", "flood-color", "flood-opacity", "font-family",
  "font-size", "font-size-adjust", "font-stretch", "font-style", "font-variant", "font-weight",
  "glyph-orientation-horizontal", "glyph-orientation-vertical", "image-rendering", "kerning",
  "letter-spacing", "lighting-color", "marker-end", "marker-mid", "marker-start", "mask",
  "opacity", "overflow", "pointer-events", "shape-rendering", "stop-color", "stop-opacity",
  "stroke", "stroke-dasharray", "stroke-dashoffset", "stroke-linecap", "stroke-linejoin",
  "stroke-miterlimit", "stroke-opacity", "stroke-width", "text-anchor", "text-decoration",
  "text-rendering", "unicode-bidi", "visibility", "word-spacing", "writing-mode"
};

static INHERITABLE_ATTRS: phf::Set<&'static str> = phf_set! {
  "color", "cursor", "direction", "fill", "fill-opacity", "fill-rule", "font-family",
  "font-size", "font-stretch", "font-style", "font-variant", "font-weight", "letter-spacing",
  "opacity", "stroke", "stroke-dasharray", "stroke-dashoffset", "stroke-linecap",
  "stroke-linejoin", "stroke-miterlimit", "stroke-opacity", "stroke-width", "text-anchor",
  "text-rendering", "visibility", "word-spacing"
};

static GROUP_EXCEPTIONS: phf::Set<&'static str> =
  phf_set! { "clip-path", "clip-rule", "filter", "mask", "opacity" };

impl<'a> RemoveNonInheritableGroupAttrsPlugin<'a> {
  pub fn new(_config: RemoveNonInheritableGroupAttrsPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    RemoveNonInheritableGroupAttrsPlugin {
      _marker: std::marker::PhantomData,
    }
  }
}

impl<'a> Plugin<'a> for RemoveNonInheritableGroupAttrsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "g" {
      el.attributes.retain(|(name, _)| {
        if !PRESENTATION_ATTRS.contains(name) {
          return true;
        }
        INHERITABLE_ATTRS.contains(name) || GROUP_EXCEPTIONS.contains(name)
      });
    }
    VisitAction::Keep
  }
}
