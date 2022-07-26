use crate::extend::string::StringExtend;
use crate::style_core::extension::StyleExtension;
use crate::style_core::filenode::StyleFileNode;
use crate::style_core::option::ParseOption;
use crate::util::file::get_dir;
use crate::util::str_enum::StringToEnum;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex, Weak};

pub type ParseCacheMap = Mutex<HashMap<String, String>>;

pub type ParseContext = Arc<Mutex<Context>>;

pub type RenderCacheMap = Mutex<HashMap<String, String>>;

///
/// 全局调用 转化时的 上下文
///
pub struct Context {
  // 内部调用方式时 需要拿到对应的 转化配置
  pub option: ParseOption,
  // 转文件 的缓存
  pub filecache: ParseCacheMap,
  // 渲染结果 的缓存
  pub render_cache: RenderCacheMap,
  // 文件的绝对路径 入口文件
  pub application_fold: String,
  // 已经生成目录的 文件
  pub code_gen_file_path: Vec<String>,
  // 自身的弱引用
  pub weak_ref: Option<Weak<Mutex<Context>>>,
}

impl Debug for Context {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Context")
      .field("option", &self.option)
      .field("entry", &self.application_fold)
      .field("filepaths", &self.code_gen_file_path)
      .finish()
  }
}

impl Context {
  ///
  /// 创建全局应用 共享上下文
  ///
  pub fn new(
    option: ParseOption,
    application_fold: Option<String>,
  ) -> Result<Arc<Mutex<Self>>, String> {
    let mut fold = application_fold.unwrap_or_else(|| {
      std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
    });
    let filepath = Path::new(&fold);
    if filepath.exists() {
      if filepath.is_absolute() {
        if filepath.is_file() {
          let current_dir = get_dir(&fold)?;
          fold = current_dir
        } else if !filepath.is_dir() {
          return Err(format!("application_fold is not file or dir,{}", fold));
        }
      } else {
        return Err(format!(
          "application_fold is must absolutely path ,{}",
          fold
        ));
      }
    } else {
      return Err(format!("application_fold is not exists,{}", fold));
    }
    let mut obj = Context {
      option,
      filecache: Mutex::new(HashMap::new()),
      render_cache: Mutex::new(HashMap::new()),
      application_fold: fold.clone(),
      code_gen_file_path: vec![],
      weak_ref: None,
    };
    obj.set_include_paths(vec![fold]);
    let heap_obj = Arc::new(Mutex::new(obj));
    {
      let mut self_value = heap_obj.try_lock().unwrap();
      self_value.weak_ref = Some(Arc::downgrade(&heap_obj));
    }
    Ok(heap_obj)
  }
  
  ///
  /// 查询 缓存上 翻译结果
  ///
  pub fn get_parse_cache(&self, file_path: &str) -> Result<Option<StyleFileNode>, String> {
    self.recovery_parse_object(file_path)
  }
  
  ///
  /// 添加 缓存上 翻译结果
  ///
  pub fn set_parse_cache(&mut self, file_path: &str, file_info_json: String) {
    let mut filecache = self.filecache.lock().unwrap();
    let res = filecache.get(file_path);
    if res.is_none() {
      filecache.insert(file_path.to_string(), file_info_json);
    }
  }
  
  ///
  /// 清除 parse cache
  /// 由于现在 缓存的是 指针 只能 单次 transform 同一个文件多次使用
  ///
  pub fn clear_parse_cache(&mut self) {
    self.filecache.lock().unwrap().clear();
  }
  
  ///
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    self.option.clone()
  }
  
  ///
  /// 安全设置 include-path
  ///
  pub fn set_include_paths(&mut self, paths: Vec<String>) {
    paths.iter().for_each(|x| {
      if !self.option.include_path.contains(x) {
        self.option.include_path.push(x.clone());
      }
    });
  }
  
  ///
  /// 增加一个css transform 下 @import 引用生成的记录
  ///
  pub fn add_codegen_record(&mut self, path: &str) {
    self.code_gen_file_path.push(path.to_string());
  }
  
  ///
  /// 清除本次 codegen 文件的记录
  ///
  pub fn clear_codegen_record(&mut self) {
    self.code_gen_file_path.clear();
  }
  
  ///
  /// 是否已经生成过 该文件
  ///
  pub fn has_codegen_record(&self, path: &str) -> bool {
    self.code_gen_file_path.contains(&path.to_string())
  }
  
  ///
  /// 插入 生成 样式文件的缓存
  ///
  pub fn add_render_cache(&mut self, filepath: &str, source: &str) {
    self
      .render_cache
      .lock()
      .unwrap()
      .insert(filepath.to_string(), source.to_string());
  }
  
  ///
  /// 清除本次 codegen 文件的记录
  ///
  pub fn clear_render_cache(&mut self) {
    self.render_cache.lock().unwrap().clear();
  }
  
  ///
  /// 获取 codegen 缓存 目标样式代码
  ///
  pub fn get_render_cache(&self, filepath: &str) -> Option<String> {
    self.render_cache.lock().unwrap().get(filepath).cloned()
  }
  
  ///
  /// 生成默认上下文
  ///
  pub fn default() -> ParseContext {
    Self::new(Default::default(), None).unwrap()
  }
  
  ///
  /// 递归恢复 json 上下文
  ///
  pub fn recovery_parse_object(&self, key: &str) -> Result<Option<StyleFileNode>, String> {
    let filecache = self.filecache.lock().unwrap();
    let json_res = filecache.get(key);
    if let Some(json) = json_res {
      let root: HashMap<String, Value> = serde_json::from_str(json).unwrap();
      return if let Some(Value::Object(map)) = root.get("info") {
        let node = self.deserializer(map)?;
        Ok(Some(node))
      } else {
        Err("info value is empty!".to_string())
      };
    }
    Ok(None)
  }
  
  ///
  /// 递归调用 json 反序列化 自制方法
  ///
  fn deserializer(&self, map: &Map<String, Value>) -> Result<StyleFileNode, String> {
    let json_disk_location = map.get("disk_location");
    let json_origin_txt_content = map.get("origin_txt_content");
    let disk_location = if let Some(Value::String(disk_location)) = json_disk_location {
      disk_location.to_string()
    } else {
      return Err("deserializer FileNode -> disk_location is empty!".to_string());
    };
    let ext_str_value = Path::new(disk_location.as_str())
      .extension()
      .unwrap()
      .to_str()
      .unwrap();
    let ext = ext_str_value
      .to_string()
      .to_enum::<StyleExtension>()
      .unwrap();
    let resolve_extension = ext;
    
    match resolve_extension {
      StyleExtension::Css => {
        let mut obj = crate::less::fileinfo::FileInfo {
          disk_location,
          block_node: vec![],
          origin_txt_content: "".to_string(),
          origin_charlist: vec![],
          locmap: None,
          context: self.weak_ref.as_ref().unwrap().upgrade().unwrap(),
          self_weak: None,
          import_files: vec![],
          resolve_extension,
        };
        if let Some(Value::String(origin_txt_content)) = json_origin_txt_content {
          obj.origin_txt_content = origin_txt_content.to_string();
          obj.origin_charlist = obj.origin_txt_content.to_char_vec();
        } else {
          return Err("deserializer FileNode -> origin_txt_content is empty!".to_string());
        }
        
        if self.option.sourcemap {
          obj.locmap = Some(crate::less::fileinfo::FileInfo::get_loc_by_content(
            &obj.origin_charlist,
          ));
        }
        let json_import_files = map.get("import_file");
        if let Some(Value::Array(disk_location)) = json_import_files {
          for json_item in disk_location {
            if let Value::Object(json_import_file_node) = json_item {
              let import_info = json_import_file_node
                .get("value")
                .unwrap()
                .as_object()
                .unwrap()
                .get("info")
                .unwrap()
                .as_object()
                .unwrap();
              let import_info_obj = self.deserializer(import_info)?;
              obj.import_files.push(import_info_obj);
            }
          }
        }
        
        let info = obj.toheap();
        let json_block_node = map.get("block_node");
        let mut block_node_recovery_list = vec![];
        if let Some(Value::Array(block_nodes)) = json_block_node {
          for json_node in block_nodes {
            if let Value::Object(json_stylenode) = json_node {
              block_node_recovery_list.push(crate::less::node::StyleNode::deserializer(
                json_stylenode,
                self.weak_ref.as_ref().unwrap().upgrade().unwrap().clone(),
                None,
                Some(Rc::downgrade(&info)),
              )?);
            }
          }
        }
        info.borrow_mut().block_node = block_node_recovery_list;
        let node = crate::less::filenode::FileNode { info };
        Ok(StyleFileNode::Less(node))
      }
      StyleExtension::Less => {
        let mut obj = crate::less::fileinfo::FileInfo {
          disk_location,
          block_node: vec![],
          origin_txt_content: "".to_string(),
          origin_charlist: vec![],
          locmap: None,
          context: self.weak_ref.as_ref().unwrap().upgrade().unwrap(),
          self_weak: None,
          import_files: vec![],
          resolve_extension,
        };
        if let Some(Value::String(origin_txt_content)) = json_origin_txt_content {
          obj.origin_txt_content = origin_txt_content.to_string();
          obj.origin_charlist = obj.origin_txt_content.to_char_vec();
        } else {
          return Err("deserializer FileNode -> origin_txt_content is empty!".to_string());
        }
        
        if self.option.sourcemap {
          obj.locmap = Some(crate::less::fileinfo::FileInfo::get_loc_by_content(
            &obj.origin_charlist,
          ));
        }
        let json_import_files = map.get("import_file");
        if let Some(Value::Array(disk_location)) = json_import_files {
          for json_item in disk_location {
            if let Value::Object(json_import_file_node) = json_item {
              let import_info = json_import_file_node
                .get("value")
                .unwrap()
                .as_object()
                .unwrap()
                .get("info")
                .unwrap()
                .as_object()
                .unwrap();
              obj.import_files.push(self.deserializer(import_info)?);
            }
          }
        }
        
        let info = obj.toheap();
        let json_block_node = map.get("block_node");
        let mut block_node_recovery_list = vec![];
        if let Some(Value::Array(block_nodes)) = json_block_node {
          for json_node in block_nodes {
            if let Value::Object(json_stylenode) = json_node {
              block_node_recovery_list.push(crate::less::node::StyleNode::deserializer(
                json_stylenode,
                self.weak_ref.as_ref().unwrap().upgrade().unwrap().clone(),
                None,
                Some(Rc::downgrade(&info)),
              )?);
            }
          }
        }
        info.borrow_mut().block_node = block_node_recovery_list;
        let node = crate::less::filenode::FileNode { info };
        Ok(StyleFileNode::Less(node))
      }
      StyleExtension::Sass => Err("*.sass has not support!".to_string()),
      StyleExtension::Scss => Err("*.scss has not support!".to_string()),
    }
  }
}
