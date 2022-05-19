use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecCharExtend;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::ident::IdentType;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::NodeWeakRef;
use crate::new_less::scan::traversal;
use crate::new_less::token::lib::Token;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{Map, Value};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct ValueNode {
  // 字符 向量 只读
  pub charlist: Vec<char>,

  // rule 父节点
  pub parent: NodeWeakRef,

  // 文件节点
  pub fileinfo: FileWeakRef,

  // 内部处理 地图
  map: LocMap,

  // 起始位置
  pub loc: Option<Loc>,

  // 单词 范式
  pub word_ident_list: Vec<IdentType>,
}

impl Serialize for ValueNode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("ValueNode", 2)?;
    state.serialize_field("content", &self.charlist.poly())?;
    state.serialize_field("loc", &self.loc)?;
    state.serialize_field("ident", &self.word_ident_list)?;
    state.end()
  }
}

impl Debug for ValueNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ValueNode")
      .field("origin_txt", &self.charlist.poly())
      .field("ident", &self.word_ident_list)
      .finish()
  }
}

impl ValueNode {
  pub fn new(
    charlist: Vec<char>,
    loc: Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Result<Self, String> {
    let map = if loc.is_none() {
      LocMap::new(&charlist)
    } else {
      LocMap::merge(loc.as_ref().unwrap(), &charlist).0
    };
    let mut obj = Self {
      charlist,
      parent,
      fileinfo,
      map,
      loc,
      word_ident_list: vec![],
    };
    obj.parse()?;
    Ok(obj)
  }

  ///
  /// 反序列
  ///
  pub fn deserializer(
    map: &Map<String, Value>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Result<Self, String> {
    let mut obj = Self {
      charlist: vec![],
      parent,
      fileinfo,
      map: LocMap::new(&vec![]),
      loc: None,
      word_ident_list: vec![],
    };
    if let Some(Value::String(content)) = map.get("content") {
      obj.charlist = content.tocharlist();
    } else {
      return Err(format!(
        "deserializer ValueNode has error -> content is empty!"
      ));
    }
    if let Some(Value::Object(loc)) = map.get("loc") {
      obj.loc = Some(Loc::deserializer(loc));
      obj.map = LocMap::merge(&obj.loc.as_ref().unwrap(), &obj.charlist).0;
    } else {
      obj.map = LocMap::new(&obj.charlist);
    }
    if let Some(Value::Array(array_value)) = map.get("ident") {
      obj.word_ident_list = array_value
        .iter()
        .map(IdentType::deserializer)
        .collect::<Vec<IdentType>>()
    } else {
      return Err(format!(
        "deserializer ValueNode has error -> ident is empty!"
      ));
    }
    Ok(obj)
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
  /// 是否是数字
  ///
  pub fn is_number(char: Option<&char>) -> bool {
    if let Some(cc) = char {
      vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(cc)
    } else {
      false
    }
  }

  ///
  /// 是否是括号
  ///
  pub fn is_brackets(char: Option<&char>) -> bool {
    if let Some(cc) = char {
      vec!['(', ')', '[', ']', '{', '}'].contains(cc)
    } else {
      false
    }
  }

  pub fn is_end(char: Option<&char>, extend_char: Option<Vec<char>>) -> bool {
    if let Some(cc) = char {
      let mut char_list = vec![
        ';', '@', '~', '#', '$', '(', ')', '[', ']', '+', '*', '/', ',',
      ];
      if let Some(mut extend_list) = extend_char {
        char_list.append(&mut extend_list);
      }
      Token::is_space_token(Some(cc)) || char_list.contains(cc)
    } else {
      false
    }
  }

  ///
  /// 转化 常用词 颜色 关键词
  ///
  pub fn parse_value_word(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, hasend) = arg;
        let (_, char, nextchar) = charword;
        temp.push(*char);
        if *char == ':' {
          return Err(self.error_msg(index));
        }
        if Self::is_end(nextchar, None) {
          *hasend = true;
        }
        Ok(())
      }),
    )?;
    Ok(res)
  }

  ///
  /// 转化 引号词
  ///
  pub fn parse_value_string_const(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut keyword = '\0';

    let (value, end) = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, hasend) = arg;
        let (_, char, nextchar) = charword;

        if temp.is_empty() {
          if *char == '\'' || *char == '"' {
            keyword = *char;
            temp.push(*char);
          } else {
            return Err(self.error_msg(index));
          }
        } else {
          temp.push(*char);
        }

        if nextchar.is_some() && *nextchar.unwrap() == keyword && *char != '\\' {
          *hasend = true;
          temp.push(keyword);
          *index += 1;
        }

        Ok(())
      }),
    )?;
    // 最终检查
    if value.is_empty() {
      return Err(self.error_msg(start));
    }
    if (value.len() > 1
      && *value.tocharlist().get(0).unwrap() == keyword
      && *value.tocharlist().get(value.len() - 1).unwrap() != keyword)
      || value.len() == 1
    {
      return Err(format!("{} is not closure", self.charlist.poly()));
    }

    Ok((value, end))
  }

  ///
  /// 转化变量
  ///
  pub fn parse_value_var(&self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let res = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, hasend) = arg;
        let (_, char, nextchar) = charword;
        // 第一位必须是 @
        if temp.is_empty() && *char == '@' {
          temp.push('@');
          Ok(())
        } else if temp.is_empty() {
          Err(self.error_msg(index))
        } else {
          // 后续写词
          if Token::is_token(Some(char)) {
            if *char == '-' {
              if Token::is_token(nextchar) {
                *hasend = true;
                *index -= 1;
              } else if nextchar.is_some() {
                temp.push(*char);
              }
              // @- is error
              if temp.len() < 2 {
                return Err(self.error_msg(index));
              }
            } else if Self::is_end(Some(char), None) {
              // @+ @* is error
              if temp.len() < 2 {
                return Err(self.error_msg(index));
              }
              *hasend = true;
              *index -= 1;
            } else if *char == '\\' {
              temp.push(*char);
            } else {
              return Err(self.error_msg(index));
            }
          } else {
            temp.push(*char);
          }
          Ok(())
        }
      }),
    )?;
    Ok(res)
  }

  ///
  /// 向前查找
  /// 第一个非 space ident
  ///
  fn find_prev_no_space_ident(&self) -> Option<IdentType> {
    for item in self.word_ident_list.iter().rev() {
      if !item.is_space() {
        return Some(item.clone());
      }
    }
    None
  }

  ///
  /// 向后查找
  /// 第一个 非空字符串
  ///
  fn find_next_no_space_char(&self, mut index: usize) -> Option<char> {
    index += 1;
    while index < self.charlist.len() {
      let cur = self.charlist.get(index).unwrap();
      if !Token::is_space_token(Some(cur)) {
        return Some(*cur);
      }
      index += 1;
    }
    None
  }

  ///
  /// 转化 数值
  ///
  pub fn parse_value_number(
    &self,
    start: &usize,
  ) -> Result<((String, Option<String>), usize), String> {
    let charlist = &self.charlist;
    let mut value: String = "".to_string();
    let mut unit: String = "".to_string();
    let mut has_record_value = false;
    let mut has_single = false;

    let (_, end) = traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, _, hasend) = arg;
        let (prevchar, char, nextchar) = charword;
        if Token::is_token(Some(char)) {
          // 判断小数点的 情况
          if *char == '.' && !has_single && Self::is_number(prevchar) && Self::is_number(nextchar) {
            value.push(*char);
            has_single = true;
          } else if *char == '%' {
            unit.push(*char);
          } else if *char == '-' && value.is_empty() {
            // 考虑负数的情况
            value.push(*char);
          } else {
            return Err(self.error_msg(index));
          }
        } else if Self::is_number(Some(char)) {
          if !has_record_value {
            value.push(*char);
          } else {
            *index -= 1;
            *hasend = true;
          }
        } else {
          if value.is_empty() {
            return Err(self.error_msg(index));
          }
          if !has_record_value {
            has_record_value = true;
          }
          unit.push(*char);
        }
        // 判断是否完结
        if Self::is_end(nextchar, Some(vec!['-']))
          || (has_single && nextchar.is_some() && *nextchar.unwrap() == '.')
          || *char == '%'
        {
          *hasend = true;
        }

        Ok(())
      }),
    )?;
    if unit.is_empty() {
      Ok(((value, None), end))
    } else {
      Ok(((value, Some(unit)), end))
    }
  }

  ///
  /// 判断 是否 是 操作符
  ///
  fn is_operator(char: &char) -> bool {
    vec!['+', '-', '*', '/'].contains(char)
  }

  ///
  /// 检测 中小 括号 是否能够对齐
  ///
  fn validate_brackets() -> Box<dyn FnMut(&char) -> Result<Vec<char>, String>> {
    let mut brackets_vaildate: Vec<char> = vec![];
    Box::new(move |char: &char| {
      if vec!['(', ')', '[', ']', '\\'].contains(char) {
        if *char == ']' || *char == ')' {
          let last = brackets_vaildate.last();
          if let Some(last_char) = last {
            if (*last_char == '(' && *char == ')') || (*last_char == '[' && *char == ']') {
              brackets_vaildate.remove(brackets_vaildate.len() - 1);
            } else {
              return Err(format!(r#"{} is error "#, char));
            }
          } else {
            return Err(format!(r#"{} is error "#, char));
          }
        } else {
          brackets_vaildate.push(*char)
        }
      } else {
        return Err(format!(r#"{} is not '(' ')' '[' ']' "#, char));
      }
      Ok(brackets_vaildate.clone())
    })
  }

  ///
  /// 转化
  ///
  fn parse(&mut self) -> Result<(), String> {
    let charlist = &self.charlist.clone();
    if charlist.is_empty() {
      return Err("var declare text is empty".to_string());
    }
    let index: usize = 0;
    let mut validate_fn = Self::validate_brackets();
    let mut vaildate_res: Vec<char> = vec![];

    traversal(
      Some(index),
      charlist,
      &mut (|arg, charword| {
        let (index, _, _) = arg;
        let (prev, char, next) = charword;

        // 处理空格
        if Token::is_space_token(Some(char)) {
          match self.word_ident_list.last() {
            None => {}
            Some(val) => match val {
              IdentType::Space => {
                return Ok(());
              }
              _ => {
                self.word_ident_list.push(IdentType::Space);
              }
            },
          }
        } else if *char == '@' {
          let (var, end) = self.parse_value_var(index)?;
          self.word_ident_list.push(IdentType::Var(var));
          *index = end;
        }
        // 处理结尾词 ignore
        else if *char == ';' {
          return if *index == self.charlist.len() - 1 {
            Ok(())
          } else {
            Err(self.error_msg(index))
          };
        }
        // 处理prop
        else if *char == '$' {
          // todo! $ style_rule
          return Err(format!(
            "$ style_rule or ~ reference has not support \n {}",
            self.error_msg(index)
          ));
        } else if *char == '~' {
          self.word_ident_list.push(IdentType::Word('~'.to_string()));
        }
        // 处理 引用
        else if *char == '#' {
          let (color, end) = self.parse_value_word(index)?;
          self.word_ident_list.push(IdentType::Color(color));
          *index = end;
        }
        // 处理 keyword
        else if *char == '!' {
          let end = *index + 10;
          if self.charlist.len() >= end
            && &self.charlist[*index..end].to_vec().poly() == "!important"
          {
            self
              .word_ident_list
              .push(IdentType::KeyWord("!important".to_string()));
            *index += 10;
          } else {
            let (word, end) = self.parse_value_word(index)?;
            self.word_ident_list.push(IdentType::Word(word));
            *index = end;
          }
        }
        // 处理引号词
        else if *char == '"' || *char == '\'' {
          let (string_const, end) = self.parse_value_string_const(index)?;
          self
            .word_ident_list
            .push(IdentType::StringConst(string_const));
          *index = end;
        }
        // 处理括号
        else if vec!['(', ')', '[', ']', '\\'].contains(char) {
          if *char != '\\' {
            // parse rgb rgba
            match validate_fn(char) {
              Ok(res) => {
                vaildate_res = res;
                self
                  .word_ident_list
                  .push(IdentType::Brackets(char.to_string()));
              }
              Err(..) => {
                return Err(self.error_msg(index));
              }
            };
          } else {
            return Ok(());
          }
        }
        // 操作符
        else if Self::is_operator(char) {
          if *char == '-' && prev == Some(&' ') && Self::is_number(Some(next.unwrap_or(&' '))) {
            let ((val, unit), end) = self.parse_value_number(index)?;
            self.word_ident_list.push(IdentType::Number(val, unit));
            *index = end;
          } else {
            let last_item = self.find_prev_no_space_ident();
            let next_char_no_space = self.find_next_no_space_char(*index).unwrap();
            if last_item.is_some()
              && last_item.unwrap().is_number()
              && (Self::is_number(Some(&next_char_no_space))
                || Self::is_brackets(Some(&next_char_no_space)))
            {
              self
                .word_ident_list
                .push(IdentType::Operator(char.to_string()));
            } else {
              let (word, end) = self.parse_value_word(index)?;
              self.word_ident_list.push(IdentType::Word(word));
              *index = end;
            }
          }
        }
        // 处理 数值
        else if Self::is_number(Some(char)) {
          let ((val, unit), end) = self.parse_value_number(index)?;
          self.word_ident_list.push(IdentType::Number(val, unit));
          *index = end;
        }
        // 处理单词
        else {
          let (word, end) = self.parse_value_word(index)?;
          self.word_ident_list.push(IdentType::Word(word));
          *index = end;
        }
        Ok(())
      }),
    )?;

    // 括号有没有闭合的情况
    if !vaildate_res.is_empty() {
      return Err(format!(
        "{} contains unclosed parentheses -> {:#?}",
        &self.charlist.poly(),
        &vaildate_res
      ));
    }

    Ok(())
  }
}
