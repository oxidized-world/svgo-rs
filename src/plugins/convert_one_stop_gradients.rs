use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct ConvertOneStopGradientsPlugin<'a> {
  pub arena: &'a Bump,
  reg_url: Regex,
}

pub struct ConvertOneStopGradientsPluginConfig {}

impl<'a> ConvertOneStopGradientsPlugin<'a> {
  pub fn new(_config: ConvertOneStopGradientsPluginConfig, arena: &'a Bump) -> Self {
    ConvertOneStopGradientsPlugin {
      arena,
      reg_url: Regex::new(r"^url\(#([^\)]+)\)$").unwrap(),
    }
  }

  fn extract_stop_color(stop: &XMLAstElement<'a>) -> Option<String> {
    if let Some((_, v)) = stop.attributes.iter().find(|(k, _)| *k == "stop-color") {
      return Some((*v).to_string());
    }

    if let Some((_, style)) = stop.attributes.iter().find(|(k, _)| *k == "style") {
      for item in style.split(';') {
        if let Some((k, v)) = item.split_once(':') {
          if k.trim().eq_ignore_ascii_case("stop-color") {
            return Some(v.trim().to_string());
          }
        }
      }
    }

    None
  }

  fn collect_gradients(
    children: &[XMLAstChild<'a>],
    out: &mut HashMap<String, String>,
    remove_ids: &mut HashSet<String>,
  ) {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        if (el.name == "linearGradient" || el.name == "radialGradient")
          && el.attributes.iter().any(|(k, _)| *k == "id")
        {
          let stops: Vec<&XMLAstElement<'a>> = el
            .children
            .iter()
            .filter_map(|c| {
              if let XMLAstChild::Element(stop) = c {
                if stop.name == "stop" {
                  return Some(stop);
                }
              }
              None
            })
            .collect();

          if stops.len() == 1 {
            let id = el
              .attributes
              .iter()
              .find(|(k, _)| *k == "id")
              .map(|(_, v)| (*v).to_string())
              .unwrap_or_default();
            if !id.is_empty() {
              if let Some(color) = Self::extract_stop_color(stops[0]) {
                out.insert(id.clone(), color);
                remove_ids.insert(id);
              }
            }
          }
        }

        Self::collect_gradients(&el.children, out, remove_ids);
      }
    }
  }

  fn rewrite_references(&self, children: &mut [XMLAstChild<'a>], colors: &HashMap<String, String>) {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        for (name, value) in &mut el.attributes {
          if *name == "fill" || *name == "stroke" {
            if let Some(cap) = self.reg_url.captures(value) {
              if let Some(id) = cap.get(1).map(|m| m.as_str()) {
                if let Some(color) = colors.get(id) {
                  *value = self.arena.alloc_str(color);
                }
              }
            }
          }
        }
        self.rewrite_references(&mut el.children, colors);
      }
    }
  }

  fn remove_gradients(children: &mut BumpVec<'a, XMLAstChild<'a>>, remove_ids: &HashSet<String>) {
    let mut i = 0usize;
    while i < children.len() {
      let mut removed = false;
      if let XMLAstChild::Element(el) = &mut children[i] {
        let is_target_gradient = (el.name == "linearGradient" || el.name == "radialGradient")
          && el
            .attributes
            .iter()
            .find(|(k, _)| *k == "id")
            .map(|(_, v)| remove_ids.contains(*v))
            .unwrap_or(false);
        if is_target_gradient {
          children.remove(i);
          removed = true;
        } else {
          Self::remove_gradients(&mut el.children, remove_ids);
          if el.name == "defs" && el.children.is_empty() {
            children.remove(i);
            removed = true;
          }
        }
      }
      if !removed {
        i += 1;
      }
    }
  }
}

impl<'a> Plugin<'a> for ConvertOneStopGradientsPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    let mut colors = HashMap::new();
    let mut remove_ids = HashSet::new();

    Self::collect_gradients(&root.children, &mut colors, &mut remove_ids);
    if colors.is_empty() {
      return;
    }

    self.rewrite_references(&mut root.children, &colors);
    Self::remove_gradients(&mut root.children, &remove_ids);
  }
}
