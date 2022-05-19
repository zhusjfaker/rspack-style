use crate::extend::vec_str::VecCharExtend;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::loc::Loc;
use crate::new_less::node::StyleNode;
use crate::new_less::rule::RuleNode;
use serde::Serialize;
use serde_json::{Map, Value};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub trait Comment {
  fn get_comment_blocknode(&self) -> Vec<CommentNode>;
  fn rm_comment(&self) -> String;
  fn skip_comment() -> Box<dyn FnMut(String, char, &mut usize) -> bool>;
}

#[derive(Clone, Serialize)]
pub struct CommentNode {
  // 节点内容
  pub content: String,
  // 节点坐标
  pub loc: Option<Loc>,
  // 注释开始索引
  pub startindex: usize,
}

impl Debug for CommentNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("CommentNode")
      .field("content", &self.content)
      .field("loc", &self.loc)
      .finish()
  }
}

impl CommentNode {
  pub fn deserializer(map: &Map<String, Value>) -> Result<Self, String> {
    let mut comment = CommentNode {
      content: "".to_string(),
      loc: None,
      startindex: 0,
    };
    if let Some(Value::String(content)) = map.get("content") {
      comment.content = content.to_string();
    } else {
      return Err(format!(
        "deserializer CommentNode has error -> content is empty"
      ));
    }
    if let Some(Value::Object(loc)) = map.get("loc") {
      comment.loc = Some(Loc::deserializer(loc));
    } else {
      return Err(format!(
        "deserializer CommentNode has error -> loc is empty"
      ));
    }
    if let Some(Value::Number(startindex)) = map.get("startindex") {
      comment.startindex = startindex.to_string().parse::<usize>().unwrap();
    } else {
      return Err(format!(
        "deserializer CommentNode has error -> startindex is empty"
      ));
    }

    Ok(comment)
  }
}

impl Comment for FileInfo {
  fn get_comment_blocknode(&self) -> Vec<CommentNode> {
    get_comment_blocknode(&self.block_node)
  }

  fn rm_comment(&self) -> String {
    let list = &self.get_comment_blocknode();
    if !list.is_empty() {
      rm_comment(list, &self.origin_charlist)
    } else {
      self.origin_txt_content.clone()
    }
  }

  fn skip_comment() -> Box<dyn FnMut(String, char, &mut usize) -> bool> {
    skip_comment()
  }
}

impl Comment for RuleNode {
  fn get_comment_blocknode(&self) -> Vec<CommentNode> {
    get_comment_blocknode(&self.block_node)
  }

  fn rm_comment(&self) -> String {
    let node_list = &self.get_comment_blocknode();
    if !node_list.is_empty() {
      rm_comment(node_list, &self.origin_charlist)
    } else {
      self.origin_charlist.poly()
    }
  }

  fn skip_comment() -> Box<dyn FnMut(String, char, &mut usize) -> bool> {
    skip_comment()
  }
}

///
/// 从当中的 成熟 AST 中获取 注释节点
///
fn get_comment_blocknode(block_node: &[StyleNode]) -> Vec<CommentNode> {
  let mut list = vec![];
  block_node.iter().for_each(|x| {
    if let StyleNode::Comment(cc) = x.deref().clone() {
      list.push(cc);
    }
  });
  list
}

///
/// 移除注释
/// 必须依赖开启 sourcemap
///
fn rm_comment(commentlist: &[CommentNode], origin_charlist: &[char]) -> String {
  return if commentlist.is_empty() {
    origin_charlist.iter().collect::<String>()
  } else {
    let mut charlist = origin_charlist.to_owned();
    for cc in commentlist {
      let length = cc.content.len();
      let start = cc.startindex;
      let end = cc.startindex + length;
      let mut i = start;
      while i < end {
        let char = charlist.get(i).unwrap();
        if *char != '\n' && *char != '\r' {
          charlist[i] = char::from(32);
        }
        i += 1;
      }
    }
    charlist.iter().collect::<String>()
  };
}

///
/// 是否跳过 返回结果 [0]
/// 跳过索引 返回结果 [1]
///
pub fn skip_comment() -> Box<dyn FnMut(String, char, &mut usize) -> bool> {
  let comment_flag = "//".to_string();
  let comment_mark_strat = "/*".to_string();
  let comment_mark_end = "*/".to_string();
  let mut comment_inline = false;
  let mut comment_mark = false;
  Box::new(move |word, char, index| {
    if word == comment_flag && !comment_inline {
      comment_inline = true;
    }
    if word == comment_mark_strat && !comment_mark {
      comment_mark = true;
    }
    if (char == '\n' || char == '\r') && comment_inline {
      comment_inline = false;
    }
    if word == comment_mark_end && comment_mark {
      comment_mark = false;
      *index += 1;
    }
    comment_inline || comment_mark
  })
}
