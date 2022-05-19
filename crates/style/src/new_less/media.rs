use crate::extend::enum_extend::EnumExtend;
use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecCharExtend;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::NodeWeakRef;
use crate::new_less::scan::traversal;
use crate::new_less::select_node::SelectorNode;
use crate::new_less::token::lib::Token;
use crate::new_less::token::media::{TokenMediaFeature, TokenMediaLogic, TokenMediaType};
use crate::new_less::var::HandleResult;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{Map, Value};

///
/// 媒体查询
///
#[derive(Debug, Clone)]
pub struct MediaQuery {
  pub loc: Option<Loc>,

  map: LocMap,

  pub charlist: Vec<char>,

  pub parent: NodeWeakRef,
}

impl Serialize for MediaQuery {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("FileInfo", 2)?;
    state.serialize_field("loc", &self.loc)?;
    state.serialize_field("content", &self.charlist.poly())?;
    state.end()
  }
}

impl MediaQuery {
  ///
  /// 初始化方法
  ///
  pub fn new(
    charlist: Vec<char>,
    loc: Option<Loc>,
    map: Option<LocMap>,
    parent: NodeWeakRef,
  ) -> HandleResult<Self> {
    let obj = Self {
      loc,
      map: map.unwrap_or_else(|| LocMap::new(&charlist)),
      charlist,
      parent,
    };
    match obj.parse() {
      Ok(_) => HandleResult::Success(obj),
      Err(msg) => {
        if &msg == "select_txt not match media query" {
          HandleResult::Swtich
        } else {
          HandleResult::Fail(msg)
        }
      }
    }
  }

  ///
  /// 反序列化
  ///
  pub fn deserializer(map: &Map<String, Value>, parent: NodeWeakRef) -> Result<Self, String> {
    let mut media = Self {
      loc: None,
      map: LocMap::new(&vec![]),
      charlist: vec![],
      parent,
    };
    if let Some(Value::String(content)) = map.get("content") {
      media.charlist = content.tocharlist();
    } else {
      return Err(format!(
        "deserializer MediaQuery has error -> charlist is empty!"
      ));
    }
    if let Some(Value::Object(loc)) = map.get("loc") {
      media.loc = Some(Loc::deserializer(loc));
      media.map = LocMap::merge(media.loc.as_ref().unwrap(), &media.charlist).0;
    } else {
      media.map = LocMap::new(&media.charlist);
    }
    Ok(media)
  }

  ///
  /// 打印错误信息
  ///
  pub fn errormsg(&self, index: &usize) -> Result<(), String> {
    let char = *self.charlist.get(*index).unwrap();
    let error_loc = self.map.get(index).unwrap();
    Err(format!(
      "select text {}, char {} is not allow,line is {} col is {}",
      self.charlist.poly(),
      char,
      error_loc.line,
      error_loc.col
    ))
  }

  pub fn value(&self) -> String {
    self.charlist.poly()
  }

  ///
  /// 向上查找 最近 select 节点 非 media
  ///
  pub fn find_up_media_node(node: NodeWeakRef) -> NodeWeakRef {
    if let Some(ref heap_node) = node {
      let rule = heap_node.upgrade().unwrap();
      if matches!(
        *rule.borrow().selector.as_ref().unwrap(),
        SelectorNode::Media(..)
      ) {
        node.clone()
      } else {
        let parent = rule.borrow().parent.clone();
        Self::find_up_media_node(parent)
      }
    } else {
      None
    }
  }

  ///
  /// 生成当前 media 字符
  ///
  pub fn code_gen(&self) -> Vec<String> {
    let mut split_media_txt = vec![];

    // 计算父 表达式
    let self_rule = self.parent.as_ref().unwrap().upgrade().unwrap();
    let node = self_rule.borrow().parent.clone();
    let meida_rule_node = Self::find_up_media_node(node);
    if let Some(any_parent_rule) = meida_rule_node {
      let heap_any_parent_rule = any_parent_rule.upgrade().unwrap();
      if let Some(SelectorNode::Media(ps)) = heap_any_parent_rule.borrow().selector.as_ref() {
        split_media_txt = ps.code_gen()
      };
    }

    // 计算自己
    if split_media_txt.is_empty() {
      split_media_txt.push(self.charlist.poly());
    } else {
      split_media_txt.push(self.charlist.poly()[6..].to_string())
    }

    split_media_txt
  }

  ///
  /// 子转化 媒体功能 转化 key
  ///
  pub fn parse_media_feature_key(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, hasend) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if *char == ':' {
            if TokenMediaFeature::is(temp.poly().trim()) {
              // 加冒号之前 先判断是否是有效 key
              *hasend = true;
            } else {
              return Err(self.errormsg(index).err().unwrap());
            }
          } else if Token::is_space_token(Some(char)) {
            if Token::is_space_token(next) {
              return Ok(());
            } else {
              temp.push(*char);
            }
          } else if *char == '-' {
            temp.push('-');
          } else {
            return Err(self.errormsg(index).err().unwrap());
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
  /// 子转化 媒体功能 转化 value
  ///
  pub fn parse_media_value(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, hasend) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if *char == ')' {
            *hasend = true;
          } else if Token::is_space_token(Some(char)) {
            if Token::is_space_token(next) {
              return Ok(());
            } else {
              temp.push(*char);
            }
          } else if *char == '-' {
            if temp.is_empty() {
              temp.push('-');
            } else {
              return Err(self.errormsg(index).err().unwrap());
            }
          } else {
            return Err(self.errormsg(index).err().unwrap());
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
  /// 子转化 媒体功能
  ///
  pub fn parse_media_feature(&self, start: &usize) -> Result<(String, usize), String> {
    let mut index = *start + 1;
    let mut word_vec: Vec<String> = vec!["(".to_string()];

    // 分析key
    let (key, jump) = match self.parse_media_feature_key(&index.clone()) {
      Ok(res) => res,
      Err(msg) => {
        return Err(msg);
      }
    };
    word_vec.push(key);
    word_vec.push(":".to_string());
    index = jump + 1;

    // 分析value
    let (value, jump) = match self.parse_media_value(&index) {
      Ok(res) => res,
      Err(msg) => {
        return Err(msg);
      }
    };
    word_vec.push(value);
    word_vec.push(")".to_string());
    index = jump + 1;

    Ok((word_vec.join(""), index))
  }

  ///
  /// 转化 逻辑词
  ///
  pub fn parse_media_logicword(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let (word, jump) = match traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, hasend) = arg;
        let (_, char, _) = charword;
        if Token::is_token(Some(char)) {
          if Token::is_space_token(Some(char)) {
            *hasend = true;
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else {
          temp.push(*char);
        }
        Ok(())
      }),
    ) {
      Ok(res) => res,
      Err(msg) => {
        return Err(msg);
      }
    };

    if TokenMediaLogic::is(&word) || TokenMediaType::is(&word) {
      Ok((word, jump))
    } else {
      Err(self.errormsg(&jump).err().unwrap())
    }
  }

  pub fn parse(&self) -> Result<(), String> {
    let charlist = &self.charlist;
    if charlist.is_empty() {
      return Err("media query text is empty".to_string());
    }
    if charlist.len() < 6
      || (charlist.len() == 6 && charlist[0..6].to_vec().poly().as_str() != "@media")
      || (charlist.len() > 6 && charlist[0..7].to_vec().poly().as_str() != "@media ")
    {
      return Err("select_txt not match media query".to_string());
    }
    let mut word_vec = vec!["@media".to_string()];
    let index = 6;

    match traversal(
      Some(index),
      charlist,
      &mut (|arg, charword| {
        let (index, _, _) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if Token::is_space_token(Some(char)) {
            if !Token::is_space_token(next) {
              word_vec.push(" ".to_string());
              Ok(())
            } else {
              Ok(())
            }
          } else if vec!['(', ')', ':'].contains(char) {
            if '(' == *char {
              match self.parse_media_feature(index) {
                Ok((word, jump)) => {
                  word_vec.push(word);
                  *index = jump;
                  Ok(())
                }
                Err(msg) => Err(msg),
              }
            } else {
              Err(self.errormsg(index).err().unwrap())
            }
          } else {
            Err(self.errormsg(index).err().unwrap())
          }
        } else {
          let (word, jump) = match self.parse_media_logicword(index) {
            Ok(res) => res,
            Err(msg) => {
              return Err(msg);
            }
          };
          *index = jump;
          word_vec.push(word);
          word_vec.push(" ".to_string());
          Ok(())
        }
      }),
    ) {
      Ok(res) => res,
      Err(msg) => {
        return Err(msg);
      }
    };
    // println!("{:#?}", word_vec);
    Ok(())
  }
}
