use bumpalo::Bump;
use phf::{phf_map, phf_set};

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct RemoveUnknownsAndDefaultsPlugin<'a> {
  _marker: std::marker::PhantomData<&'a Bump>,
  pub unknown_content: bool,
  pub unknown_attrs: bool,
  pub default_attrs: bool,
  pub keep_data_attrs: bool,
  pub keep_aria_attrs: bool,
  pub keep_role_attr: bool,
}

pub struct RemoveUnknownsAndDefaultsPluginConfig {
  pub unknown_content: bool,
  pub unknown_attrs: bool,
  pub default_attrs: bool,
  pub default_markup_declarations: bool,
  pub useless_overrides: bool,
  pub keep_data_attrs: bool,
  pub keep_aria_attrs: bool,
  pub keep_role_attr: bool,
}

static KNOWN_ELEMS: phf::Set<&'static str> = phf_set! {
  "svg","g","defs","symbol","use","path","rect","circle","ellipse","line","polyline","polygon",
  "clipPath","mask","pattern","linearGradient","radialGradient","stop","text","tspan","tref","image",
  "a","title","desc","metadata","style","filter","feGaussianBlur","feColorMatrix","marker"
};

static COMMON_ATTRS: phf::Set<&'static str> = phf_set! {
  "id","class","style","transform","x","y","x1","y1","x2","y2","width","height","cx","cy","r","rx","ry",
  "d","points","fill","fill-opacity","fill-rule","stroke","stroke-width","stroke-opacity","stroke-linecap",
  "stroke-linejoin","stroke-dasharray","stroke-dashoffset","opacity","display","visibility","viewBox","preserveAspectRatio",
  "href","xlink:href","xmlns","xmlns:xlink","version","baseProfile","enable-background","filter","clip-path","mask"
};

static DEFAULT_ATTRS: phf::Map<&'static str, &'static str> = phf_map! {
  "stroke" => "none",
  "stroke-opacity" => "1",
  "stroke-width" => "1",
  "fill-opacity" => "1",
  "opacity" => "1",
};

impl<'a> RemoveUnknownsAndDefaultsPlugin<'a> {
  pub fn new(config: RemoveUnknownsAndDefaultsPluginConfig, arena: &'a Bump) -> Self {
    let _ = arena;
    let _ = config.default_markup_declarations;
    let _ = config.useless_overrides;
    RemoveUnknownsAndDefaultsPlugin {
      _marker: std::marker::PhantomData,
      unknown_content: config.unknown_content,
      unknown_attrs: config.unknown_attrs,
      default_attrs: config.default_attrs,
      keep_data_attrs: config.keep_data_attrs,
      keep_aria_attrs: config.keep_aria_attrs,
      keep_role_attr: config.keep_role_attr,
    }
  }
}

impl<'a> Plugin<'a> for RemoveUnknownsAndDefaultsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if self.unknown_content && !KNOWN_ELEMS.contains(el.name) && !el.name.contains(':') {
      return VisitAction::Remove;
    }

    el.attributes.retain(|(name, value)| {
      if self.keep_data_attrs && name.starts_with("data-") {
        return true;
      }
      if self.keep_aria_attrs && name.starts_with("aria-") {
        return true;
      }
      if self.keep_role_attr && *name == "role" {
        return true;
      }
      if *name == "xmlns" {
        return true;
      }

      if self.unknown_attrs && !COMMON_ATTRS.contains(name) {
        return false;
      }

      if self.default_attrs {
        if let Some(default) = DEFAULT_ATTRS.get(name) {
          if *value == *default {
            return false;
          }
        }
      }

      true
    });

    VisitAction::Keep
  }
}
