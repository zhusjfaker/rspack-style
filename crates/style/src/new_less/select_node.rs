use crate::new_less::fileinfo::{FileRef, FileWeakRef};
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::media::MediaQuery;
use crate::new_less::node::{NodeRef, NodeWeakRef};
use crate::new_less::option::OptionExtend;
use crate::new_less::select::{NewSelector, SelectParadigm};
use crate::new_less::token::lib::TokenInterface;
use crate::new_less::token::select::TokenCombinaChar;
use crate::new_less::var::HandleResult;
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::HashSet;
use std::ops::Deref;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum SelectorNode {
  Select(NewSelector),
  Media(MediaQuery),
}

///
/// 创建 选择器 混合节点
///
impl SelectorNode {
  ///
  /// 初始化方法
  ///
  pub fn new(
    charlist: Vec<char>,
    loc: &mut Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Result<Self, String> {
    let mut map: Option<LocMap> = None;
    let start_loc = loc.as_ref().cloned();
    match parent.as_ref().unwrap().upgrade() {
      None => {}
      Some(p) => {
        if p.deref().borrow().get_options().sourcemap {
          let (calcmap, end) = LocMap::merge(loc.as_ref().unwrap(), &charlist);
          *loc = Some(end);
          map = Some(calcmap);
        }
      }
    }
    // 处理 media
    match MediaQuery::new(
      charlist.clone(),
      start_loc.clone(),
      map.clone(),
      parent.clone(),
    ) {
      HandleResult::Success(obj) => {
        return Ok(SelectorNode::Media(obj));
      }
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };

    let obj = NewSelector::new(charlist.clone(), start_loc.clone(), map, parent, fileinfo);
    Ok(SelectorNode::Select(obj))
  }

  ///
  /// 反序列化
  ///
  pub fn deserializer(
    map: &Map<String, Value>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Result<Self, String> {
    // 处理 select
    if parent.is_none() {
      return Err("SelectorNode -> parent must not be None!".to_string());
    }
    let value_type = map.get("type").unwrap().to_string();
    if value_type == r#""Select""# {
      // 处理引用
      let value_map = map.get("value").unwrap().as_object().unwrap();
      return Ok(SelectorNode::Select(NewSelector::deserializer(
        value_map, parent, fileinfo,
      )?));
    } else if value_type == r#""Media""# {
      // 处理变量
      let value_map = map.get("value").unwrap().as_object().unwrap();
      return Ok(SelectorNode::Media(MediaQuery::deserializer(
        value_map, parent,
      )?));
    }
    Err("SelectorNode -> noting type is matched".to_string())
  }

  pub fn value(&self) -> String {
    match self {
      SelectorNode::Select(obj) => obj.value(),
      SelectorNode::Media(obj) => obj.value(),
    }
  }

  ///
  /// 是否需要css_module
  /// 第一个 是否需要
  /// 第二个 content_hash
  ///
  pub fn need_css_modules(&self) -> (bool, Option<String>) {
    if let Some(file) = self.get_file() {
      if file.borrow().modules {
        return (true, Some(file.borrow().hash_perfix.clone()));
      }
    }
    (false, None)
  }

  ///
  /// 获取对应 FileInfo 节点
  ///
  pub fn get_file(&self) -> Option<FileRef> {
    let rule = self.get_rule();
    if let Some(rule_heap) = rule {
      if let Some(file) = rule_heap.borrow().file_info.as_ref() {
        Some(file.upgrade().unwrap())
      } else {
        None
      }
    } else {
      None
    }
  }

  ///
  /// 获取对应 StyleRule 完整节点
  ///
  pub fn get_rule(&self) -> Option<NodeRef> {
    if let SelectorNode::Select(ss) = &self {
      return if ss.parent.is_some() {
        Some(ss.parent.as_ref().unwrap().upgrade().unwrap())
      } else {
        None
      };
    } else if let SelectorNode::Media(mm) = &self {
      return if mm.parent.is_some() {
        Some(mm.parent.as_ref().unwrap().upgrade().unwrap())
      } else {
        None
      };
    }
    None
  }

  pub fn convert_class_paradigm(
    &self,
    word: String,
    hash_value: &str,
    map: &mut HashSet<String>,
  ) -> String {
    let class_word = word[1..word.len()].to_string();
    map.insert(class_word);
    format!("{}_{}", word, hash_value)
  }

  ///
  /// css_module 的 词缀 转化
  ///
  pub fn convert_paradigm_to_word(
    &self,
    list: &Vec<SelectParadigm>,
    hash_value: String,
    map: &mut HashSet<String>,
  ) -> Result<Vec<SelectParadigm>, String> {
    let mut new_list = vec![];
    let mut index = 0;
    let mut has_global = false;
    while index < list.len() {
      let par = list.get(index).unwrap();
      let nextpar = if index + 1 < list.len() {
        list.get(index + 1)
      } else {
        None
      };
      let prevpar = if index > 0 { list.get(index - 1) } else { None };
      match par {
        SelectParadigm::SelectWrap(ss) => {
          if ss == ":global" {
            if nextpar == Some(&SelectParadigm::CominaWrap(TokenCombinaChar::Space)) && !has_global
            {
              has_global = true;
              index += 1;
            }
          } else {
            if &ss[0..1] == "." && !has_global {
              // 转化 class 样式选择器
              let new_value = self.convert_class_paradigm(ss.to_string(), &hash_value, map);
              new_list.push(SelectParadigm::SelectWrap(new_value));
            } else if &ss[0..1] == "." && has_global {
              // 不转化 class 样式选择器
              new_list.push(par.clone());
            } else if &ss[0..1] == "("
              && &ss[ss.len() - 1..ss.len()] == ")"
              && prevpar == Some(&SelectParadigm::SelectWrap(":global".to_string()))
            {
              // 第一位'(' 最后一位是')'
              let new_value = ss[1..ss.len() - 1].to_string();
              new_list.push(SelectParadigm::SelectWrap(new_value));
            } else {
              new_list.push(par.clone())
            }
          }
        }
        SelectParadigm::CominaWrap(..) | SelectParadigm::KeyWrap(..) => new_list.push(par.clone()),
        _ => {
          return Err(format!(
            "{:#?} \n -> list_paradigm must not include SelectParadigm::VarWrap",
            list
          ));
        }
      }
      index += 1;
    }
    Ok(new_list)
  }

  ///
  /// 计算语义 合并计算
  ///
  pub fn calc_paradigm(
    &self,
    list: Vec<Vec<SelectParadigm>>,
    css_module_info: (bool, Option<String>),
    map: &mut HashSet<String>,
  ) -> Result<String, String> {
    let mut select_res = "".to_string();
    let (css_module, hash_value) = css_module_info;

    for (index, index_list) in list.iter().enumerate() {
      let mut txt = "".to_string();

      let calc_index_list = if css_module {
        self.convert_paradigm_to_word(index_list, hash_value.as_ref().unwrap().to_string(), map)?
      } else {
        index_list.to_owned()
      };

      for par in calc_index_list.iter() {
        match par {
          SelectParadigm::SelectWrap(ss) => {
            txt += &ss;
          }
          SelectParadigm::CominaWrap(cc) => txt += &cc.to_str().to_string(),
          SelectParadigm::KeyWrap(key) => {
            txt += &key;
          }
          _ => {
            return Err(format!(
              "{:#?} \n -> list_paradigm must not include SelectParadigm::VarWrap",
              list
            ));
          }
        }
      }
      select_res += &txt;
      if index != list.len() - 1 {
        select_res += ",";
      }
    }

    Ok(select_res)
  }

  ///
  /// 代码生成方法
  ///
  pub fn code_gen(&self, map: &mut HashSet<String>) -> Result<(String, String), String> {
    let mut node = None;
    let css_module_info = self.need_css_modules();
    if let Self::Select(ss) = &self {
      node = ss.parent.clone();
    } else if let Self::Media(ss) = &self {
      node = ss.parent.clone();
    }

    let nearly_select_node = NewSelector::find_up_select_node(node.clone());
    let nearly_media_node = MediaQuery::find_up_media_node(node.clone());

    if nearly_select_node.is_none() && nearly_media_node.is_none() {
      return Err(
        "codegen select_node -> nearly_select_node || nearly_media_node  one of them is not empty!"
          .to_string(),
      );
    }

    let mut select_res = "".to_string();
    if let Some(snode) = nearly_select_node {
      if let SelectorNode::Select(s) = snode.upgrade().unwrap().borrow().selector.as_ref().unwrap()
      {
        select_res = self.calc_paradigm(s.code_gen()?, css_module_info, map)?;
      }
    }

    let mut media_res = "".to_string();
    if let Some(mnode) = nearly_media_node {
      if let SelectorNode::Media(m) = mnode.upgrade().unwrap().borrow().selector.as_ref().unwrap() {
        media_res = m.code_gen().join(" and ");
      }
    }

    Ok((select_res, media_res))
  }
}
