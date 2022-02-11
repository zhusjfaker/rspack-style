use crate::extend::string::StringExtend;
use crate::new_less::comment::Comment;
use crate::new_less::file::cmd_path_resolve;
use crate::new_less::file_manger::FileManger;
use crate::new_less::loc::LocMap;
use crate::new_less::node::{NodeRef, StyleNode, StyleNodeJson};
use crate::new_less::option::ParseOption;
use crate::new_less::rule::Rule;
use crate::new_less::var::Var;
use derivative::Derivative;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct FileInfo {
  // 文件的磁盘位置
  pub disk_location: Option<std::string::String>,
  // 文件的原始内容
  pub origin_txt_content: String,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<String>,
  // 文件的 原始AST节点
  pub block_node: Vec<StyleNode>,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  pub locmap: Option<LocMap>,
  // 内部调用方式时 需要拿到对应的 转化配置
  pub option: ParseOption,
  // 当前引用链
  pub import_file: Vec<FileRef>,
  // 自身弱引用
  #[derivative(Debug = "ignore")]
  pub self_weak: FileWeakRef,
  // 转文件 的缓存
  #[derivative(Debug = "ignore")]
  pub filecache: ParseCacheMap,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileInfoJson {
  pub disk_location: Option<std::string::String>,
  pub block_node: Vec<StyleNodeJson>,
  pub import_file: Vec<FileInfoJson>,
}

pub type FileRef = Rc<RefCell<FileInfo>>;

pub type FileWeakRef = Option<Weak<RefCell<FileInfo>>>;

pub type ParseCacheMap = Rc<RefCell<HashMap<String, FileWeakRef>>>;

impl FileInfo {
  ///
  /// 转 json 标准化
  ///
  pub fn tojson(&self) -> FileInfoJson {
    let mut block_node = vec![];
    self
      .block_node
      .clone()
      .into_iter()
      .for_each(|node| match node {
        StyleNode::Comment(cc) => block_node.push(StyleNodeJson::Comment(cc)),
        StyleNode::Var(vv) => block_node.push(StyleNodeJson::Var(vv)),
        StyleNode::Rule(rule) => {
          let futex_rule = rule.deref().borrow().deref().clone().tojson();
          block_node.push(StyleNodeJson::Rule(futex_rule));
        }
      });
    let import_file = self
      .import_file
      .clone()
      .into_iter()
      .map(|x| x.borrow().tojson())
      .collect::<Vec<FileInfoJson>>();
    FileInfoJson {
      disk_location: self.disk_location.clone(),
      block_node,
      import_file,
    }
  }

  ///
  /// 转 heap 堆上对象
  ///
  pub fn toheap(self) -> FileRef {
    let heapobj = Rc::new(RefCell::new(self));
    heapobj.borrow_mut().self_weak = Some(Rc::downgrade(&heapobj));
    heapobj
  }

  ///
  /// 生成整个文件的 locmap 地图
  ///
  pub fn get_loc_by_content(content: &str) -> LocMap {
    LocMap::new(content.to_string())
  }

  ///
  /// 根据文件路径 转换 文件
  ///
  pub fn create_disklocation(filepath: String, option: ParseOption) -> Result<String, String> {
    let obj_heap = Self::create_disklocation_parse(filepath, option, None)?;
    let res = match obj_heap.deref().borrow().code_gen() {
      Ok(res) => Ok(res),
      Err(msg) => Err(msg),
    };
    res
  }

  ///
  /// 根据文件路径 解析 文件
  ///
  pub fn create_disklocation_parse(
    filepath: String,
    option: ParseOption,
    filecache: Option<ParseCacheMap>,
  ) -> Result<FileRef, String> {
    let abs_path: String;
    let text_content: String;
    let charlist: Vec<String>;
    let mut locmap: Option<LocMap> = None;
    let obj = match FileManger::resolve(filepath, option.include_path.clone()) {
      Ok((calc_path, content)) => {
        abs_path = calc_path;
        text_content = content.clone();
        if option.sourcemap {
          locmap = Some(FileInfo::get_loc_by_content(content.as_str()));
        }
        charlist = content.tocharlist();
        FileInfo {
          disk_location: Some(abs_path),
          block_node: vec![],
          origin_txt_content: text_content,
          origin_charlist: charlist,
          locmap,
          option,
          import_file: vec![],
          self_weak: None,
          filecache: filecache.unwrap_or_else(|| Rc::new(RefCell::new(HashMap::new()))),
        }
      }
      Err(msg) => {
        return Err(msg);
      }
    };
    let obj_heap = obj.toheap();
    Self::parse_heap(obj_heap.clone())?;
    Ok(obj_heap)
  }

  ///
  /// 根据文件内容 解析文件
  ///
  pub fn create_txt_content_parse(
    content: String,
    option: ParseOption,
    filename: Option<String>,
    filecache: Option<ParseCacheMap>,
  ) -> Result<FileRef, String> {
    let text_content: String = content.clone();
    let charlist: Vec<String> = text_content.tocharlist();
    let mut locmap: Option<LocMap> = None;
    if option.sourcemap {
      locmap = Some(FileInfo::get_loc_by_content(content.as_str()));
    }
    let abs_path = match filename {
      None => cmd_path_resolve("_virtual.less"),
      Some(path_val) => path_val,
    };
    let obj = FileInfo {
      disk_location: Some(abs_path),
      block_node: vec![],
      origin_txt_content: text_content,
      origin_charlist: charlist,
      locmap,
      option,
      import_file: vec![],
      self_weak: None,
      filecache: filecache.unwrap_or_else(|| Rc::new(RefCell::new(HashMap::new()))),
    };
    let obj_heap = obj.toheap();
    match Self::parse_heap(obj_heap.clone()) {
      Ok(_) => {}
      Err(msg) => {
        return Err(msg);
      }
    }
    Ok(obj_heap)
  }

  pub fn create_txt_content(
    content: String,
    option: ParseOption,
    filename: Option<String>,
  ) -> Result<String, String> {
    let obj = Self::create_txt_content_parse(content, option, filename, None)?;
    let res = match obj.deref().borrow().code_gen() {
      Ok(res) => Ok(res),
      Err(msg) => Err(msg),
    };
    res
  }

  pub fn parse_heap(obj: FileRef) -> Result<(), String> {
    // 把当前 节点 的 对象 指针 放到 节点上 缓存中
    let disk_location_path = obj.borrow().disk_location.clone().unwrap();

    obj
      .borrow()
      .set_cache(disk_location_path.as_str(), obj.borrow().self_weak.clone());
    // 开始转换
    let mut comments = match obj.borrow().parse_comment() {
      Ok(blocks) => blocks
        .into_iter()
        .map(StyleNode::Comment)
        .collect::<Vec<StyleNode>>(),
      Err(msg) => {
        return Err(msg);
      }
    };
    obj.borrow_mut().block_node.append(&mut comments);
    let mut vars = match obj.borrow().parse_var() {
      Ok(blocks) => blocks
        .into_iter()
        .map(StyleNode::Var)
        .collect::<Vec<StyleNode>>(),
      Err(msg) => {
        return Err(msg);
      }
    };
    obj.borrow_mut().block_node.append(&mut vars);
    let mut rules = match obj.borrow().parse_rule() {
      Ok(blocks) => blocks
        .into_iter()
        .map(StyleNode::Rule)
        .collect::<Vec<StyleNode>>(),
      Err(msg) => {
        return Err(msg);
      }
    };
    obj.borrow_mut().block_node.append(&mut rules);
    Ok(())
  }

  pub fn getrules(&self) -> Vec<NodeRef> {
    let mut list = vec![];

    self.block_node.iter().for_each(|x| {
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
    for item in self.getrules() {
      item.deref().borrow().code_gen(&mut res);
    }
    Ok(res)
  }

  ///
  /// 查询 缓存上 翻译结果
  ///
  pub fn get_cache(&self, file_path: &str) -> FileWeakRef {
    let map = self.filecache.deref().borrow();
    let res = map.get(file_path);
    res.map(|x| x.clone().as_ref().unwrap().clone())
  }

  ///
  /// 添加 缓存上 翻译结果
  ///
  pub fn set_cache(&self, file_path: &str, file_weak_ref: FileWeakRef) {
    let mut map = self.filecache.deref().borrow_mut();
    let res = map.get(file_path);
    if res.is_none() {
      map.insert(file_path.to_string(), file_weak_ref);
    }
  }
}
