use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecCharExtend;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{NodeWeakRef, StyleNode};
use crate::new_less::scan::traversal;
use crate::new_less::select_node::SelectorNode;
use crate::new_less::token::lib::{Token, TokenInterface};
use crate::new_less::token::select::{
  TokenAllowChar, TokenCombinaChar, TokenKeyWordChar, TokenSelectChar,
};
use crate::new_less::value::ValueNode;
use crate::new_less::var::VarRuleNode;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Map, Value};
use std::cmp::Ordering;
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", content = "value")]
enum SelectVarText {
  Txt(String),
  Var(String),
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SelectParadigm {
  SelectWrap(String),
  CominaWrap(TokenCombinaChar),
  VarWrap(char),
  KeyWrap(String),
}

impl SelectParadigm {
  ///
  /// 反序列化
  ///
  pub fn deserializer(val: &Value) -> Self {
    serde_json::from_str(&serde_json::to_string(val).unwrap()).unwrap()
  }
}

pub trait Paradigm {
  ///
  /// 转成 样式词的 方法
  /// 简化版 -> 用于测试
  ///
  fn tostr(&self) -> String;
}

impl Paradigm for Vec<SelectParadigm> {
  fn tostr(&self) -> String {
    let mut txt = "".to_string();
    self.iter().for_each(|par| match par {
      SelectParadigm::SelectWrap(cc) => txt += cc,
      SelectParadigm::CominaWrap(cc) => {
        txt.push(cc.to_str());
      }
      SelectParadigm::VarWrap(cc) => txt.push(*cc),
      SelectParadigm::KeyWrap(cc) => {
        txt = cc.clone();
      }
    });
    txt
  }
}

#[derive(Debug, Clone)]
pub struct NewSelector {
  // 字符串规则 根据逗号分割
  pub paradigm_vec: Vec<Vec<SelectParadigm>>,

  // 坐标位置
  pub loc: Option<Loc>,

  // 内部处理 地图
  map: LocMap,

  // 字符串 操作 序列
  pub charlist: Vec<char>,

  // 节点 父节点
  // 延迟赋值
  pub parent: NodeWeakRef,

  // 文件节点
  pub fileinfo: FileWeakRef,
}

impl Serialize for NewSelector {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("FileInfo", 3)?;
    state.serialize_field("loc", &self.loc)?;
    state.serialize_field("content", &self.charlist.poly())?;
    state.serialize_field("paradigm_vec", &self.paradigm_vec)?;
    state.end()
  }
}

impl NewSelector {
  ///
  ///  初始化
  ///
  pub fn new(
    charlist: Vec<char>,
    loc: Option<Loc>,
    map: Option<LocMap>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Self {
    let obj = NewSelector {
      paradigm_vec: vec![],
      loc,
      map: map.unwrap_or_else(|| LocMap::new(&charlist)),
      charlist,
      parent,
      fileinfo,
    };
    obj
  }

  ///
  /// 向上查找 最近 select 节点 非 media
  ///
  pub fn find_up_select_node(node: NodeWeakRef) -> NodeWeakRef {
    if let Some(ref heap_node) = node {
      let rule = heap_node.upgrade().unwrap();
      if matches!(
        *rule.deref().borrow().selector.as_ref().unwrap(),
        SelectorNode::Select(..)
      ) {
        node.clone()
      } else {
        let parent = rule.deref().borrow().parent.clone();
        Self::find_up_select_node(parent)
      }
    } else {
      None
    }
  }

  ///
  /// 反序列化
  ///
  pub fn deserializer(
    map: &Map<String, Value>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Result<Self, String> {
    let mut select = Self {
      paradigm_vec: vec![],
      loc: None,
      map: LocMap::new(&vec![]),
      charlist: vec![],
      parent,
      fileinfo,
    };
    if let Some(Value::String(content)) = map.get("content") {
      select.charlist = content.tocharlist();
    } else {
      return Err(format!(
        "deserializer NewSelector has error -> charlist is empty!"
      ));
    }
    if let Some(Value::Object(loc)) = map.get("loc") {
      select.loc = Some(Loc::deserializer(loc));
      select.map = LocMap::merge(select.loc.as_ref().unwrap(), &select.charlist).0;
    } else {
      select.map = LocMap::new(&select.charlist);
    }
    if let Some(Value::Array(charlist)) = map.get("paradigm_vec") {
      for paradigm_vec in charlist {
        let mut list = vec![];
        if let Value::Array(paradigm) = paradigm_vec {
          list = paradigm.iter().map(SelectParadigm::deserializer).collect();
        }
        if !list.is_empty() {
          select.paradigm_vec.push(list);
        }
      }
    } else {
      return Err(format!(
        "deserializer NewSelector has error -> paradigm_vec is empty!"
      ));
    }
    Ok(select)
  }

  ///
  /// 尽量减少调用次数
  ///
  pub fn value(&self) -> String {
    self.charlist.poly()
  }

  ///
  /// 生成当前 select 字符
  ///
  pub fn code_gen(&self) -> Result<Vec<Vec<SelectParadigm>>, String> {
    let mut split_select_txt: Vec<Vec<SelectParadigm>> = vec![];

    for list in self.paradigm_vec.iter() {
      // 计算父 表达式
      let self_rule = self.parent.as_ref().unwrap().upgrade().unwrap();
      let node = self_rule.deref().borrow().parent.clone();
      let select_rule_node = Self::find_up_select_node(node);
      let mut parent_select_txt = vec![];
      if let Some(any_parent_rule) = select_rule_node {
        let heap_any_parent_rule = any_parent_rule.upgrade().unwrap();
        if let Some(SelectorNode::Select(ps)) =
          heap_any_parent_rule.deref().borrow().selector.as_ref()
        {
          parent_select_txt = ps.code_gen()?;
        };
      }

      // 清洗后 拼接自我结果
      let mut self_list = vec![];
      let mut has_var = false;

      // 计算自己
      let mut var_index = -1;
      for (np, p) in list.iter().enumerate() {
        if matches!(p, SelectParadigm::VarWrap(..)) {
          var_index = np as i32;
        } else if matches!(p, SelectParadigm::KeyWrap(..)) {
          has_var = true;
        }
      }

      if var_index > -1 {
        has_var = true;

        for expr in parent_select_txt.iter() {
          let mut cplist = list.clone();
          cplist.remove(var_index as usize);
          for item in expr.iter().rev() {
            cplist.insert(var_index as usize, item.clone())
          }
          self_list.push(cplist);
        }
      } else {
        self_list.push(list.clone());
      }

      if has_var || parent_select_txt.is_empty() {
        if !self_list.is_empty() {
          split_select_txt.append(&mut self_list);
        }
      } else {
        for expr in parent_select_txt.iter() {
          for m in self_list.iter() {
            let mut cp_expr = expr.clone();
            if cp_expr.last() != Some(&SelectParadigm::CominaWrap(TokenCombinaChar::Space)) {
              cp_expr.push(SelectParadigm::CominaWrap(TokenCombinaChar::Space));
            }
            cp_expr.append(&mut m.clone());
            split_select_txt.push(cp_expr);
          }
        }
      }
    }

    // 最终结果
    Ok(split_select_txt)
  }

  ///
  /// 打印错误信息
  ///
  pub fn errormsg(&self, index: &usize) -> Result<(), String> {
    let mut safe_index = *index;
    if *index > self.charlist.len() - 1 {
      safe_index = self.charlist.len() - 1;
    }
    let char = *self.charlist.get(safe_index).unwrap();
    let error_loc = self.map.get(&safe_index).unwrap();
    Err(format!(
      "select text {}, char {} is not allow, line is {} col is {}",
      self.charlist.poly(),
      char,
      error_loc.line,
      error_loc.col
    ))
  }

  ///
  /// 在二维数组 的最后 追加 词
  /// 需要在 连接词 前后适当 保持 1个 空格！
  /// 加入 & 前 如果有 唯一 连接符 则需要清除!
  /// > & .a {
  ///
  pub fn add_paradigm(&mut self, obj: SelectParadigm) {
    if self.paradigm_vec.is_empty() {
      if !matches!(obj, SelectParadigm::CominaWrap(..)) {
        self.paradigm_vec.push(vec![obj]);
      } else {
        self.paradigm_vec.push(vec![
          obj,
          SelectParadigm::CominaWrap(TokenCombinaChar::Space),
        ]);
      }
    } else {
      let list = self.paradigm_vec.last_mut().unwrap();
      if !matches!(obj, SelectParadigm::CominaWrap(..))
        || matches!(obj, SelectParadigm::CominaWrap(TokenCombinaChar::Space))
      {
        if matches!(obj, SelectParadigm::VarWrap(..))
          && matches!(list.get(0), Some(&SelectParadigm::CominaWrap(..)))
        {
          list.remove(0);
        }
        if list.is_empty() && !matches!(obj, SelectParadigm::CominaWrap(TokenCombinaChar::Space))
          || !list.is_empty()
        {
          list.push(obj);
        }
      } else {
        let last_paradigm = list.last();
        if last_paradigm.is_some()
          && !matches!(
            last_paradigm,
            Some(SelectParadigm::CominaWrap(TokenCombinaChar::Space))
          )
        {
          list.push(SelectParadigm::CominaWrap(TokenCombinaChar::Space));
        }
        list.push(obj);
        list.push(SelectParadigm::CominaWrap(TokenCombinaChar::Space));
      }
    }
  }

  ///
  /// 最后一组词 的 最后一位 非空
  /// 逗号情况 调用
  /// example ->  .a > {}  remove '>'
  ///
  pub fn clear_paraigm(&mut self, index: &usize) -> Result<(), String> {
    if let Some(list) = self.paradigm_vec.last_mut() {
      let mut rm_index_list: Vec<usize> = vec![];
      let mut num = 0;
      for (index, word) in list.iter().rev().enumerate() {
        match word {
          SelectParadigm::CominaWrap(_) => {
            rm_index_list.push(list.len() - 1 - index);
          }
          _ => {
            break;
          }
        }
      }
      for (_, val) in rm_index_list.into_iter().enumerate() {
        list.remove(val - num);
        num += 1;
      }
      if list.is_empty() {
        return Err(self.errormsg(index).err().unwrap());
      }
      Ok(())
    } else {
      Err(self.errormsg(index).err().unwrap())
    }
  }

  ///
  /// 在二维数组中 开辟 一组 新词 序列
  ///
  pub fn add_paradigm_vec(&mut self) {
    self.paradigm_vec.push(vec![]);
  }

  ///
  /// 获取 最后 词
  ///
  pub fn last_paradigm(&self) -> Option<&SelectParadigm> {
    if self.paradigm_vec.last().is_some() {
      return self.paradigm_vec.last().unwrap().last();
    }
    None
  }

  ///
  /// 获取 最后 非空格 词
  ///
  pub fn last_paradigm_without_space(&self) -> Option<&SelectParadigm> {
    if let Some(list) = self.paradigm_vec.last() {
      for p in list.iter().rev() {
        if *p != SelectParadigm::CominaWrap(TokenCombinaChar::Space) {
          return Some(p);
        } else {
          continue;
        }
      }
      None
    } else {
      None
    }
  }

  ///
  /// 是否 停词 的判断
  ///
  pub fn is_end(char: Option<&char>, extend_char: Option<Vec<char>>) -> bool {
    if let Some(cc) = char {
      let mut charlist: Vec<char> = vec![];
      if let Some(mut extend_list) = extend_char {
        charlist.append(&mut extend_list);
      }
      charlist.append(
        &mut TokenSelectChar::iterator()
          .map(|x| x.to_str())
          .collect::<Vec<char>>(),
      );
      charlist.append(
        &mut TokenCombinaChar::iterator()
          .map(|x| x.to_str())
          .collect::<Vec<char>>(),
      );
      charlist.append(
        &mut TokenKeyWordChar::iterator()
          .map(|x| x.to_str())
          .collect::<Vec<char>>(),
      );
      charlist.contains(cc)
    } else {
      false
    }
  }

  ///
  /// parse select txt
  /// https://www.w3schools.com/cssref/css_selectors.asp
  ///
  pub fn parse(&mut self, parent_node: NodeWeakRef) -> Result<(), String> {
    let index: usize = 0;
    // 先判断 可以减少工作量调用
    if self.charlist.contains(&'@') {
      self.pure_select_txt(parent_node)?;
    }
    let charlist = &self.charlist.clone();

    let (_, end) = traversal(
      Some(index),
      charlist,
      &mut (|arg, charword| {
        let (index, _, _) = arg;
        let (_, char, _) = charword;

        if Token::is_token(Some(char)) {
          if TokenSelectChar::is(char) {
            // example a, li , h2
            let (select_word, end) = self.parse_selector_word(index)?;
            self.add_paradigm(SelectParadigm::SelectWrap(select_word));
            *index = end;
          } else if TokenCombinaChar::is(char) {
            let (_, end) = self.parse_combina_word(index)?;
            *index = end;
          } else if TokenKeyWordChar::is(char) {
            if *char == '&' {
              self.add_paradigm(SelectParadigm::VarWrap(*char));
            } else {
              // @ -> @keyframes @{abc}
              let (key_word, end) = self.parse_at_keyword(index)?;
              self.add_paradigm(SelectParadigm::KeyWrap(key_word));
              *index = end;
            }
          } else if TokenAllowChar::is(char) {
            // example a, li , h2
            let (select_word, end) = self.parse_selector_word(index)?;
            self.add_paradigm(SelectParadigm::SelectWrap(select_word));
            *index = end;
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else {
          // example a, li , h2
          let (select_word, end) = self.parse_selector_word(index)?;
          self.add_paradigm(SelectParadigm::SelectWrap(select_word));
          *index = end;
        }
        Ok(())
      }),
    )?;
    self.clear_paraigm(&end)?;
    Ok(())
  }

  ///
  /// parse @
  /// example -> @keyframes identifierb || @font-face
  ///
  fn parse_at_keyword(&mut self, index: &usize) -> Result<(String, usize), String> {
    traversal(
      Some(*index),
      &self.charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if *char == '@' {
            if temp.is_empty() {
              temp.push(*char);
            } else {
              return Err(self.errormsg(index).err().unwrap());
            }
          } else if TokenAllowChar::is(char) {
            temp.push(*char);
          } else if Token::is_space_token(Some(char)) {
            temp.push(*char);
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else {
          temp.push(*char);
        }
        if next.is_none() {
          *end = true;
        }
        Ok(())
      }),
    )
  }

  ///
  /// 连接词的处理
  ///
  fn parse_combina_word(&mut self, index: &usize) -> Result<(String, usize), String> {
    let char = *self.charlist.get(*index).unwrap();
    if Token::is_space_token(Some(&char)) {
      let last_pardigm = self.last_paradigm();
      if let Some(SelectParadigm::CominaWrap(token)) = last_pardigm {
        if *token != TokenCombinaChar::Space {
          self.add_paradigm(SelectParadigm::CominaWrap(TokenCombinaChar::Space));
        }
      } else {
        self.add_paradigm(SelectParadigm::CominaWrap(TokenCombinaChar::Space));
      }
    } else if char == TokenCombinaChar::AddChar.to_str()
      || char == TokenCombinaChar::ExtendChar.to_str()
      || char == TokenCombinaChar::BrotherMatchChar.to_str()
      || char == TokenCombinaChar::ColumnChar.to_str()
    {
      let last_pardigm = self.last_paradigm_without_space();
      // 只要最后一个非空字符是 非链接符 即可
      if !matches!(last_pardigm, Some(SelectParadigm::CominaWrap(..))) {
        let combin_token = TokenCombinaChar::get(&char).unwrap();
        self.add_paradigm(SelectParadigm::CominaWrap(combin_token));
      } else {
        return Err(self.errormsg(index).err().unwrap());
      }
    } else if char == TokenCombinaChar::Comma.to_str() {
      self.clear_paraigm(index)?;
      self.add_paradigm_vec();
    }
    Ok((char.to_string(), *index))
  }

  ///
  /// parse select word
  ///
  fn parse_selector_word(&mut self, start: &usize) -> Result<(String, usize), String> {
    let res: (String, usize);
    let charlist = &self.charlist;
    let char = charlist.get(*start).unwrap();
    if *char == '.' || *char == '#' {
      res = self.parse_selector_class_or_id_word(start)?;
    } else if *char == ':' {
      res = self.parse_selector_pseudo_class_word(start)?;
    } else if *char == '*' {
      res = ('*'.to_string(), *start);
    } else if *char == '[' {
      res = self.parse_selector_attr(start)?;
    } else if *char == '(' {
      res = self.parse_selector_brackets(start)?;
    } else if !Token::is_token(Some(char)) || TokenAllowChar::is(char) {
      res = self.parse_selector_ele(start)?;
    } else {
      return Err(self.errormsg(start).err().unwrap());
    }
    Ok(res)
  }

  ///
  /// parse example h2 span div
  ///
  fn parse_selector_ele(&mut self, start: &usize) -> Result<(String, usize), String> {
    traversal(
      Some(*start),
      &self.charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if TokenAllowChar::is(char) {
            temp.push(*char);
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else {
          temp.push(*char);
        }
        if next.is_none() || Self::is_end(next, None) {
          *end = true;
        }
        Ok(())
      }),
    )
  }

  ///
  /// parse example #h2 .abc
  ///
  fn parse_selector_class_or_id_word(&mut self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut record = false;
    traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if TokenAllowChar::is(char) {
            temp.push(*char);
          } else if TokenSelectChar::is(char) && !record {
            record = true;
            temp.push(*char);
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else {
          temp.push(*char);
        }
        // 执行结束检查
        if (next.is_none() || Self::is_end(next, None)) && record {
          if temp.len() < 2 {
            // . , # single word is error
            return Err(self.errormsg(index).err().unwrap());
          }
          *end = true;
        }
        Ok(())
      }),
    )
  }

  ///
  /// parse example ::hide :next
  ///
  fn parse_selector_pseudo_class_word(&mut self, start: &usize) -> Result<(String, usize), String> {
    let charlist = &self.charlist;
    let mut record = false;
    traversal(
      Some(*start),
      charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (_, char, next) = charword;
        let mut next_fix = next;
        if Token::is_token(Some(char)) {
          if !record && *char == ':' {
            if next == Some(&':') {
              temp.push(*char);
              *index += 1;
              next_fix = {
                if *index + 1 < charlist.len() {
                  charlist.get(*index + 1)
                } else {
                  None
                }
              };
            }
            temp.push(*char);
            record = true;
          } else if TokenAllowChar::is(char) {
            temp.push(*char);
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else {
          temp.push(*char);
        }
        // 执行结束检查
        if (next.is_none() || Self::is_end(next_fix, None)) && record {
          if (temp.len() < 2 && *temp == vec![':']) || (temp.len() < 3 && *temp == vec![':', ':']) {
            // . , # single word is error
            return Err(self.errormsg(index).err().unwrap());
          }
          *end = true;
        }
        Ok(())
      }),
    )
  }

  ///
  /// parse example (language)
  ///
  fn parse_selector_brackets(&mut self, start: &usize) -> Result<(String, usize), String> {
    let mut level = 0;
    let res = traversal(
      Some(*start),
      &self.charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (_, char, next) = charword;
        if Token::is_token(Some(char)) {
          if TokenAllowChar::is(char) {
            temp.push(*char);
            return Ok(());
          }
          if *char == ')' {
            level -= 1;
            match level.cmp(&0) {
              Ordering::Less => {
                return Err(self.errormsg(index).err().unwrap());
              }
              Ordering::Equal => {
                *end = true;
                temp.push(*char);
                return Ok(());
              }
              Ordering::Greater => {
                temp.push(*char);
              }
            }
          } else if *char == '@' && next != Some(&'{') {
            return Err(self.errormsg(index).err().unwrap());
          } else if *char == '(' {
            level += 1;
            temp.push(*char);
          } else {
            temp.push(*char);
          }
        } else {
          temp.push(*char);
        }
        Ok(())
      }),
    )?;
    if level > 0 {
      return Err(self.errormsg(&res.1).err().unwrap());
    }
    Ok(res)
  }

  ///
  /// parse example '[arco-theme='dark']'
  ///
  fn parse_selector_attr(&mut self, start: &usize) -> Result<(String, usize), String> {
    // example ~= *= ^=
    let equal_chars = vec!['~', '|', '^', '$', '*'];
    let mut hasequal = false;
    let mut has_brackest = false;
    let mut queto: Option<char> = None;

    let res = traversal(
      Some(*start),
      &self.charlist,
      &mut (|arg, charword| {
        let (index, temp, end) = arg;
        let (prev, char, next) = charword;
        let get_prev_char_without_space = || -> Option<char> {
          let mut res: Option<char> = None;
          for tc in temp.iter().rev() {
            if *tc == ' ' {
              continue;
            } else {
              res = Some(*tc);
              break;
            }
          }
          res
        };

        if let Some(token_queto) = queto {
          if has_brackest {
            if *char == token_queto && prev != Some(&'\\') {
              queto = None;
            }
            temp.push(*char);
          } else {
            return Err(self.errormsg(index).err().unwrap());
          }
        } else if Token::is_token(Some(char)) {
          if has_brackest {
            if *char == '[' {
              return Err(self.errormsg(index).err().unwrap());
            } else if *char == ']' {
              let prev_char_without_space = get_prev_char_without_space();
              if prev_char_without_space == Some('=') {
                return Err(self.errormsg(index).err().unwrap());
              }
              // is attr end record
              temp.push(*char);
              has_brackest = false;
              *end = true;
            } else if TokenAllowChar::is(char) {
              temp.push(*char);
            } else if equal_chars.contains(char) {
              if next == Some(&'=') && !hasequal {
                temp.push(*char);
                temp.push('=');
                hasequal = true;
                *index += 1;
              } else {
                return Err(self.errormsg(index).err().unwrap());
              }
            } else if *char == '=' {
              let prev_char_without_space = get_prev_char_without_space();
              if prev_char_without_space == Some('[') {
                return Err(self.errormsg(index).err().unwrap());
              }
              if !hasequal {
                temp.push('=');
                hasequal = true;
              } else {
                return Err(self.errormsg(index).err().unwrap());
              }
            } else if *char == '"' || *char == '\'' {
              queto = Some(*char);
              temp.push(*char);
            } else {
              return Err(self.errormsg(index).err().unwrap());
            }
          } else {
            // start record attr
            if *char == '[' {
              temp.push(*char);
              has_brackest = true;
            } else {
              return Err(self.errormsg(index).err().unwrap());
            }
          }
        } else if has_brackest {
          temp.push(*char);
        } else {
          return Err(self.errormsg(index).err().unwrap());
        }
        Ok(())
      }),
    )?;
    if has_brackest {
      return Err(self.errormsg(&res.1).err().unwrap());
    }
    Ok(res)
  }

  ///
  /// support var in select_txt
  /// like @{abc} .a{  .... }
  ///
  fn pure_select_txt(&mut self, parent_node: NodeWeakRef) -> Result<(), String> {
    let mut record = false;
    let mut list: Vec<SelectVarText> = vec![];
    traversal(
      None,
      &self.charlist,
      &mut (|arg, charword| {
        let (index, temp, _end) = arg;
        let (_prev, char, next) = charword;
        if *char == '@' && next == Some(&'{') {
          if temp.len() > 0 {
            list.push(SelectVarText::Txt(temp.poly()));
          }
          temp.clear();
          temp.push('@');
          temp.push('{');
          *index += 1;
          record = true
        } else if *char == '}' && record {
          temp.push(*char);
          if temp.len() > 0 {
            list.push(SelectVarText::Var(temp.poly()));
          } else {
            return Err(format!(
              "select txt {} index is {} -> @ var is not closure",
              self.charlist.poly(),
              *index
            ));
          }
          temp.clear();
          record = false;
        } else {
          temp.push(*char);
        }
        Ok(())
      }),
    )?;

    let mut new_content = "".to_string();
    if list.len() > 0 {
      for tt in list {
        if let SelectVarText::Txt(t) = tt {
          new_content += &t;
        } else if let SelectVarText::Var(v) = tt {
          let val = v.tocharlist()[2..v.len() - 1].to_vec().poly();
          let var_ident = format!("@{}", val);
          let var_node_value = self.get_var_by_key(
            var_ident.as_str(),
            parent_node.clone(),
            self.fileinfo.clone(),
          )?;

          new_content += &var_node_value.code_gen()?;
        }
      }
      self.charlist = new_content.tocharlist();
    }

    Ok(())
  }

  ///
  /// 查找变量
  /// 用于 (变量计算)
  ///
  pub fn get_var_by_key(
    &self,
    key: &str,
    rule_info: NodeWeakRef,
    file_info: FileWeakRef,
  ) -> Result<ValueNode, String> {
    if let Some(rule_ref) = rule_info {
      let rule = rule_ref.upgrade().unwrap();
      let nodelist = &rule.deref().borrow().block_node;
      for item in nodelist {
        if let StyleNode::Var(VarRuleNode::Var(var)) = item.deref() {
          if var.key.as_ref().unwrap() == key {
            return Ok(var.value.as_ref().unwrap().clone());
          }
        }
      }
      return if rule.deref().borrow().parent.is_some() {
        // 非顶层 向上递归
        self.get_var_by_key(key, rule.deref().borrow().parent.clone(), None)
      } else {
        // 顶层 同层 引用递归 查看下半段代码
        self.get_var_by_key(key, None, self.fileinfo.clone())
      };
    }
    // 到达顶层后 取当前文件 的 顶层变量 或者 其他引用 文件的 顶层变量
    else if let Some(file_ref) = file_info {
      // 若没有则已经到达 顶层 则按照 顶层处理
      let fileinfo_ref = file_ref.upgrade().unwrap();
      let nodelist = &fileinfo_ref.deref().borrow().block_node;
      for item in nodelist {
        if let StyleNode::Var(VarRuleNode::Var(var)) = item.deref() {
          if var.key.as_ref().unwrap() == key {
            return Ok(var.value.as_ref().unwrap().clone());
          }
        }
      }
      // 获取 其他 引用文件 顶层变量
      let top_level_other_vars = fileinfo_ref.deref().borrow().collect_vars();
      for var in top_level_other_vars {
        if var.key.as_ref().unwrap() == key {
          return Ok(var.value.as_ref().unwrap().clone());
        }
      }
    };

    Err(format!("no var key {} has found", key))
  }
}
