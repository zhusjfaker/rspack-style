use crate::new_less::context::{Context, ParseContext};
use crate::new_less::filenode::FileNode;
use crate::new_less::interceptor::LessInterceptor;
use crate::new_less::option::ParseOption;
use std::collections::HashMap;
use std::sync::Arc;

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
  /// 根据 硬盘上 文件
  ///
  pub fn render(&self, filepath: &str) -> Result<String, String> {
    FileNode::create_disklocation(filepath.to_string(), self.context.clone())
  }

  ///
  /// 产生代码
  /// 根据 内存上 内容
  ///
  pub fn render_content(&self, content: &str, filepath: &str) -> Result<String, String> {
    FileNode::create_txt_content(
      content.to_string(),
      filepath.to_string(),
      self.context.clone(),
    )
  }

  ///
  /// 产生代码
  /// 并且分层 进入 hashmap
  /// 根据 硬盘上 文件
  ///
  pub fn render_into_hashmap(
    &self,
    filepath: &str,
  ) -> Result<(HashMap<String, String>, String), String> {
    FileNode::create_disklocation_into_hashmap(filepath.to_string(), self.context.clone())
  }

  ///
  /// 产生代码
  /// 并且分层 进入 hashmap
  /// 根据 内存上 内容
  ///
  pub fn render_content_into_hashmap(
    &self,
    content: &str,
    filepath: &str,
  ) -> Result<(HashMap<String, String>, String), String> {
    FileNode::create_content_into_hashmap(
      content.to_string(),
      filepath.to_string(),
      self.context.clone(),
    )
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

  ///
  /// 增加 less.js 的 内容变化
  ///
  pub fn add_lessjs_content_interceptor(&mut self) {
    let mut context = self.context.lock().unwrap();
    context.option.hooks.content_interceptor = Some(Arc::new(|filepath, content| {
      LessInterceptor::handle(filepath, content)
    }));
  }
}
