use crate::less::comment::CommentNode;
use crate::less::fileinfo::FileWeakRef;
use crate::less::rule::RuleNode;
use crate::less::var::VarRuleNode;
use crate::sourcemap::loc::Loc;
use crate::style_core::context::ParseContext;
use serde::Serialize;
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NodeWeakRef = Option<Weak<RefCell<RuleNode>>>;
pub type NodeRef = Rc<RefCell<RuleNode>>;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum StyleNode {
  Comment(CommentNode),
  Var(VarRuleNode),
  Rule(NodeRef),
}

impl StyleNode {
  ///
  /// json 反解恢复对象的方法
  ///
  pub fn deserializer(
    map: &Map<String, Value>,
    context: ParseContext,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Result<Self, String> {
    let value_type = map.get("type").unwrap().to_string();
    if value_type == r#""Comment""# {
      // 处理注释
      let value_map = map.get("value").unwrap().as_object().unwrap();
      return Ok(StyleNode::Comment(CommentNode::deserializer(value_map)?));
    } else if value_type == r#""Var""# {
      // 处理变量
      let value_map = map.get("value").unwrap().as_object().unwrap();
      return Ok(StyleNode::Var(VarRuleNode::deserializer(
        value_map, context, parent, fileinfo,
      )?));
    } else if value_type == r#""Rule""# {
      let value_map = map.get("value").unwrap().as_object().unwrap();
      return Ok(StyleNode::Rule(RuleNode::deserializer(
        value_map, context, parent, fileinfo,
      )?));
    }
    Err("StyleNode -> noting type is matched".to_string())
  }

  ///
  /// 收集 文件 上所有 rule 节点的 loc
  ///
  pub fn collect_loc(node: StyleNode, list: &mut Vec<Option<Loc>>) {
    if let StyleNode::Rule(rule) = node {
      list.push(rule.borrow().loc.as_ref().cloned());
      for child_node in &rule.borrow().block_node {
        Self::collect_loc(child_node.clone(), list);
      }
    } else if let StyleNode::Var(VarRuleNode::StyleRule(style)) = node {
      list.push(style.loc.as_ref().cloned());
    } else if let StyleNode::Var(VarRuleNode::Var(var)) = node {
      list.push(var.loc.as_ref().cloned());
    } else if let StyleNode::Comment(cc) = node {
      list.push(cc.loc.as_ref().cloned());
    }
  }
}
