use bumpalo::Bump;

use crate::optimizer::{Plugin, VisitAction};

/// Remove <title>.
#[allow(dead_code)]
pub struct RemoveEditorsNSData<'a> {
  pub arena: &'a Bump,
  /// 暂存 namespace 前缀
  prefixes: Vec<&'a str>,
  /// 需要移除的 namespace 列表
  namespaces: Vec<&'a str>,
}

pub struct RemoveEditorsNSDataConfig<'a> {
  additional_namespace: Option<Vec<&'a str>>,
}

impl<'a> RemoveEditorsNSData<'a> {
  pub fn new(config: RemoveEditorsNSDataConfig<'a>, arena: &'a Bump) -> Self {
    let mut default_namespaces = vec![
      "http://creativecommons.org/ns#",
      "http://inkscape.sourceforge.net/DTD/sodipodi-0.dtd",
      "http://ns.adobe.com/AdobeIllustrator/10.0/",
      "http://ns.adobe.com/AdobeSVGViewerExtensions/3.0/",
      "http://ns.adobe.com/Extensibility/1.0/",
      "http://ns.adobe.com/Flows/1.0/",
      "http://ns.adobe.com/GenericCustomNamespace/1.0/",
      "http://ns.adobe.com/Graphs/1.0/",
      "http://ns.adobe.com/ImageReplacement/1.0/",
      "http://ns.adobe.com/SaveForWeb/1.0/",
      "http://ns.adobe.com/Variables/1.0/",
      "http://ns.adobe.com/XPath/1.0/",
      "http://purl.org/dc/elements/1.1/",
      "http://schemas.microsoft.com/visio/2003/SVGExtensions/",
      "http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd",
      "http://taptrix.com/vectorillustrator/svg_extensions",
      "http://www.bohemiancoding.com/sketch/ns",
      "http://www.figma.com/figma/ns",
      "http://www.inkscape.org/namespaces/inkscape",
      "http://www.serif.com/",
      "http://www.vector.evaxdesign.sk",
      "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    ];
    if let Some(mut additional_namespaces) = config.additional_namespace {
      default_namespaces.append(&mut additional_namespaces);
    }
    RemoveEditorsNSData {
      arena,
      prefixes: vec![],
      namespaces: default_namespaces,
    }
  }
}

impl<'a> Plugin<'a> for RemoveEditorsNSData<'a> {
  fn element_enter(&self, el: &mut crate::parser::XMLAstElement<'a>) -> VisitAction {
    // // collect namespace prefixes from svg element
    // if (node.name === 'svg') {
    //   for (const [name, value] of Object.entries(node.attributes)) {
    //     if (name.startsWith('xmlns:') && namespaces.includes(value)) {
    //       prefixes.push(name.slice('xmlns:'.length));
    //       // <svg xmlns:sodipodi="">
    //       delete node.attributes[name];
    //     }
    //   }
    // }
    // // remove editor attributes, for example
    // // <* sodipodi:*="">
    // for (const name of Object.keys(node.attributes)) {
    //   if (name.includes(':')) {
    //     const [prefix] = name.split(':');
    //     if (prefixes.includes(prefix)) {
    //       delete node.attributes[name];
    //     }
    //   }
    // }
    // // remove editor elements, for example
    // // <sodipodi:*>
    // if (node.name.includes(':')) {
    //   const [prefix] = node.name.split(':');
    //   if (prefixes.includes(prefix)) {
    //     detachNodeFromParent(node, parentNode);
    //   }
    // }
    // if el.name == "svg" {
    //   el.attributes.iter().for_each(|(name, _)| {
    //     if name.starts_with("xmlns:")
    //   });
    // }
    // if el.name == "svg" {
    //   el.attributes.iter().for_each(|(name, value)| {
    //     if name.starts_with("xmlns:") && self.namespaces.iter().any(|key| key == value) {
    //       if let Some(prefix) = name.strip_prefix("xmlns:") {
    //         self.prefixes.push(prefix);
    //       }
    //     }
    //   });
    // }
    VisitAction::Keep
  }
}
