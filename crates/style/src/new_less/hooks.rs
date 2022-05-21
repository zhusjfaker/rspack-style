use crate::new_less::interceptor::LessInterceptor;
use std::sync::Arc;

pub type ImportAliasHook =
  Option<Arc<dyn Fn(String, String) -> Result<String, String> + Send + Sync>>;

pub type ContentInterceptor =
  Option<Arc<dyn Fn(&str, &str) -> Result<String, String> + Send + Sync>>;

#[derive(Clone)]
pub struct ParseHooks {
  ///
  /// 导入 import 路径的 hook 的 特殊处理
  ///
  pub import_alias: ImportAliasHook,

  ///
  /// 内容的特殊处理 可以 提前加载其他 预处理器
  ///
  pub content_interceptor: ContentInterceptor,
}

impl Default for ParseHooks {
  fn default() -> Self {
    ParseHooks {
      import_alias: None,
      content_interceptor: Some(Arc::new(|filepath, content| {
        LessInterceptor::handle(filepath, content)
      })),
      // content_interceptor: None,
    }
  }
}
