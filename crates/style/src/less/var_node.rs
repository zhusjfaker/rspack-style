use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecCharExtend;
use crate::less::fileinfo::FileWeakRef;
use crate::less::node::NodeWeakRef;
use crate::less::value::ValueNode;
use crate::less::var::HandleResult;
use crate::sourcemap::loc::{Loc, LocMap};
use crate::style_core::context::ParseContext;
use crate::style_core::option::ParseOption;
use crate::style_core::scan::traversal;
use crate::token::lib::Token;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{Map, Value};
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

#[derive(Clone)]
pub struct VarNode {
  // 节点坐标
  pub loc: Option<Loc>,

  // uuid 避免 查找时循环引用
  pub uuid: String,

  // 内部处理 地图
  map: LocMap,

  // 字符串 操作 序列
  charlist: Vec<char>,

  // 节点 父节点
  pub parent: NodeWeakRef,

  // 文件信息
  pub fileinfo: FileWeakRef,

  pub key: Option<String>,

  pub value: Option<ValueNode>,

  // 上下文
  pub context: ParseContext,
}

impl Serialize for VarNode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("VarNode", 5)?;
    state.serialize_field("content", &self.charlist.poly())?;
    state.serialize_field("loc", &self.loc)?;
    state.serialize_field("uuid", &self.uuid)?;
    state.serialize_field("key", &self.key)?;
    state.serialize_field("value", &self.value)?;
    state.end()
  }
}

impl Debug for VarNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("VarNode")
      .field("content", &self.charlist.poly())
      .field("loc", &self.loc)
      .field("uuid", &self.uuid)
      .field("key", &self.key)
      .field("value", &self.value)
      .finish()
  }
}

impl VarNode {
  ///
  /// 初始化
  ///
  pub fn new(
    charlist: Vec<char>,
    loc: Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
    context: ParseContext,
  ) -> HandleResult<Self> {
    let map = if loc.is_none() {
      LocMap::new(&charlist)
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &charlist).0
    };
    let mut obj = Self {
      loc,
      uuid: Uuid::new_v4().to_string(),
      map,
      charlist,
      parent,
      fileinfo,
      key: None,
      value: None,
      context,
    };
    // HandleResult::Success(obj)
    match obj.parse() {
      Ok(_) => HandleResult::Success(obj),
      Err(msg) => HandleResult::Fail(msg),
    }
  }

  ///
  /// 反序列
  ///
  pub fn deserializer(
    map: &Map<String, Value>,
    context: ParseContext,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Result<Self, String> {
    let mut obj = Self {
      loc: None,
      uuid: "".to_string(),
      map: LocMap::new(&[]),
      charlist: vec![],
      parent: parent.as_ref().cloned(),
      fileinfo: fileinfo.as_ref().cloned(),
      key: None,
      value: None,
      context,
    };
    if let Some(Value::String(content)) = map.get("content") {
      obj.charlist = content.to_char_vec();
    } else {
      return Err("deserializer VarNode has error -> content is empty!".to_string());
    }
    if let Some(Value::Object(loc)) = map.get("loc") {
      obj.loc = Some(Loc::deserializer(loc));
      obj.map = LocMap::merge(obj.loc.as_ref().unwrap(), &obj.charlist).0;
    } else {
      obj.map = LocMap::new(&obj.charlist);
    }
    if let Some(Value::String(uuid)) = map.get("uuid") {
      obj.uuid = uuid.to_string();
    } else {
      return Err("deserializer VarNode has error -> uuid is empty!".to_string());
    }
    if let Some(Value::String(key)) = map.get("key") {
      obj.key = Some(key.to_string());
    } else {
      return Err("deserializer VarNode has error -> key is empty!".to_string());
    }
    if let Some(Value::Object(value_map)) = map.get("value") {
      obj.value = Some(ValueNode::deserializer(value_map, parent, fileinfo)?);
    }
    Ok(obj)
  }

  ///
  /// 判断是否是 顶层 节点 下的变量
  ///
  pub fn is_top(&self) -> bool {
    self.parent.is_none()
  }

  ///
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    self.context.lock().unwrap().option.clone()
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(*index).unwrap();
    let char = self.charlist.get(*index).unwrap().to_string();
    format!(
      "text {}, char {} is not allow, line is {} col is {}",
      &self.charlist.poly(),
      char,
      error_loc.line,
      error_loc.col
    )
  }

  ///
  /// 转化变量声明 key
  ///
  pub fn parse_var_ident(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut has_space = false;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, has_end) = arg;
        let (_, char, next) = charword;
        // 变量声明 只允许 冒号前后有空格
        let is_current_char_space_token = Token::is_space_token(Some(char));
        if has_space && Token::is_space_token(next) {
          return Ok(());
        } else if has_space && !is_current_char_space_token {
          if *char == ':' {
            temp.push(*char);
          } else {
            return Err(self.error_msg(&(*index - 1)));
          }
        } else if !has_space && Token::is_token(Some(char)) {
          if *char == ':' {
            *has_end = true;
          } else if *char == '-' {
            temp.push(*char);
          } else if is_current_char_space_token {
            has_space = true;
            temp.push(*char);
          } else {
            return Err(self.error_msg(index));
          }
        } else if !has_space && !Token::is_token(Some(char)) {
          temp.push(*char);
        }
        Ok(())
      }),
    )?;
    Ok(res)
  }

  ///
  /// 转化变量声明 value
  ///
  pub fn parse_var_value(&self, start: &usize) -> Result<(ValueNode, usize), String> {
    // 取分号前一位 最后一定是分号
    let end = self.charlist.len() - 1;
    let mut trim_start = *start;
    while trim_start < self.charlist.len() {
      if !Token::is_space_token(Some(self.charlist.get(trim_start).unwrap())) {
        break;
      }
      trim_start += 1;
    }
    let node = ValueNode::new(
      self.charlist[trim_start..end].to_vec(),
      self.map.get(*start),
      self.parent.clone(),
      self.fileinfo.clone(),
    )?;
    Ok((node, self.charlist.len() - 1))
  }

  ///
  /// 转化校验
  ///
  fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist;
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    let index = 1;
    let mut obj_key: Option<String> = None;
    let mut obj_value: Option<ValueNode> = None;

    match traversal(
      Some(index),
      charlist,
      &mut (|arg, _| {
        let (index, _, _) = arg;
        if obj_key.is_none() {
          let (key, jump) = self.parse_var_ident(index)?;
          *index = jump;
          obj_key = Some("@".to_string() + &key);
        } else if obj_value.is_none() {
          let (value, jump) = self.parse_var_value(index)?;
          *index = jump;
          obj_value = Some(value);
        } else if obj_key.is_some() && obj_value.is_some() {
          return Err(self.error_msg(index));
        }
        Ok(())
      }),
    ) {
      Ok(_) => {
        self.key = obj_key;
        self.value = obj_value;
      }
      Err(msg) => {
        return Err(msg);
      }
    };

    Ok(())
  }
}
