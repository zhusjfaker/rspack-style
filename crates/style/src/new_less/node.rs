use crate::new_less::comment::CommentNode;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::rule::RuleNode;
use crate::new_less::var::VarRuleNode;
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
}
