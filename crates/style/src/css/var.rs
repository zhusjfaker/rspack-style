use crate::css::fileinfo::{FileRef, FileWeakRef};
use crate::css::import::ImportNode;
use crate::css::node::NodeWeakRef;
use crate::css::style_rule::StyleRuleNode;
use crate::extend::vec_str::VecCharExtend;
use crate::sourcemap::loc::Loc;
use crate::style_core::context::ParseContext;
use serde::Serialize;
use serde_json::{Map, Value};

///
/// 处理类型
///
pub enum HandleResult<T> {
  /// 匹配成功 且 处理成功
  Success(T),

  /// 匹配成功 且 处理失败
  Fail(String),

  /// 匹配失败
  Swtich,
}

///
/// 变量内容
///
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum VarRuleNode {
  /// 引用
  Import(ImportNode),

  /// 样式规则
  StyleRule(StyleRuleNode),
}

///
/// 联合 节点 声明
///
impl VarRuleNode {
  ///
  /// 初始化
  ///
  pub fn new(
    charlist: Vec<char>,
    loc: Option<Loc>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
    context: ParseContext,
    importfiles: &mut Vec<FileRef>,
  ) -> Result<Self, String> {
    // 处理 导入
    if charlist.len() > "@import".len() && charlist[0..7] == vec!['@', 'i', 'm', 'p', 'o', 'r', 't']
    {
      match ImportNode::new(charlist, loc, parent, fileinfo, context, importfiles) {
        HandleResult::Success(obj) => return Ok(VarRuleNode::Import(obj)),
        HandleResult::Fail(msg) => {
          return Err(msg);
        }
        HandleResult::Swtich => {}
      };
    } else if charlist.len() > "@".len() && *charlist.get(0).unwrap() == '@' {
      // 处理 变量声明
      return Err(format!(
        "cssfile {} has not allow include vars in content -> {} !",
        fileinfo.unwrap().upgrade().unwrap().borrow().disk_location,
        charlist.poly()
      ));
    } else {
      // 处理 规则
      match StyleRuleNode::new(charlist, loc, parent, fileinfo, context) {
        HandleResult::Success(obj) => return Ok(VarRuleNode::StyleRule(obj)),
        HandleResult::Fail(msg) => {
          return Err(msg);
        }
        HandleResult::Swtich => {}
      };
    }
    Err("nothing node match the txt!".to_string())
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
    let value_type = map.get("type").unwrap().to_string();
    if value_type == r#""Import""# {
      // 处理引用
      let value_map = map.get("value").unwrap().as_object().unwrap();
      return Ok(VarRuleNode::Import(ImportNode::deserializer(
        value_map, context, parent, fileinfo,
      )?));
    } else if value_type == r#""StyleRule""# {
      let value_map = map.get("value").unwrap().as_object().unwrap();
      return Ok(VarRuleNode::StyleRule(StyleRuleNode::deserializer(
        value_map, context, parent, fileinfo,
      )?));
    }
    Err("VarRuleNode -> noting type is matched".to_string())
  }
}
