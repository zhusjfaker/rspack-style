use crate::new_less::fileinfo::FileInfo;
use crate::new_less::hooks::ParseHooks;
use crate::new_less::rule::RuleNode;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Clone)]
pub struct ParseOption {
  pub include_path: Vec<String>,
  pub sourcemap: bool,
  pub tabspaces: usize,
  pub modules: Option<bool>,
  pub hooks: ParseHooks,
}

impl Debug for ParseOption {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ParseOption")
      .field("include_path", &self.include_path)
      .field("sourcemap", &self.sourcemap)
      .field("tabspaces", &self.tabspaces)
      .finish()
  }
}

impl PartialEq for ParseOption {
  fn eq(&self, other: &Self) -> bool {
    self.sourcemap == other.sourcemap
      && self.include_path == other.include_path
      && self.tabspaces == other.tabspaces
  }
}

impl Default for ParseOption {
  fn default() -> Self {
    ParseOption {
      include_path: vec![],
      sourcemap: true,
      tabspaces: 2,
      modules: None,
      hooks: Default::default(),
    }
  }
}

pub trait OptionExtend {
  fn get_options(&self) -> ParseOption;
}

impl OptionExtend for FileInfo {
  fn get_options(&self) -> ParseOption {
    self.context.deref().lock().unwrap().option.clone()
  }
}

impl OptionExtend for RuleNode {
  fn get_options(&self) -> ParseOption {
    self.context.deref().lock().unwrap().option.clone()
  }
}
