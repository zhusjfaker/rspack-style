use serde::Serialize;
use std::collections::HashSet;

#[derive(Clone, Serialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum StyleFileNode {
  Less(crate::less::filenode::FileNode),
  Css(crate::css::filenode::FileNode),
}

impl StyleFileNode {
  pub fn to_less(self) -> Option<crate::less::filenode::FileNode> {
    if let StyleFileNode::Less(node) = self {
      Some(node)
    } else {
      None
    }
  }

  pub fn to_css(self) -> Option<crate::css::filenode::FileNode> {
    if let StyleFileNode::Css(node) = self {
      Some(node)
    } else {
      None
    }
  }

  pub fn collect_class_modules_set(file: &StyleFileNode, list: &mut HashSet<String>) {
    match file {
      StyleFileNode::Less(lessfile) => {
        let file_class_module_list = &lessfile.info.borrow().class_selector_collect;
        for item in file_class_module_list {
          list.insert(item.to_string());
        }
        let self_import_files = &lessfile.info.borrow().import_files;
        for child_file in self_import_files {
          Self::collect_class_modules_set(child_file, list);
        }
      }
      StyleFileNode::Css(cssfile) => {
        let file_class_module_list = &cssfile.info.borrow().class_selector_collect;
        for item in file_class_module_list {
          list.insert(item.to_string());
        }
        let self_import_files = &cssfile.info.borrow().import_files;
        for child_file in self_import_files {
          let wrap_file_node = StyleFileNode::Css(child_file.clone());
          Self::collect_class_modules_set(&wrap_file_node, list);
        }
      }
    }
  }
}
