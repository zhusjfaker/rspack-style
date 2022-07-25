use crate::extend::string::StringExtend;
use crate::less::fileinfo::{FileInfo, FileRef};
use crate::less::node::{NodeRef, StyleNode};
use crate::less::parse::Parse;
use crate::less::select_node::SelectorNode;
use crate::sourcemap::loc::{Loc, LocMap};
use crate::style_core::context::ParseContext;
use crate::style_core::extension::StyleExtension;
use crate::style_core::filenode::StyleFileNode;
use crate::style_core::option::OptionExtend;
use crate::util::hash::StyleHash;
use crate::util::str_enum::StringToEnum;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Clone, Debug, Serialize)]
pub struct FileNode {
  pub info: FileRef,
}

impl FileNode {
  ///
  /// 收集当前节点下所有的Rule
  ///
  pub fn getrules(&self) -> Vec<NodeRef> {
    let mut list = vec![];
    self.info.borrow().block_node.iter().for_each(|x| {
      if let StyleNode::Rule(rule) = x {
        list.push(rule.clone())
      }
    });
    list
  }

  ///
  /// 生成代码
  ///
  pub fn code_gen(&self) -> Result<String, String> {
    let mut res = "".to_string();
    let mut set = HashSet::new();
    let info = self.info.borrow();
    let option = info.get_options();
    let mut need_add_cache = false;
    if !info.import_files.is_empty() {
      for item in info.import_files.iter() {
        match item {
          StyleFileNode::Less(less_node) => {
            let has_codegen_record = {
              let context = info.context.lock().unwrap();
              context.has_codegen_record(&less_node.info.borrow().disk_location)
            };
            if !has_codegen_record {
              let import_res = less_node.code_gen()?;
              res += &import_res;
              if !option.minify {
                res += "\n";
              } else {
                res += " ";
              }
            }
          }
        }
      }
    }
    let mut self_code_gen_res = "".to_string();

    let source = {
      info
        .context
        .lock()
        .unwrap()
        .get_render_cache(info.disk_location.as_str())
    };
    if let Some(code) = source {
      self_code_gen_res = code;
    } else {
      for item in self.getrules() {
        item.borrow().code_gen(&mut self_code_gen_res, &mut set)?;
      }
      need_add_cache = true;
    }
    res += self_code_gen_res.as_str();
    let mut context = info.context.lock().unwrap();
    context.add_codegen_record(info.disk_location.as_str());
    // 增加缓存
    if need_add_cache {
      context.add_render_cache(info.disk_location.as_str(), self_code_gen_res.as_str());
    }
    drop(context);
    drop(info);
    if !set.is_empty() {
      self.info.borrow_mut().class_selector_collect = set.clone();
      let info = self.info.borrow();
      let key = info.disk_location.clone();
      let context = info.context.lock().unwrap();
      let mut parse_cache_map = context.filecache.lock().unwrap();
      parse_cache_map.remove(&key);
      let file_info_json = serde_json::to_string_pretty(self).unwrap();
      parse_cache_map.insert(key, file_info_json);
    }
    Ok(res)
  }

  ///
  /// 用来执行多 css 之间的 bundle
  /// 初始化 执行 需要 放入一个 空map 的引用 后续会自动填充到 该参数 上
  ///
  pub fn code_gen_into_map(&self, map: &mut HashMap<String, String>) -> Result<(), String> {
    let info = self.info.borrow();
    let mut set = HashSet::new();
    let mut need_add_cache = false;
    if !info.import_files.is_empty() {
      for item in info.import_files.iter() {
        match item {
          StyleFileNode::Less(less_node) => {
            let has_codegen_record = {
              let context = info.context.lock().unwrap();
              context.has_codegen_record(&less_node.info.borrow().disk_location)
            };
            if !has_codegen_record {
              less_node.code_gen_into_map(map)?;
            }
          }
        }
      }
    }
    let mut res = "".to_string();
    let source = {
      info
        .context
        .lock()
        .unwrap()
        .get_render_cache(info.disk_location.as_str())
    };
    if let Some(code) = source {
      res = code;
    } else {
      for item in self.getrules() {
        item.borrow().code_gen(&mut res, &mut set)?;
      }
      need_add_cache = true;
    }
    // 增加缓存
    let mut context = info.context.lock().unwrap();
    if need_add_cache {
      context.add_render_cache(info.disk_location.as_str(), res.as_str());
    }
    map.insert(self.info.borrow().disk_location.clone(), res);
    context.add_codegen_record(info.disk_location.as_str());
    drop(context);
    drop(info);
    // 拼接css_module 的内容
    if !set.is_empty() {
      self.info.borrow_mut().class_selector_collect = set.clone();
      let info = self.info.borrow();
      let key = info.disk_location.clone();
      let context = info.context.lock().unwrap();
      let mut parse_cache_map = context.filecache.lock().unwrap();
      parse_cache_map.remove(&key);
      let file_info_json = serde_json::to_string_pretty(self).unwrap();
      parse_cache_map.insert(key, file_info_json);
    }
    Ok(())
  }

  ///
  /// 拼接css_module 的 js 导出内容
  ///
  pub fn output_js_with_cssmodule(
    class_selector_collect: &Vec<String>,
    perfix_hash: &str,
  ) -> String {
    let mut res = "".to_string();
    for item in class_selector_collect {
      let key = if item.contains('-') {
        format!(r#"["{}"]"#, item)
      } else {
        item.to_string()
      };
      let value = format!("{}_{}", item, perfix_hash);
      res += format!(r#"{}: "{}","#, key, value).as_str();
      res += " \n";
    }
    res
  }

  ///
  /// 产生 树 hash 用来全局锁定唯一标识
  ///
  pub fn get_hash_perfix(content: &str, filepath: &str) -> String {
    StyleHash::generate_css_module_hash(filepath, content)
  }

  ///
  /// parse 当前文件下 所有的 select 字符串
  /// 需要 第一遍 完成基本遍历
  ///
  pub fn parse_select_all_node(&self) -> Result<(), String> {
    // todo! 若要支持 @{abc} 变量 跨文件调用 select 需要 select 解析放到 codegen 里
    for node in self.info.borrow().block_node.iter() {
      if let StyleNode::Rule(heapnode) = node {
        let mut mut_node = heapnode.borrow_mut();
        if let Some(SelectorNode::Select(s_node)) = mut_node.selector.as_mut() {
          s_node.parse(None)?;
        }
        drop(mut_node);
        heapnode.borrow().parse_select_all_node()?;
      }
    }
    Ok(())
  }

  ///
  /// 是否需要 css module
  ///
  pub fn is_need_css_modules(filepath: &str, modules: Option<bool>) -> bool {
    if let Some(module) = modules {
      module
    } else {
      let path = Path::new(filepath);
      let filename = path.file_name().unwrap().to_str().unwrap().to_string();
      let ext = format!(".module.{}", path.extension().unwrap().to_str().unwrap());
      filename.to_lowercase().contains(&ext.to_lowercase())
    }
  }

  ///
  /// 根据文件路径 解析 文件
  ///
  pub fn create_disklocation_parse(
    filepath: String,
    context: ParseContext,
  ) -> Result<FileNode, String> {
    let cp_context = context.clone();
    let option = {
      let context_value = cp_context.lock().unwrap();
      context_value.get_options()
    };
    let need_modules = {
      let modules = context.lock().unwrap().option.modules;
      Self::is_need_css_modules(filepath.as_str(), modules)
    };
    let (abs_path, mut content) = FileInfo::resolve(filepath, &option.include_path)?;
    let content_transform = {
      context
        .lock()
        .unwrap()
        .option
        .hooks
        .content_interceptor
        .as_ref()
        .cloned()
    };
    if let Some(content_transform_fn) = content_transform {
      content = content_transform_fn(abs_path.as_str(), content.as_str())?;
    }
    let node = {
      let context_value = cp_context.lock().unwrap();
      context_value.get_parse_cache(&abs_path)?
    };
    // 缓存里有的话 直接跳出
    if let Some(StyleFileNode::Less(node)) = node {
      return Ok(node);
    }
    let text_content = content.clone();
    let charlist = content.to_char_vec();
    let mut locmap: Option<LocMap> = None;
    if option.sourcemap {
      locmap = Some(FileInfo::get_loc_by_content(&charlist));
    }
    let ext_str_value = Path::new(abs_path.as_str())
      .extension()
      .unwrap()
      .to_str()
      .unwrap();
    let ext = ext_str_value
      .to_string()
      .to_enum::<StyleExtension>()
      .unwrap();
    let obj = FileInfo {
      disk_location: abs_path.clone(),
      block_node: vec![],
      origin_txt_content: text_content,
      origin_charlist: charlist,
      locmap,
      context,
      self_weak: None,
      import_files: vec![],
      modules: need_modules,
      class_selector_collect: Default::default(),
      hash_perfix: StyleHash::generate_css_module_hash(&abs_path, &content),
      resolve_extension: ext,
    };
    let info = obj.toheap();
    let mut obj = Self { info: info.clone() };
    obj.parse_heap()?;
    obj.parse_select_all_node()?;
    // 把当前 节点 的 对象 指针 放到 节点上 缓存中
    let disk_location = info.borrow().disk_location.clone();
    let file_info_json = serde_json::to_string_pretty(&obj).unwrap();
    let mut context_value = cp_context.lock().unwrap();
    context_value.set_parse_cache(disk_location.as_str(), file_info_json);
    Ok(obj)
  }

  ///
  /// 根据文件路径 转换 文件(分页)
  ///
  pub fn create_disklocation(
    filepath: String,
    context: ParseContext,
  ) -> Result<(String, String), String> {
    let obj = Self::create_disklocation_parse(filepath, context.clone())?;
    let mut res = obj.code_gen()?;
    let mut sync_context = context.lock().unwrap();
    sync_context.clear_codegen_record();
    let mut js_content = "".to_string();
    if obj.info.borrow().modules {
      drop(sync_context);
      let perfix_hash = obj.info.borrow().hash_perfix.clone();
      res = res.replace("@@@hash_str_replace_value@@@", &perfix_hash);
      let js_modules_collect = obj.collect_class_modules_set();
      let mut js_modules_collect = js_modules_collect.into_iter().collect::<Vec<String>>();
      js_modules_collect.sort();
      js_content = format!(
        r#"
    const style = {}
      {}
    {};
    export default style;
    "#,
        "{",
        Self::output_js_with_cssmodule(&js_modules_collect, &perfix_hash),
        "}"
      );
    }
    Ok((res, js_content))
  }

  ///
  /// 根据文件路径 转换 文件
  ///
  pub fn create_disklocation_into_hashmap(
    filepath: String,
    context: ParseContext,
  ) -> Result<(HashMap<String, String>, String), String> {
    let obj = Self::create_disklocation_parse(filepath, context.clone())?;
    let mut map = HashMap::new();
    obj.code_gen_into_map(&mut map)?;
    let mut sync_context = context.lock().unwrap();
    sync_context.clear_codegen_record();
    let mut js_content = "".to_string();
    if obj.info.borrow().modules {
      drop(sync_context);
      let perfix_hash = obj.info.borrow().hash_perfix.clone();
      for res in map.values_mut() {
        *res = res
          .replace("@@@hash_str_replace_value@@@", &perfix_hash)
          .to_string()
      }
      let js_modules_collect = obj.collect_class_modules_set();
      let mut js_modules_collect = js_modules_collect.into_iter().collect::<Vec<String>>();
      js_modules_collect.sort();
      js_content = format!(
        r#"
    const style = {}
      {}
    {};
    export default style;
    "#,
        "{",
        Self::output_js_with_cssmodule(&js_modules_collect, &perfix_hash),
        "}"
      );
    }
    Ok((map, js_content))
  }

  ///
  /// 根据文件内容 解析文件
  /// 内存上 内容
  ///
  pub fn create_txt_content_parse(
    mut content: String,
    context: ParseContext,
    filename: String,
  ) -> Result<Self, String> {
    let node = {
      let context_value = context.lock().unwrap();
      context_value.get_parse_cache(&filename)?
    };
    let need_modules = {
      let modules = context.lock().unwrap().option.modules;
      Self::is_need_css_modules(filename.as_str(), modules)
    };
    // 缓存里有的话 直接跳出
    if let Some(StyleFileNode::Less(node_wrap_value)) = node {
      return Ok(node_wrap_value);
    }
    let content_transform = {
      context
        .lock()
        .unwrap()
        .option
        .hooks
        .content_interceptor
        .as_ref()
        .cloned()
    };
    if let Some(content_transform_fn) = content_transform {
      content = content_transform_fn(filename.as_str(), content.as_str())?;
    }
    let text_content: String = content.clone();
    let charlist = text_content.to_char_vec();
    let cp_context = context.clone();
    let option = {
      let sync_context = cp_context.lock().unwrap();
      sync_context.get_options()
    };
    let mut locmap: Option<LocMap> = None;
    if option.sourcemap {
      locmap = Some(FileInfo::get_loc_by_content(&charlist));
    }
    let ext_str_value = Path::new(filename.as_str())
      .extension()
      .unwrap()
      .to_str()
      .unwrap();
    let ext = ext_str_value
      .to_string()
      .to_enum::<StyleExtension>()
      .unwrap();
    let obj = FileInfo {
      disk_location: filename.clone(),
      block_node: vec![],
      origin_txt_content: text_content,
      origin_charlist: charlist,
      locmap,
      context,
      self_weak: None,
      import_files: vec![],
      modules: need_modules,
      class_selector_collect: Default::default(),
      hash_perfix: StyleHash::generate_css_module_hash(&filename, &content),
      resolve_extension: ext,
    };
    let info = obj.toheap();
    let mut obj = Self { info: info.clone() };
    obj.parse_heap()?;
    obj.parse_select_all_node()?;
    // 把当前 节点 的 对象 指针 放到 节点上 缓存中
    let disk_location = info.borrow().disk_location.clone();
    let file_info_json = serde_json::to_string_pretty(&obj).unwrap();
    {
      let mut sync_context = cp_context.lock().unwrap();
      sync_context.set_parse_cache(disk_location.as_str(), file_info_json);
    }
    Ok(obj)
  }

  ///
  /// codegen 样式内容
  /// 内存上 内容
  ///
  pub fn create_txt_content(
    content: String,
    filename: String,
    context: ParseContext,
  ) -> Result<(String, String), String> {
    let obj = Self::create_txt_content_parse(content, context.clone(), filename)?;
    let mut sync_context = context.lock().unwrap();
    let mut res = obj.code_gen()?;
    sync_context.clear_codegen_record();
    let mut js_content = "".to_string();
    if obj.info.borrow().modules {
      drop(sync_context);
      let perfix_hash = obj.info.borrow().hash_perfix.clone();
      res = res.replace("@@@hash_str_replace_value@@@", &perfix_hash);
      let js_modules_collect = obj.collect_class_modules_set();
      let mut js_modules_collect = js_modules_collect.into_iter().collect::<Vec<String>>();
      js_modules_collect.sort();
      js_content = format!(
        r#"
    const style = {}
      {}
    {};
    export default style;
    "#,
        "{",
        Self::output_js_with_cssmodule(&js_modules_collect, &perfix_hash),
        "}"
      );
    }
    Ok((res, js_content))
  }

  ///
  /// 根据文件路径 转换 文件(分页)
  /// 内存上 内容
  ///
  pub fn create_content_into_hashmap(
    content: String,
    filepath: String,
    context: ParseContext,
  ) -> Result<(HashMap<String, String>, String), String> {
    let obj = Self::create_txt_content_parse(content, context.clone(), filepath)?;
    let mut map = HashMap::new();
    obj.code_gen_into_map(&mut map)?;
    let mut sync_context = context.lock().unwrap();
    sync_context.clear_codegen_record();
    let mut js_content = "".to_string();
    if obj.info.borrow().modules {
      drop(sync_context);
      let perfix_hash = obj.info.borrow().hash_perfix.clone();
      for res in map.values_mut() {
        *res = res
          .replace("@@@hash_str_replace_value@@@", &perfix_hash)
          .to_string()
      }
      let js_modules_collect = obj.collect_class_modules_set();
      let mut js_modules_collect = js_modules_collect.into_iter().collect::<Vec<String>>();
      js_modules_collect.sort();
      js_content = format!(
        r#"
    const style = {}
      {}
    {};
    export default style;
    "#,
        "{",
        Self::output_js_with_cssmodule(&js_modules_collect, &perfix_hash),
        "}"
      );
    }
    Ok((map, js_content))
  }

  ///
  ///  收集 文件 上所有 rule 节点的 loc
  ///
  pub fn collect_loc_list(&self) -> Vec<Option<Loc>> {
    let mut list = vec![];
    for node in &self.info.borrow().block_node {
      StyleNode::collect_loc(node.clone(), &mut list);
    }
    list
  }

  ///
  /// 渲染时 获取 引用树上的 所有值
  ///
  pub fn collect_class_modules_set(&self) -> HashSet<String> {
    let mut self_set = self.info.borrow().class_selector_collect.clone();
    for file in &self.info.borrow().import_files {
      crate::style_core::filenode::StyleFileNode::collect_class_modules_set(file, &mut self_set);
    }
    self_set
  }
}
