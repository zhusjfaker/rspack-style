use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecCharExtend;
use crate::new_less::context::ParseContext;
use crate::new_less::fileinfo::{FileInfo, FileWeakRef};
use crate::new_less::loc::{Loc, LocMap};
use crate::new_less::node::{NodeRef, NodeWeakRef, StyleNode};
use crate::new_less::option::OptionExtend;
use crate::new_less::parse::Parse;
use crate::new_less::select_node::SelectorNode;
use crate::new_less::style_rule::StyleRuleNode;
use crate::new_less::var::VarRuleNode;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct RuleNode {
  // 选择器 文字
  pub selector: Option<SelectorNode>,
  // 根据 原始内容 -> 转化的 字符数组
  pub origin_charlist: Vec<char>,
  // 节点坐标
  pub loc: Option<Loc>,
  // 当前所有 索引 对应的 坐标行列 -> 用于执行 sourcemap
  pub locmap: Option<LocMap>,
  // 节点 父节点
  pub parent: NodeWeakRef,
  // 自己的引用关系
  pub weak_self: NodeWeakRef,
  // 节点 子节点
  pub block_node: Vec<StyleNode>,
  // 文件弱引用
  pub file_info: FileWeakRef,
  // 全局上下文
  pub context: ParseContext,
}

impl Serialize for RuleNode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("RuleNode", 4)?;
    state.serialize_field("content", &self.origin_charlist.poly())?;
    state.serialize_field("loc", &self.loc)?;
    state.serialize_field("select", &self.selector.as_ref().unwrap())?;
    state.serialize_field("block_node", &self.block_node)?;
    state.end()
  }
}

impl Debug for RuleNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("RuleNode")
      .field("content", &self.origin_charlist.poly())
      .field("loc", &self.loc)
      .field("select", &self.selector.as_ref().unwrap().value())
      .field("block_node", &self.block_node)
      .finish()
  }
}

impl RuleNode {
  ///
  /// 构造方法
  ///
  pub fn new(
    charlist: Vec<char>,
    selector_txt: Vec<char>,
    loc: Option<Loc>,
    file_info: FileWeakRef,
    context: ParseContext,
  ) -> Result<NodeRef, String> {
    let mut change_loc: Option<Loc> = loc.clone();
    let obj = RuleNode {
      selector: None,
      origin_charlist: charlist,
      loc,
      locmap: None,
      block_node: vec![],
      parent: None,
      weak_self: None,
      file_info: file_info.clone(),
      context,
    };
    let heapobj = Rc::new(RefCell::new(obj));
    let wek_self = Rc::downgrade(&heapobj);
    heapobj.borrow_mut().weak_self = Some(wek_self.clone());

    let selector = match SelectorNode::new(selector_txt, &mut change_loc, Some(wek_self), file_info)
    {
      Ok(result) => result,
      Err(msg) => {
        return Err(msg);
      }
    };
    heapobj.borrow_mut().selector = Some(selector);
    if heapobj.deref().borrow().get_options().sourcemap {
      heapobj.borrow_mut().loc = change_loc.as_ref().cloned();
      let (calcmap, _) = LocMap::merge(
        change_loc.as_ref().unwrap(),
        &heapobj.borrow().origin_charlist,
      );
      heapobj.borrow_mut().locmap = Some(calcmap);
    }
    heapobj.borrow_mut().parse_heap()?;
    Ok(heapobj)
  }

  ///
  /// 反序列化
  ///
  pub fn deserializer(
    map: &Map<String, Value>,
    context: ParseContext,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
  ) -> Result<Rc<RefCell<Self>>, String> {
    let mut rule_node = Self {
      selector: None,
      origin_charlist: vec![],
      loc: None,
      locmap: None,
      parent: parent.as_ref().cloned(),
      weak_self: None,
      block_node: vec![],
      file_info: fileinfo.as_ref().cloned(),
      context: context.clone(),
    };
    if let Some(Value::String(content)) = map.get("content") {
      rule_node.origin_charlist = content.tocharlist();
    } else {
      return Err(format!(
        "deserializer RuleNode has error -> content is empty!"
      ));
    }
    if let Some(Value::Object(loc)) = map.get("loc") {
      rule_node.loc = Some(Loc::deserializer(loc));
      rule_node.locmap =
        Some(LocMap::merge(rule_node.loc.as_ref().unwrap(), &rule_node.origin_charlist).0);
    } else {
      rule_node.locmap = Some(LocMap::new(&rule_node.origin_charlist));
    }
    let heapobj = Rc::new(RefCell::new(rule_node));
    let weak_self = Rc::downgrade(&heapobj);
    heapobj.borrow_mut().weak_self = Some(weak_self.clone());
    let json_block_node = map.get("block_node");
    let mut block_node_recovery_list = vec![];
    if let Some(Value::Array(block_nodes)) = json_block_node {
      for json_node in block_nodes {
        if let Value::Object(json_stylenode) = json_node {
          block_node_recovery_list.push(StyleNode::deserializer(
            json_stylenode,
            context.clone(),
            Some(weak_self.clone()),
            fileinfo.as_ref().cloned(),
          )?);
        }
      }
    }
    if let Some(Value::Object(map)) = map.get("select") {
      heapobj.borrow_mut().selector = Some(SelectorNode::deserializer(
        map,
        Some(weak_self.clone()),
        fileinfo.as_ref().cloned(),
      )?);
    } else {
      return Err(format!(
        "deserializer RuleNode has error -> select is empty!"
      ));
    }
    heapobj.borrow_mut().block_node = block_node_recovery_list;
    Ok(heapobj)
  }

  ///
  /// parse 当前文件下 所有的 select 字符串
  /// 需要 第一遍 完成基本遍历
  /// 由 fileinfo -> call 调用
  ///
  pub fn parse_select_all_node(&self) -> Result<(), String> {
    for node in self.block_node.iter() {
      if let StyleNode::Rule(heapnode) = node {
        let mut mut_node = heapnode.borrow_mut();
        let parent = mut_node.parent.clone();
        if let Some(SelectorNode::Select(s_node)) = mut_node.selector.as_mut() {
          s_node.parse(parent)?;
        }
        drop(mut_node);
        heapnode.borrow().parse_select_all_node()?;
      }
    }
    Ok(())
  }

  pub fn visit_mut_file(&self, fileinfo: &mut FileInfo) {
    self.block_node.iter().for_each(|x| {
      if let StyleNode::Rule(rule) = x {
        rule.borrow().visit_mut_file(fileinfo);
      }
    });
  }

  pub fn getrules(&self) -> Vec<NodeRef> {
    let mut list = vec![];

    self.block_node.iter().for_each(|x| {
      if let StyleNode::Rule(rule) = x {
        list.push(rule.clone());
      }
    });
    list
  }

  pub fn get_style_rule(&self) -> Vec<StyleRuleNode> {
    let mut list = vec![];
    self.block_node.iter().for_each(|x| {
      if let StyleNode::Var(VarRuleNode::StyleRule(style)) = x {
        list.push(style.clone());
      }
    });
    list
  }

  pub fn code_gen(&self, content: &mut String, map: &mut HashSet<String>) -> Result<(), String> {
    let rules = self.get_style_rule();
    let (select_txt, media_txt) = self.selector.as_ref().unwrap().code_gen(map).unwrap();
    let mut tab: String = "".to_string();
    let mut index = 0;
    while index < self.get_options().tabspaces {
      tab += " ";
      index += 1;
    }

    // example -> @keyframes, @font-family
    if select_txt.find("@") == Some(0) {
      if media_txt.is_empty() {
        *content += format!(
          "\n{}{}\n{}\n{}",
          select_txt,
          "{",
          tab.clone() + &tab.clone() + self.origin_charlist.poly().as_str(),
          "}"
        )
        .as_str();
      } else {
        *content += format!(
          "\n{}{}\n{}{}\n{}\n{}\n{}",
          media_txt,
          "{",
          tab.clone() + &select_txt,
          "{",
          tab.clone() + &tab.clone() + &tab.clone() + self.origin_charlist.poly().as_str(),
          tab.clone() + "}",
          "}"
        )
        .as_str();
      }

      // 后续不递归了
      return Ok(());
    } else if !rules.is_empty() {
      let create_rules = |tab: String| -> Result<String, String> {
        let mut res: String = "".to_string();
        for (index, rule_res) in rules.iter().enumerate() {
          if index != rules.len() - 1 {
            res += &format!("{}{}{}", tab.clone(), rule_res.code_gen()?, "\n");
          } else {
            res += &format!("{}{}", tab.clone(), rule_res.code_gen()?);
          }
        }
        Ok(res)
      };

      if media_txt.is_empty() {
        *content += format!(
          "\n{}{}\n{}\n{}\n",
          select_txt,
          " {",
          create_rules(tab)?,
          "}"
        )
        .as_ref();
      } else {
        *content += format!(
          "\n{}{}\n{}{}\n{}\n{}\n{}",
          media_txt,
          " {",
          tab.clone() + &select_txt,
          " {",
          create_rules(tab.clone() + &tab.clone())?,
          "  }",
          "}"
        )
        .as_ref();
      }
    }

    for node_ref in self.getrules() {
      node_ref.deref().borrow().code_gen(content, map)?;
    }

    Ok(())
  }
}
