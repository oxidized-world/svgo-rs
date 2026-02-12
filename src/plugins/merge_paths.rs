use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use regex::Regex;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct MergePathsPlugin<'a> {
  pub arena: &'a Bump,
  pub force: bool,
}

pub struct MergePathsPluginConfig {
  pub force: bool,
}

impl<'a> MergePathsPlugin<'a> {
  pub fn new(config: MergePathsPluginConfig, arena: &'a Bump) -> Self {
    MergePathsPlugin {
      arena,
      force: config.force,
    }
  }

  fn process_children(&self, children: &mut BumpVec<'a, XMLAstChild<'a>>) {
    let reg_moveto_lineto = Regex::new(
      r"M\s*([-+]?\d*\.?\d+(?:[eE][-+]?\d+)?)\s+([-+]?\d*\.?\d+(?:[eE][-+]?\d+)?)\s*L\s*",
    )
    .unwrap();

    for child in children.iter_mut() {
      if let XMLAstChild::Element(el) = child {
        self.process_children(&mut el.children);
      }
    }

    let mut i = 0usize;
    while i + 1 < children.len() {
      let can_merge = {
        let left = children.get(i);
        let right = children.get(i + 1);
        match (left, right) {
          (Some(XMLAstChild::Element(a)), Some(XMLAstChild::Element(b))) => self.can_merge(a, b),
          _ => false,
        }
      };

      if !can_merge {
        i += 1;
        continue;
      }

      let right_d = match children.get(i + 1) {
        Some(XMLAstChild::Element(b)) => b
          .attributes
          .iter()
          .find(|(k, _)| *k == "d")
          .map(|(_, v)| (*v).to_string())
          .unwrap_or_default(),
        _ => String::new(),
      };

      if let Some(XMLAstChild::Element(a)) = children.get_mut(i) {
        if let Some((_, a_d)) = a.attributes.iter_mut().find(|(k, _)| *k == "d") {
          let mut merged = (*a_d).to_string();
          if !merged.is_empty() && !right_d.is_empty() {
            let need_space = merged
              .chars()
              .last()
              .map(|ch| ch.is_ascii_digit() || ch == '.')
              .unwrap_or(false)
              && right_d
                .chars()
                .next()
                .map(|ch| ch.is_ascii_digit() || ch == '.')
                .unwrap_or(false);
            if need_space {
              merged.push(' ');
            }
          }
          merged.push_str(&right_d);
          merged = reg_moveto_lineto.replace_all(&merged, "M$1 $2 ").to_string();
          *a_d = self.arena.alloc_str(&merged);
        }
      }

      children.remove(i + 1);
    }
  }

  fn can_merge(&self, a: &XMLAstElement<'a>, b: &XMLAstElement<'a>) -> bool {
    if a.name != "path" || b.name != "path" {
      return false;
    }

    let has_url_or_filter = |el: &XMLAstElement<'a>| {
      el.attributes
        .iter()
        .any(|(k, v)| *k == "filter" || *k == "mask" || v.contains("url(#") || *k == "id")
    };

    if !self.force && (has_url_or_filter(a) || has_url_or_filter(b)) {
      return false;
    }

    let normalize_attrs = |el: &XMLAstElement<'a>| {
      let mut attrs: Vec<(String, String)> = el
        .attributes
        .iter()
        .filter(|(k, _)| *k != "d")
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect();
      attrs.sort_by(|x, y| x.0.cmp(&y.0));
      attrs
    };

    normalize_attrs(a) == normalize_attrs(b)
      && a.attributes.iter().any(|(k, _)| *k == "d")
      && b.attributes.iter().any(|(k, _)| *k == "d")
  }
}

impl<'a> Plugin<'a> for MergePathsPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    self.process_children(&mut root.children);
  }
}
