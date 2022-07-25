use crate::less::fileinfo::{FileRef, FileWeakRef};
use crate::less::media::MediaQuery;
use crate::less::node::{NodeRef, NodeWeakRef};
use crate::less::select::{NewSelector, SelectParadigm};
use crate::less::var::HandleResult;
use crate::sourcemap::loc::{Loc, LocMap};
use crate::style_core::option::OptionExtend;
use crate::util::str_enum::EnumToChar;
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
    match MediaQuery::new(charlist.clone(), start_loc, map.clone(), parent.clone()) {
      HandleResult::Success(obj) => {
        return Ok(SelectorNode::Media(obj));
      }
      HandleResult::Fail(msg) => {
        return Err(msg);
      }
      HandleResult::Swtich => {}
    };

    let obj = NewSelector::new(charlist, start_loc, map, parent, fileinfo);
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
  /// 获取对应 FileInfo 节点
  ///
  pub fn get_file(&self) -> Option<FileRef> {
    let rule = self.get_rule();
    if let Some(rule_heap) = rule {
      rule_heap
        .borrow()
        .file_info
        .as_ref()
        .map(|file| file.upgrade().unwrap())
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
  /// 计算语义 合并计算
  ///
  pub fn calc_paradigm(
    &self,
    list: Vec<Vec<SelectParadigm>>,
  ) -> Result<String, String> {
    let mut select_res = "".to_string();

    for (index, index_list) in list.iter().enumerate() {
      let mut txt = "".to_string();
      let calc_index_list = index_list.to_owned();
      for par in calc_index_list.iter() {
        match par {
          SelectParadigm::SelectWrap(ss) => {
            txt += ss;
          }
          SelectParadigm::CominaWrap(cc) => txt += &cc.to_str().to_string(),
          SelectParadigm::KeyWrap(key) => {
            txt += key;
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
  pub fn code_gen(&self) -> Result<(String, String), String> {
    let mut node = None;
    if let Self::Select(ss) = &self {
      node = ss.parent.clone();
    } else if let Self::Media(ss) = &self {
      node = ss.parent.clone();
    }

    let nearly_select_node = NewSelector::find_up_select_node(node.clone());
    let nearly_media_node = MediaQuery::find_up_media_node(node);

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
        select_res = self.calc_paradigm(s.code_gen()?)?;
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
