use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::XMLAstElement;

pub struct SortAttrsPlugin<'a> {
  pub arena: &'a Bump,
  pub order: Vec<&'a str>,
  pub xmlns_order: &'a str,
}

pub struct SortAttrsPluginConfig<'a> {
  pub order: Vec<&'a str>,
  pub xmlns_order: &'a str,
}

impl<'a> SortAttrsPlugin<'a> {
  pub fn new(config: SortAttrsPluginConfig<'a>, arena: &'a Bump) -> Self {
    SortAttrsPlugin {
      arena,
      order: config.order,
      xmlns_order: config.xmlns_order,
    }
  }

  fn ns_priority(&self, name: &str) -> i32 {
    if self.xmlns_order == "front" {
      if name == "xmlns" {
        return 3;
      }
      if name.starts_with("xmlns:") {
        return 2;
      }
    }
    if name.contains(':') {
      return 1;
    }
    0
  }
}

impl<'a> Plugin<'a> for SortAttrsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    let mut attrs = el
      .attributes
      .iter()
      .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
      .collect::<Vec<(String, String)>>();

    attrs.sort_by(|(a_name, _), (b_name, _)| {
      let a_prio = self.ns_priority(a_name);
      let b_prio = self.ns_priority(b_name);
      if a_prio != b_prio {
        return b_prio.cmp(&a_prio);
      }

      let a_part = a_name.split('-').next().unwrap_or("");
      let b_part = b_name.split('-').next().unwrap_or("");
      if a_part != b_part {
        let a_idx = self.order.iter().position(|v| *v == a_part);
        let b_idx = self.order.iter().position(|v| *v == b_part);
        match (a_idx, b_idx) {
          (Some(ai), Some(bi)) => return ai.cmp(&bi),
          (Some(_), None) => return std::cmp::Ordering::Less,
          (None, Some(_)) => return std::cmp::Ordering::Greater,
          _ => {}
        }
      }

      a_name.cmp(b_name)
    });

    el.attributes.clear();
    for (k, v) in attrs {
      el.attributes.push((self.arena.alloc_str(&k), self.arena.alloc_str(&v)));
    }

    VisitAction::Keep
  }
}
