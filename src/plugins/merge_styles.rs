use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;

use crate::optimizer::Plugin;
use crate::parser::{XMLAstCdata, XMLAstChild, XMLAstRoot, XMLAstText};

pub struct MergeStylesPlugin<'a> {
  pub arena: &'a Bump,
}

pub struct MergeStylesPluginConfig {}

impl<'a> MergeStylesPlugin<'a> {
  pub fn new(_config: MergeStylesPluginConfig, arena: &'a Bump) -> Self {
    MergeStylesPlugin { arena }
  }

  fn process_children(&self, children: &mut BumpVec<'a, XMLAstChild<'a>>, in_foreign_object: bool) {
    if in_foreign_object {
      return;
    }

    for child in children.iter_mut() {
      if let XMLAstChild::Element(el) = child {
        self.process_children(&mut el.children, el.name == "foreignObject");
      }
    }

    let mut first_style_index: Option<usize> = None;
    let mut collected_styles = String::new();
    let mut use_cdata = false;

    let mut i = 0usize;
    while i < children.len() {
      let mut remove_current = false;
      let mut css_for_this: Option<String> = None;
      let mut has_media = false;
      let mut media_value = String::new();
      let mut valid_style = false;

      if let XMLAstChild::Element(el) = &mut children[i] {
        if el.name == "style" {
          let style_type =
            el.attributes.iter().find(|(k, _)| *k == "type").map(|(_, v)| *v).unwrap_or("");
          if style_type.is_empty() || style_type == "text/css" {
            valid_style = true;

            let mut css = String::new();
            for c in &el.children {
              match c {
                XMLAstChild::Text(t) => css.push_str(t.value),
                XMLAstChild::Cdata(cd) => {
                  use_cdata = true;
                  css.push_str(cd.value);
                }
                _ => {}
              }
            }

            if css.trim().is_empty() {
              remove_current = true;
            } else {
              css_for_this = Some(css);
              if let Some((_, media)) = el.attributes.iter().find(|(k, _)| *k == "media") {
                has_media = true;
                media_value = (*media).to_string();
              }

              if first_style_index.is_none() {
                first_style_index = Some(i);
                if has_media {
                  el.attributes.retain(|(k, _)| *k != "media");
                }
              } else {
                remove_current = true;
              }
            }
          }
        }
      }

      if valid_style {
        if let Some(css) = css_for_this {
          if has_media {
            collected_styles.push_str("@media ");
            collected_styles.push_str(&media_value);
            collected_styles.push('{');
            collected_styles.push_str(&css);
            collected_styles.push('}');
          } else {
            collected_styles.push_str(&css);
          }
        }
      }

      if remove_current {
        children.remove(i);
        continue;
      }

      i += 1;
    }

    if let Some(first_idx) = first_style_index {
      if let Some(XMLAstChild::Element(first)) = children.get_mut(first_idx) {
        let mut merged_children = BumpVec::new_in(self.arena);
        if use_cdata {
          merged_children.push(XMLAstChild::Cdata(XMLAstCdata {
            value: self.arena.alloc_str(&collected_styles),
          }));
        } else {
          merged_children.push(XMLAstChild::Text(XMLAstText {
            value: self.arena.alloc_str(&collected_styles),
          }));
        }
        first.children = merged_children;
      }
    }
  }
}

impl<'a> Plugin<'a> for MergeStylesPlugin<'a> {
  fn root_exit(&self, root: &mut XMLAstRoot<'a>) {
    self.process_children(&mut root.children, false);
  }
}
