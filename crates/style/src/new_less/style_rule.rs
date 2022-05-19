use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecCharExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::NodeWeakRef;
use crate::new_less::option::ParseOption;
use crate::new_less::scan::traversal;
use crate::new_less::token::lib::Token;
use crate::new_less::value::ValueNode;
use crate::new_less::var::HandleResult;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{Map, Value};
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

#[derive(Clone)]
pub struct StyleRuleNode {
  // 节点坐标
  pub loc: Option<Loc>,

  // 字符串 操作 序列
  charlist: Vec<char>,

  // uuid 避免 查找时循环引用
  pub uuid: String,

  // 内部处理 地图
  map: LocMap,

  // 文件信息
  pub fileinfo: FileWeakRef,

  // 节点 父节点
  pub parent: NodeWeakRef,

  // 上下文
  pub context: ParseContext,

  // 对应的 key 值
  pub key: Option<String>,

  // 对应 值
  pub value: Option<ValueNode>,
}

impl Serialize for StyleRuleNode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("StyleRuleNode", 5)?;
    state.serialize_field("content", &self.charlist.poly())?;
    state.serialize_field("loc", &self.loc)?;
    state.serialize_field("uuid", &self.uuid)?;
    state.serialize_field("key", &self.key)?;
    state.serialize_field("value", &self.value)?;
    state.end()
  }
}

impl Debug for StyleRuleNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("StyleRuleNode")
      .field("content", &self.charlist.poly())
      .field("loc", &self.loc)
      .field("uuid", &self.uuid)
      .field("key", &self.key)
      .field("value", &self.value)
      .finish()
  }
}

impl StyleRuleNode {
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
      charlist,
      uuid: Uuid::new_v4().to_string(),
      map,
      fileinfo,
      parent,
      context,
      key: None,
      value: None,
    };
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
      map: LocMap::new(&vec![]),
      charlist: vec![],
      parent: parent.as_ref().cloned(),
      fileinfo: fileinfo.as_ref().cloned(),
      key: None,
      value: None,
      context,
    };
    if let Some(Value::String(content)) = map.get("content") {
      obj.charlist = content.tocharlist();
    } else {
      return Err(format!(
        "deserializer VarNode has error -> content is empty!"
      ));
    }
    if let Some(Value::Object(loc)) = map.get("loc") {
      obj.loc = Some(Loc::deserializer(loc));
      obj.map = LocMap::merge(&obj.loc.as_ref().unwrap(), &obj.charlist).0;
    } else {
      obj.map = LocMap::new(&obj.charlist);
    }
    if let Some(Value::String(uuid)) = map.get("uuid") {
      obj.uuid = uuid.to_string();
    } else {
      return Err(format!("deserializer VarNode has error -> uuid is empty!"));
    }
    if let Some(Value::String(key)) = map.get("key") {
      obj.key = Some(key.to_string());
    } else {
      return Err(format!("deserializer VarNode has error -> key is empty!"));
    }
    if let Some(Value::Object(value_map)) = map.get("value") {
      obj.value = Some(ValueNode::deserializer(value_map, parent, fileinfo)?);
    }
    Ok(obj)
  }

  ///
  /// 获取选项
  ///
  pub fn get_options(&self) -> ParseOption {
    self.context.lock().unwrap().option.clone()
  }

  ///
  /// 判断是否是 顶层 节点 下的变量
  ///
  pub fn is_top(&self) -> bool {
    self.parent.is_none()
  }

  ///
  /// 报错信息
  ///
  pub fn error_msg(&self, index: &usize) -> String {
    let error_loc = self.map.get(index).unwrap();
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

    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, hasend) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if vec![':', '-'].contains(char) {
            if *char == ':' {
              *hasend = true;
            } else {
              temp.push(*char);
            }
          } else if Token::is_space_token(Some(char)) {
            if Token::is_space_token(next) {
              return Ok(());
            } else if next.is_some() && *next.unwrap() == ':' {
              temp.push(*char);
            } else {
              return Ok(());
            }
          } else {
            return Err(self.error_msg(index));
          }
        } else {
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
      self.map.get(start),
      self.parent.clone(),
      self.fileinfo.clone(),
    )?;
    Ok((node, self.charlist.len() - 1))
  }

  ///
  /// 转化校验
  ///
  fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    traversal(
      None,
      charlist,
      &mut (|arg, _| {
        let (index, _, _) = arg;
        if self.key.is_none() {
          let (key, jump) = self.parse_var_ident(index)?;
          *index = jump;
          self.key = Some(key);
        } else if self.value.is_none() {
          let (value, jump) = self.parse_var_value(index)?;
          *index = jump;
          self.value = Some(value);
        } else if self.value.is_some() && self.key.is_some() {
          return Err(self.error_msg(index));
        }
        Ok(())
      }),
    )?;

    Ok(())
  }

  ///
  /// 代码生成
  ///
  pub fn code_gen(&self) -> Result<String, String> {
    let res = match self.value.as_ref() {
      None => "".to_string(),
      Some(value) => value.code_gen()?,
    };
    let code_res = format!("{}: {};", self.key.as_ref().unwrap(), res);
    Ok(code_res)
  }
}
