use crate::interceptor::less_interceptor::LessInterceptor;
use crate::style_core::context::{Context, ParseContext};
use crate::style_core::extension::StyleExtension;
use crate::style_core::filenode::StyleFileNode;
use crate::style_core::option::ParseOption;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Application {
  pub context: ParseContext,
}

impl Application {
  ///
  /// 初始化函数
  ///
  pub fn new(option: ParseOption, application_fold: Option<String>) -> Result<Self, String> {
    let context = Context::new(option, application_fold)?;
    Ok(Application { context })
  }

  ///
  /// 修复输出 是否需要压缩
  ///
  pub fn set_minify(&self, minify: bool) {
    let mut context = self.context.lock().unwrap();
    context.clear_render_cache();
    context.option.minify = minify;
  }

  ///
  /// 产生代码
  /// 根据 硬盘上 文件
  ///
  pub fn render(&self, filepath: &str) -> Result<String, String> {
    let ext = StyleExtension::from_filepath(filepath);
    if let Some(ext) = ext {
      return match ext {
        StyleExtension::Css => crate::css::filenode::FileNode::create_disklocation(
          filepath.to_string(),
          self.context.clone(),
        ),
        StyleExtension::Less => crate::less::filenode::FileNode::create_disklocation(
          filepath.to_string(),
          self.context.clone(),
        ),
        StyleExtension::Sass => Err("render -> sass file has not support".to_string()),
        StyleExtension::Scss => Err("render -> sass file has not support".to_string()),
      };
    }
    Err("render -> filepath is not file!".to_string())
  }

  ///
  /// 产生代码
  /// 根据 硬盘上 文件
  /// 降级为 css 处理
  ///
  pub fn render_with_css(&self, filepath: &str) -> Result<String, String> {
    crate::css::filenode::FileNode::create_disklocation(
      filepath.to_string(),
      self.context.clone(),
    )
  }

  ///
  /// 产生代码
  /// 根据 内存上 内容
  ///
  pub fn render_content(&self, content: &str, filepath: &str) -> Result<String, String> {
    let ext = StyleExtension::from_filepath(filepath);
    if let Some(ext) = ext {
      return match ext {
        StyleExtension::Css => crate::css::filenode::FileNode::create_txt_content(
          content.to_string(),
          filepath.to_string(),
          self.context.clone(),
        ),
        StyleExtension::Less => crate::less::filenode::FileNode::create_txt_content(
          content.to_string(),
          filepath.to_string(),
          self.context.clone(),
        ),
        StyleExtension::Sass => Err("render_content -> sass file has not support".to_string()),
        StyleExtension::Scss => Err("render_content -> sass file has not support".to_string()),
      };
    }
    Err("render_content -> filepath is not file!".to_string())
  }

  ///
  /// 产生代码
  /// 根据 内存上 内容
  /// 降级为 css 处理
  ///
  pub fn render_content_with_css(&self, content: &str, filepath: &str) -> Result<String, String> {
    crate::css::filenode::FileNode::create_txt_content(
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
    let ext = StyleExtension::from_filepath(filepath);
    if let Some(ext) = ext {
      return match ext {
        StyleExtension::Css => crate::css::filenode::FileNode::create_disklocation_into_hashmap(
          filepath.to_string(),
          self.context.clone(),
        ),
        StyleExtension::Less => crate::less::filenode::FileNode::create_disklocation_into_hashmap(
          filepath.to_string(),
          self.context.clone(),
        ),
        StyleExtension::Sass => Err("render_into_hashmap -> sass file has not support".to_string()),
        StyleExtension::Scss => Err("render_into_hashmap -> sass file has not support".to_string()),
      };
    }
    Err("render_into_hashmap -> filepath is not file!".to_string())
  }

  ///
  /// 产生代码
  /// 并且分层 进入 hashmap
  /// 根据 硬盘上 文件
  /// 降级为 css 处理
  ///
  pub fn render_into_hashmap_with_css(
    &self,
    filepath: &str,
  ) -> Result<(HashMap<String, String>, String), String> {
    crate::css::filenode::FileNode::create_disklocation_into_hashmap(
      filepath.to_string(),
      self.context.clone(),
    )
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
    let ext = StyleExtension::from_filepath(filepath);
    if let Some(ext) = ext {
      return match ext {
        StyleExtension::Css => crate::css::filenode::FileNode::create_content_into_hashmap(
          content.to_string(),
          filepath.to_string(),
          self.context.clone(),
        ),
        StyleExtension::Less => crate::less::filenode::FileNode::create_content_into_hashmap(
          content.to_string(),
          filepath.to_string(),
          self.context.clone(),
        ),
        StyleExtension::Sass => {
          Err("render_content_into_hashmap -> sass file has not support".to_string())
        }
        StyleExtension::Scss => {
          Err("render_content_into_hashmap -> sass file has not support".to_string())
        }
      };
    }
    Err("render_content_into_hashmap -> filepath is not file!".to_string())
  }

  ///
  /// 产生代码
  /// 并且分层 进入 hashmap
  /// 根据 内存上 内容
  /// 降级为 css 处理
  ///
  pub fn render_content_into_hashmap_with_css(
    &self,
    content: &str,
    filepath: &str,
  ) -> Result<(HashMap<String, String>, String), String> {
    crate::css::filenode::FileNode::create_content_into_hashmap(
      content.to_string(),
      filepath.to_string(),
      self.context.clone(),
    )
  }

  ///
  /// 解析代码
  ///
  pub fn parse(&self, filepath: &str) -> Result<StyleFileNode, String> {
    let ext = StyleExtension::from_filepath(filepath);
    if let Some(ext) = ext {
      return match ext {
        StyleExtension::Css => {
          let node = crate::css::filenode::FileNode::create_disklocation_parse(
            filepath.to_string(),
            self.context.clone(),
          )?;
          Ok(StyleFileNode::Css(node))
        }
        StyleExtension::Less => {
          let node = crate::less::filenode::FileNode::create_disklocation_parse(
            filepath.to_string(),
            self.context.clone(),
          )?;
          Ok(StyleFileNode::Less(node))
        }
        StyleExtension::Sass => Err("parse-> sass file has not support".to_string()),
        StyleExtension::Scss => Err("parse-> scss file has not support".to_string()),
      };
    }
    Err("parse-> filepath is not file!".to_string())
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
