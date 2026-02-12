use bumpalo::Bump;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::optimizer::Plugin;
use crate::parser::{XMLAstChild, XMLAstElement, XMLAstRoot};

pub struct CleanupIdsPlugin<'a> {
  pub arena: &'a Bump,
  pub remove: bool,
  pub minify: bool,
  pub preserve: HashSet<&'a str>,
  pub preserve_prefixes: Vec<&'a str>,
  pub force: bool,
}

pub struct CleanupIdsPluginConfig<'a> {
  pub remove: bool,
  pub minify: bool,
  pub preserve: Vec<&'a str>,
  pub preserve_prefixes: Vec<&'a str>,
  pub force: bool,
}

impl<'a> CleanupIdsPlugin<'a> {
  pub fn new(config: CleanupIdsPluginConfig<'a>, arena: &'a Bump) -> Self {
    CleanupIdsPlugin {
      arena,
      remove: config.remove,
      minify: config.minify,
      preserve: config.preserve.into_iter().collect(),
      preserve_prefixes: config.preserve_prefixes,
      force: config.force,
    }
  }

  fn has_prefix(&self, value: &str) -> bool {
    self.preserve_prefixes.iter().any(|prefix| value.starts_with(prefix))
  }

  fn is_preserved(&self, value: &str) -> bool {
    self.preserve.contains(value) || self.has_prefix(value)
  }

  fn has_scripts_or_style(children: &[XMLAstChild<'a>]) -> bool {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        if el.name == "style" && !el.children.is_empty() {
          return true;
        }
        if el.name == "script" && !el.children.is_empty() {
          return true;
        }
        if el.attributes.iter().any(|(k, _)| k.starts_with("on")) {
          return true;
        }
        if Self::has_scripts_or_style(&el.children) {
          return true;
        }
      }
    }
    false
  }

  fn collect(
    children: &[XMLAstChild<'a>],
    ids: &mut HashSet<String>,
    refs: &mut HashSet<String>,
    reg_href: &Regex,
    reg_begin: &Regex,
  ) {
    for child in children {
      if let XMLAstChild::Element(el) = child {
        for (name, value) in &el.attributes {
          if *name == "id" {
            ids.insert((*value).to_string());
          } else {
            if [
              "clip-path",
              "color-profile",
              "fill",
              "filter",
              "marker-end",
              "marker-mid",
              "marker-start",
              "mask",
              "stroke",
              "style",
            ]
            .contains(name)
            {
              for id in Self::extract_url_hash_refs(value) {
                refs.insert(id);
              }
            }
            if *name == "href" || name.ends_with(":href") {
              if let Some(cap) = reg_href.captures(value) {
                if let Some(id) = cap.get(1) {
                  refs.insert(id.as_str().to_string());
                }
              }
            }
            if *name == "begin" {
              if let Some(cap) = reg_begin.captures(value) {
                if let Some(id) = cap.get(1) {
                  refs.insert(id.as_str().to_string());
                }
              }
            }
          }
        }
        Self::collect(&el.children, ids, refs, reg_href, reg_begin);
      }
    }
  }

  fn extract_url_hash_refs(value: &str) -> Vec<String> {
    let mut refs = Vec::new();
    let mut start = 0usize;

    while let Some(found) = value[start..].find("url(") {
      let mut i = start + found + 4;
      let bytes = value.as_bytes();

      while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
      }

      if i >= bytes.len() {
        break;
      }

      let quote = if bytes[i] == b'\'' || bytes[i] == b'"' {
        let q = bytes[i];
        i += 1;
        Some(q)
      } else {
        None
      };

      if i >= bytes.len() || bytes[i] != b'#' {
        start = i;
        continue;
      }
      i += 1;

      let id_start = i;
      while i < bytes.len() {
        let ch = bytes[i];
        if let Some(q) = quote {
          if ch == q {
            break;
          }
        } else if ch == b')' || ch.is_ascii_whitespace() {
          break;
        }
        i += 1;
      }

      if i > id_start {
        refs.push(value[id_start..i].to_string());
      }

      if let Some(q) = quote {
        if i < bytes.len() && bytes[i] == q {
          i += 1;
        }
      }

      while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
      }

      if i < bytes.len() && bytes[i] == b')' {
        i += 1;
      }

      start = i;
    }

    refs
  }

  fn generate_id(mut num: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut out = String::new();
    loop {
      let rem = num % CHARS.len();
      out.insert(0, CHARS[rem] as char);
      num /= CHARS.len();
      if num == 0 {
        break;
      }
      num -= 1;
    }
    out
  }

  fn rewrite(
    children: &mut [XMLAstChild<'a>],
    id_map: &HashMap<String, String>,
    remove: bool,
    preserve: &dyn Fn(&str) -> bool,
    arena: &'a Bump,
  ) {
    let mut seen_ids: HashSet<String> = HashSet::new();

    fn rewrite_element<'a>(
      el: &mut XMLAstElement<'a>,
      id_map: &HashMap<String, String>,
      remove: bool,
      preserve: &dyn Fn(&str) -> bool,
      seen_ids: &mut HashSet<String>,
      arena: &'a Bump,
    ) {
      for (name, value) in &mut el.attributes {
        if *name == "id" {
          let id = (*value).to_string();
          if let Some(new_id) = id_map.get(&id) {
            *value = arena.alloc_str(new_id);
          } else if remove && !preserve(&id) {
            *value = "";
          }

          if !value.is_empty() {
            let current = (*value).to_string();
            if seen_ids.contains(&current) {
              *value = "";
            } else {
              seen_ids.insert(current);
            }
          }
          continue;
        }

        let mut updated = (*value).to_string();
        for (old_id, new_id) in id_map {
          updated = updated.replace(&format!("#{old_id}"), &format!("#{new_id}"));
          updated = updated.replace(&format!("{old_id}."), &format!("{new_id}."));
        }
        *value = arena.alloc_str(&updated);
      }

      el.attributes.retain(|(k, v)| !(*k == "id" && v.is_empty()));

      for child in &mut el.children {
        if let XMLAstChild::Element(child_el) = child {
          rewrite_element(child_el, id_map, remove, preserve, seen_ids, arena);
        }
      }
    }

    for child in children {
      if let XMLAstChild::Element(el) = child {
        rewrite_element(el, id_map, remove, preserve, &mut seen_ids, arena);
      }
    }
  }
}

impl<'a> Plugin<'a> for CleanupIdsPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    if !self.force && Self::has_scripts_or_style(&root.children) {
      return;
    }

    let reg_href = Regex::new(r"^#(.+?)$").unwrap();
    let reg_begin = Regex::new(r"(\w+)\.[a-zA-Z]").unwrap();

    let mut ids = HashSet::new();
    let mut refs = HashSet::new();
    Self::collect(&root.children, &mut ids, &mut refs, &reg_href, &reg_begin);

    let mut id_map: HashMap<String, String> = HashMap::new();
    if self.minify {
      let mut index = 0usize;
      for id in &refs {
        if ids.contains(id) && !self.is_preserved(id) {
          loop {
            let candidate = Self::generate_id(index);
            index += 1;
            if !self.is_preserved(&candidate) && !refs.contains(&candidate) {
              id_map.insert(id.clone(), candidate);
              break;
            }
          }
        }
      }
    }

    let preserve = |id: &str| self.is_preserved(id);
    Self::rewrite(
      &mut root.children,
      &id_map,
      self.remove,
      &preserve,
      self.arena,
    );

    if self.remove {
      let refs_after_rewrite: HashSet<String> = refs
        .iter()
        .map(|id| id_map.get(id).cloned().unwrap_or_else(|| id.clone()))
        .collect();

      fn prune_unreferenced<'a>(
        children: &mut [XMLAstChild<'a>],
        refs: &HashSet<String>,
        preserve: &dyn Fn(&str) -> bool,
      ) {
        for child in children {
          if let XMLAstChild::Element(el) = child {
            el.attributes.retain(|(k, v)| {
              if *k != "id" {
                return true;
              }
              refs.contains(*v) || preserve(v)
            });
            prune_unreferenced(&mut el.children, refs, preserve);
          }
        }
      }
      prune_unreferenced(&mut root.children, &refs_after_rewrite, &preserve);
    }
  }
}
