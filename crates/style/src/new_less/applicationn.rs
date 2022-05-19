use crate::new_less::context::{Context, ParseContext};
use crate::new_less::filenode::FileNode;
use crate::new_less::option::ParseOption;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Application {
  pub context: ParseContext,
}

impl Application {
  pub fn new(option: ParseOption, application_fold: Option<String>) -> Result<Self, String> {
    let context = Context::new(option, application_fold)?;
    Ok(Application { context })
  }

  ///
  /// 产生代码
  ///
  pub fn render(&self, filepath: &str) -> Result<String, String> {
    FileNode::create_disklocation(filepath.to_string(), self.context.clone())
  }

  ///
  /// 产生代码
  /// 并且分层 进入 hashmap
  ///
  pub fn render_into_hashmap(&self, filepath: &str) -> Result<(HashMap<String, String>, String), String> {
    FileNode::create_disklocation_into_hashmap(filepath.to_string(), self.context.clone())
  }

  ///
  /// 解析代码
  ///
  pub fn parse(&self, filepath: &str) -> Result<FileNode, String> {
    FileNode::create_disklocation_parse(filepath.to_string(), self.context.clone())
  }

  ///
  /// 生成默认上下文
  ///
  pub fn default() -> Application {
    Self::new(Default::default(), None).unwrap()
  }
}
