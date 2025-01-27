use crate::process_xml::XmlPlugin;

// 示例插件1：为指定元素添加 class 属性
pub struct ClassAdderPlugin {
  pub target_element: String,
  pub class_name: String,
}

impl XmlPlugin for ClassAdderPlugin {
  fn handle_start_element(&mut self, name: &[u8], attrs: &mut Vec<(String, String)>) {
    if name == self.target_element.as_bytes() {
      // 直接在 attrs 中间结构中操作
      let mut has_class = false;
      for (k, v) in attrs.iter_mut() {
        if k == "class" {
          *v += &format!(" {}", self.class_name);
          has_class = true;
          break;
        }
      }
      if !has_class {
        attrs.push(("class".into(), self.class_name.clone()));
      }
      println!("ClassAdderPlugin: {:?}", attrs);
    }
  }
}
