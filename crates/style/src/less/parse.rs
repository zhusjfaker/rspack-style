use crate::extend::vec_str::VecCharExtend;
use crate::less::comment::CommentNode;
use crate::less::fileinfo::FileWeakRef;
use crate::less::filenode::FileNode;
use crate::less::node::{NodeRef, NodeWeakRef, StyleNode};
use crate::less::rule::RuleNode;
use crate::less::var::VarRuleNode;
use crate::sourcemap::loc::{Loc, LocMap};
use crate::style_core::context::ParseContext;
use crate::style_core::filenode::StyleFileNode;

impl Parse for FileNode {
  fn parse_heap(&mut self) -> Result<(), String> {
    let mut importfiles: Vec<StyleFileNode> = vec![];
    let (commentlsit, varlist, rulelist) = {
      let info = self.info.borrow();
      Self::parse(
        info.context.clone(),
        &info.origin_charlist,
        &info.locmap,
        None,
        info.self_weak.clone(),
        &mut importfiles,
      )?
    };
    let mut info = self.info.borrow_mut();
    info.block_node.append(
      &mut commentlsit
        .into_iter()
        .map(StyleNode::Comment)
        .collect::<Vec<StyleNode>>(),
    );
    info.block_node.append(
      &mut varlist
        .into_iter()
        .map(StyleNode::Var)
        .collect::<Vec<StyleNode>>(),
    );
    info.block_node.append(
      &mut rulelist
        .into_iter()
        .map(StyleNode::Rule)
        .collect::<Vec<StyleNode>>(),
    );
    info.import_files = importfiles;
    Ok(())
  }
}

impl Parse for RuleNode {
  fn parse_heap(&mut self) -> Result<(), String> {
    let mut importfiles: Vec<StyleFileNode> = vec![];
    let (commentlsit, varlist, rulelist) = Self::parse(
      self.context.clone(),
      &self.origin_charlist,
      &self.locmap,
      self.weak_self.clone(),
      self.file_info.clone(),
      &mut importfiles,
    )?;
    self.block_node.append(
      &mut commentlsit
        .into_iter()
        .map(StyleNode::Comment)
        .collect::<Vec<StyleNode>>(),
    );
    self.block_node.append(
      &mut varlist
        .into_iter()
        .map(StyleNode::Var)
        .collect::<Vec<StyleNode>>(),
    );
    rulelist.iter().for_each(|node| {
      node.borrow_mut().parent = self.weak_self.clone();
    });
    self.block_node.append(
      &mut rulelist
        .into_iter()
        .map(StyleNode::Rule)
        .collect::<Vec<StyleNode>>(),
    );
    Ok(())
  }
}

pub type TupleNodeVec = (Vec<CommentNode>, Vec<VarRuleNode>, Vec<NodeRef>);

pub trait Parse {
  ///
  /// ?????? ??????????????????
  ///
  fn parse(
    context: ParseContext,
    origin_charlist: &[char],
    locmap: &Option<LocMap>,
    parent: NodeWeakRef,
    fileinfo: FileWeakRef,
    importfiles: &mut Vec<StyleFileNode>,
  ) -> Result<TupleNodeVec, String> {
    let mut comment_list: Vec<CommentNode> = vec![];
    let mut rule_node_list: Vec<NodeRef> = vec![];
    let mut var_node_list: Vec<VarRuleNode> = vec![];

    let mut comment_word: Vec<char> = vec![];
    let mut temp_word: Vec<char> = vec![];
    let mut selector_txt: Vec<char> = vec![];

    let mut index = 0;
    // ?????????
    let mut braces_level = 0;
    // ???????????? & ????????????
    let endqueto = ';';
    let start_braces = '{';
    let end_braces = '}';

    // ????????? ?????? ?????????
    let mut wirte_comment = false;
    let mut wirte_line_comment = false;
    let mut wirte_closure_comment = false;

    let single_queto = '\'';
    let double_queto = '"';
    let mut match_queto: Option<char> = None;

    // ???????????? sourcemap ?????????????????????
    let mut record_loc: Option<Loc> = None;

    // ?????? ???????????? ??????
    let mut comment_start_index: Option<usize> = None;

    let mut ignore_braces_level = 0;

    let option = {
      let sync_context = context.lock().unwrap();
      sync_context.option.clone()
    };

    while index < origin_charlist.len() {
      // ????????????
      let char = origin_charlist.get(index).unwrap();
      let next = if index < origin_charlist.len() - 1 {
        origin_charlist.get(index + 1)
      } else {
        None
      };
      let prev = if index > 0 {
        origin_charlist.get(index - 1)
      } else {
        None
      };

      // ??????????????? ????????????
      if (*char == single_queto || *char == double_queto)
        && match_queto.is_none()
        && !wirte_comment
        && !wirte_line_comment
      {
        match_queto = Some(*char);
      } else if match_queto.is_some()
        && *char == match_queto.unwrap()
        && !wirte_comment
        && !wirte_line_comment
      {
        match_queto = None;
      }

      // ??????????????????
      if match_queto.is_none()
        && (char, next) == (&'/', Some(&'/'))
        && braces_level == 0
        && !wirte_comment
      {
        wirte_comment = true;
        wirte_line_comment = true;
      } else if match_queto.is_none()
        && (char, next) == (&'/', Some(&'*'))
        && braces_level == 0
        && !wirte_comment
      {
        wirte_comment = true;
        wirte_closure_comment = true;
      }

      // ????????????
      if braces_level == 0
        && wirte_comment
        && ((wirte_line_comment && (*char == '\n' || *char == '\r'))
          || (wirte_closure_comment && (char, next) == (&'*', Some(&'/'))))
      {
        wirte_comment = false;
        if wirte_line_comment {
          index += 1;
          comment_word.push(*char);
          wirte_line_comment = false;
        } else if wirte_closure_comment {
          index += 2;
          comment_word.push(*char);
          comment_word.push(*next.unwrap());
          wirte_closure_comment = false;
        }
        let comment = CommentNode {
          content: comment_word.poly(),
          loc: record_loc,
          startindex: comment_start_index.unwrap(),
        };
        comment_list.push(comment);
        comment_word.clear();
        comment_start_index = None;
        record_loc = None;
        continue;
      }
      if wirte_comment {
        // ???????????? sourcemap ???????????????
        if option.sourcemap && *char != '\r' && *char != '\n' && record_loc.is_none() {
          record_loc = Some(locmap.as_ref().unwrap().get(index).unwrap());
        }
        if comment_start_index.is_none() {
          comment_start_index = Some(index);
        }
        comment_word.push(*char);
      } else {
        // ?????? var ??? rule ?????????

        // ????????????
        if option.sourcemap
          && *char != ' '
          && *char != '\r'
          && *char != '\n'
          && record_loc.is_none()
        {
          record_loc = Some(locmap.as_ref().unwrap().get(index).unwrap());
        }
        // ?????????????????????
        temp_word.push(*char);
        if *char == endqueto && braces_level == 0 {
          let style_var = match VarRuleNode::new(
            temp_word.trim(),
            record_loc,
            parent.clone(),
            fileinfo.clone(),
            context.clone(),
            importfiles,
          ) {
            Ok(obj) => obj,
            Err(msg) => {
              return Err(msg);
            }
          };
          var_node_list.push(style_var);
          temp_word.clear();
          record_loc = None;
        }
        // ???????????? ?????? && ignore ?????? ??????????????? && ????????????????????? ?????????
        if *char == start_braces && match_queto.is_none() {
          if prev == Some(&'@') {
            ignore_braces_level += 1;
          } else if ignore_braces_level == 0 {
            if braces_level == 0 {
              selector_txt = temp_word[0..temp_word.len() - 1].to_vec().trim();
              temp_word.clear();
            }
            braces_level += 1;
          } else {
            ignore_braces_level += 1;
          }
        }
        if *char == end_braces && match_queto.is_none() {
          if ignore_braces_level == 0 {
            braces_level -= 1;
            let _content = temp_word[0..temp_word.len() - 1].to_vec().trim();
            if braces_level == 0 {
              match RuleNode::new(
                temp_word[0..temp_word.len() - 1].to_vec(),
                selector_txt.clone(),
                record_loc,
                fileinfo.clone(),
                context.clone(),
              ) {
                Ok(rule) => {
                  rule_node_list.push(rule);
                }
                Err(msg) => {
                  return Err(msg);
                }
              }
              selector_txt.clear();
              temp_word.clear();
              record_loc = None;
            }
          } else {
            ignore_braces_level -= 1;
          }
        }
      }
      index += 1;
    }

    Ok((comment_list, var_node_list, rule_node_list))
  }

  fn parse_heap(&mut self) -> Result<(), String>;
}
