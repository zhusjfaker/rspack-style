use crate::new_less::context::ParseContext;
use crate::new_less::file::{path_join, readfile};
use crate::new_less::filenode::FileNode;
use crate::new_less::loc::LocMap;
use crate::new_less::node::StyleNode;
use crate::new_less::var::VarRuleNode;
use crate::new_less::var_node::VarNode;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct FileInfo {
  // 文件的磁盘位置
  pub disk_location: String,
  // 文件的原始内容
  pub origin_txt_content: String,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<char>,
  // 文件的 原始AST节点
  pub block_node: Vec<StyleNode>,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  pub locmap: Option<LocMap>,
  // 全局上下文
  pub context: ParseContext,
  // 自身弱引用
  pub self_weak: FileWeakRef,
  // 该文件的引用文件
  pub import_files: Vec<FileNode>,
  // 是否 codegen 时需要处理 css_module
  pub modules: bool,
  // 处理 css 所有的 类选择器的 合集 已经去重
  pub class_selector_collect: HashSet<String>,
  // css_modules 需要增加的 hash 尾串
  pub hash_perfix: String,
}

pub type FileRef = Rc<RefCell<FileInfo>>;

pub type FileWeakRef = Option<Weak<RefCell<FileInfo>>>;

impl Serialize for FileInfo {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("FileInfo", 3)?;
    state.serialize_field("disk_location", &self.disk_location)?;
    state.serialize_field("origin_txt_content", &self.disk_location)?;
    state.serialize_field("block_node", &self.block_node)?;
    state.serialize_field("import_file", &self.import_files)?;
    state.end()
  }
}

impl Debug for FileInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("FileInfo")
      .field("disk_location", &self.disk_location)
      .field("block_node", &self.block_node)
      .field("import_file", &self.import_files)
      .finish()
  }
}

impl FileInfo {
  ///
  /// 转 heap 堆上对象
  ///
  pub fn toheap(self) -> FileRef {
    let heapobj = Rc::new(RefCell::new(self));
    heapobj.borrow_mut().self_weak = Some(Rc::downgrade(&heapobj));
    heapobj
  }

  ///
  /// 获取 某文件下 所有的 变量节点
  /// 递归 获取所有 fileinfo 上 block_node -> var 节点
  ///
  pub fn collect_vars(&self) -> Vec<VarNode> {
    let mut varlist = vec![];
    for filenode in &self.import_files {
      for item in &filenode.info.borrow().block_node {
        if let StyleNode::Var(VarRuleNode::Var(var)) = &item {
          varlist.push(var.clone());
        }
      }
      // 递归收集
      let mut child_var_list = filenode.info.borrow().collect_vars();
      varlist.append(&mut child_var_list)
    }
    varlist
  }

  ///
  /// 生成整个文件的 locmap 地图
  ///
  pub fn get_loc_by_content(chars: &[char]) -> LocMap {
    LocMap::new(chars)
  }

  ///
  /// 获取指定文件的路径
  /// 如果是路径 -> 直接返回该路径
  ///
  pub fn get_dir(path_value: &str) -> Result<String, String> {
    let path = Path::new(path_value);
    if path.is_file() {
      Ok(path.parent().unwrap().to_str().unwrap().to_string())
    } else if path.is_dir() {
      Ok(path_value.to_string())
    } else {
      Err(format!(
        "path type is file or dir please check {}",
        path_value
      ))
    }
  }

  ///
  /// 是否是相对路径
  ///
  pub fn is_relative_path(txt: &str) -> bool {
    let path = Path::new(txt);
    path.is_relative()
  }

  ///
  /// 文件查找对应解析路径
  /// 返回值 -> (路径, 文件内容)
  ///
  pub fn resolve(filepath: String, include_path: &Vec<String>) -> Result<(String, String), String> {
    // 相对路径 和 绝对路径 分开计算
    return if Self::is_relative_path(&filepath) {
      // 相对路径的情况
      let mut abs_path: Option<String> = None;
      let mut failpath = vec![];
      let mut content: Option<String> = None;
      for basepath in include_path {
        let temp_path = path_join(basepath.as_str(), filepath.as_str());
        match readfile(temp_path.as_str()) {
          Ok(res) => {
            content = Some(res);
            abs_path = Some(temp_path.clone());
            break;
          }
          Err(_) => failpath.push(temp_path.clone()),
        }
      }
      return if let Some(match_path) = abs_path {
        Ok((match_path, content.unwrap()))
      } else {
        Err(format!(
          "Nothings File is find in cmdpath and inculdepath,{}",
          failpath.join(";")
        ))
      };
    } else {
      // 绝对路径的情况
      let res = readfile(filepath.as_str())?;
      Ok((filepath.clone(), res))
    };
  }
}
