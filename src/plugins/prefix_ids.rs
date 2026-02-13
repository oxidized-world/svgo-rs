use bumpalo::Bump;
use regex::{Captures, Regex};

use crate::optimizer::{Plugin, VisitAction};
use crate::parser::{XMLAstChild, XMLAstElement};

pub struct PrefixIdsPlugin<'a> {
  pub arena: &'a Bump,
  pub prefix: &'a str,
  pub delim: &'a str,
  pub prefix_ids: bool,
  pub prefix_class_names: bool,
  reg_url_ref: Regex,
  reg_style_url_ref: Regex,
  reg_style_id_selector: Regex,
  reg_style_class_selector: Regex,
}

pub struct PrefixIdsPluginConfig<'a> {
  pub prefix: Option<&'a str>,
  pub delim: &'a str,
  pub prefix_ids: bool,
  pub prefix_class_names: bool,
}

impl<'a> PrefixIdsPlugin<'a> {
  pub fn new(config: PrefixIdsPluginConfig<'a>, arena: &'a Bump) -> Self {
    PrefixIdsPlugin {
      arena,
      prefix: config.prefix.unwrap_or("prefix"),
      delim: config.delim,
      prefix_ids: config.prefix_ids,
      prefix_class_names: config.prefix_class_names,
      reg_url_ref: Regex::new(r#"(?i)\burl\((?:['"])?(#.+?)(?:['"])?\)"#).unwrap(),
      reg_style_url_ref: Regex::new(r#"(?i)\burl\((?:['"])?(#([A-Za-z_][\w:.-]*))(?:['"])?\)"#)
        .unwrap(),
      reg_style_id_selector: Regex::new(r"(^|[\s,{>+~])#([A-Za-z_][\w:.-]*)").unwrap(),
      reg_style_class_selector: Regex::new(r"(^|[\s,{>+~])\.([A-Za-z_][\w:.-]*)").unwrap(),
    }
  }

  fn prefixed(&self, body: &str) -> String {
    let prefix = format!("{}{}", self.prefix, self.delim);
    if body.starts_with(&prefix) {
      body.to_string()
    } else {
      format!("{prefix}{body}")
    }
  }

  fn prefix_reference(&self, value: &str) -> Option<String> {
    if !value.starts_with('#') {
      return None;
    }
    let body = &value[1..];
    Some(format!("#{}", self.prefixed(body)))
  }

  fn rewrite_url_refs(&self, value: &str) -> String {
    self
      .reg_url_ref
      .replace_all(value, |caps: &Captures<'_>| {
        let full = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        if let Some(prefixed) = self.prefix_reference(full) {
          format!("url({prefixed})")
        } else {
          caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string()
        }
      })
      .to_string()
  }

  fn rewrite_style_text(&self, css: &str) -> String {
    let with_urls = self
      .reg_style_url_ref
      .replace_all(css, |caps: &Captures<'_>| {
        let full = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        if let Some(prefixed) = self.prefix_reference(full) {
          format!("url({prefixed})")
        } else {
          caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string()
        }
      })
      .to_string();

    let with_ids = if self.prefix_ids {
      self
        .reg_style_id_selector
        .replace_all(&with_urls, |caps: &Captures<'_>| {
          let lead = caps.get(1).map(|m| m.as_str()).unwrap_or("");
          let id = caps.get(2).map(|m| m.as_str()).unwrap_or("");
          format!("{lead}#{}", self.prefixed(id))
        })
        .to_string()
    } else {
      with_urls
    };

    if self.prefix_class_names {
      self
        .reg_style_class_selector
        .replace_all(&with_ids, |caps: &Captures<'_>| {
          let lead = caps.get(1).map(|m| m.as_str()).unwrap_or("");
          let class_name = caps.get(2).map(|m| m.as_str()).unwrap_or("");
          format!("{lead}.{}", self.prefixed(class_name))
        })
        .to_string()
    } else {
      with_ids
    }
  }

  fn rewrite_begin_or_end(&self, value: &str) -> String {
    value
      .split(';')
      .map(|item| item.trim())
      .map(|item| {
        if let Some(id) = item.strip_suffix(".end") {
          format!("{}.end", self.prefixed(id))
        } else if let Some(id) = item.strip_suffix(".start") {
          format!("{}.start", self.prefixed(id))
        } else {
          item.to_string()
        }
      })
      .collect::<Vec<String>>()
      .join("; ")
  }
}

impl<'a> Plugin<'a> for PrefixIdsPlugin<'a> {
  fn element_enter(&mut self, el: &mut XMLAstElement<'a>) -> VisitAction {
    if el.name == "style" {
      for child in &mut el.children {
        match child {
          XMLAstChild::Text(text) => {
            let next = self.rewrite_style_text(text.value);
            text.value = self.arena.alloc_str(&next);
          }
          XMLAstChild::Cdata(cdata) => {
            let next = self.rewrite_style_text(cdata.value);
            cdata.value = self.arena.alloc_str(&next);
          }
          _ => {}
        }
      }
    }

    for (name, value) in &mut el.attributes {
      if self.prefix_ids && *name == "id" && !value.is_empty() {
        let next = self.prefixed(value);
        *value = self.arena.alloc_str(&next);
        continue;
      }

      if self.prefix_class_names && *name == "class" && !value.is_empty() {
        let next = value
          .split_whitespace()
          .map(|name| self.prefixed(name))
          .collect::<Vec<String>>()
          .join(" ");
        *value = self.arena.alloc_str(&next);
        continue;
      }

      if (*name == "href" || *name == "xlink:href") && !value.is_empty() {
        if let Some(next) = self.prefix_reference(value) {
          *value = self.arena.alloc_str(&next);
        }
        continue;
      }

      if *name == "begin" || *name == "end" {
        let next = self.rewrite_begin_or_end(value);
        *value = self.arena.alloc_str(&next);
        continue;
      }

      let next = self.rewrite_url_refs(value);
      if next != *value {
        *value = self.arena.alloc_str(&next);
      }
    }

    VisitAction::Keep
  }
}
