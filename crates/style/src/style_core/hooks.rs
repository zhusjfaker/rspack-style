use std::sync::Arc;

pub type ImportAliasHook =
  Option<Arc<dyn Fn(String, String) -> Result<String, String> + Send + Sync>>;

pub type ContentInterceptor =
  Option<Arc<dyn Fn(&str, &str) -> Result<String, String> + Send + Sync>>;

#[derive(Clone, Default)]
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
