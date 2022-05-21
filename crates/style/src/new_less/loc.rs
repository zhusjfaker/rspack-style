use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loc {
  pub line: usize,
  pub col: usize,
  pub char: char,
  pub index: usize,
}

impl Loc {
  pub fn deserializer(map: &Map<String, Value>) -> Self {
    serde_json::from_str(&serde_json::to_string(map).unwrap()).unwrap()
  }
}

#[derive(Debug, Clone)]
pub struct LocMap {
  data: HashMap<usize, Loc>,
}

impl LocMap {
  ///
  /// 初始化对象
  /// 根据传入的 字符串 txt 构造索引 行|列
  ///
  pub fn new(chars: &[char]) -> Self {
    let map = HashMap::new();
    let mut line = 1;
    let mut col = 1;
    let mut obj = Self { data: map };
    for (index, cc) in chars.iter().enumerate() {
      let loc: Loc;
      if *cc != '\r' && *cc != '\n' {
        loc = Loc {
          col,
          line,
          char: *cc,
          index,
        };
        col += 1;
      } else {
        loc = Loc {
          col,
          line,
          char: *cc,
          index,
        };
        col = 1;
        line += 1;
      }
      obj.data.insert(index, loc);
    }
    obj
  }

  pub fn get(&self, index: &usize) -> Option<Loc> {
    self.data.get(index).cloned()
  }

  pub fn getloc(&self, line: usize, col: usize) -> Option<Loc> {
    let mut loc: Option<Loc> = None;
    for (_index, (_, map)) in self.data.iter().enumerate() {
      if map.line == line && map.col == col {
        loc = Some(map.clone());
        break;
      }
    }
    loc
  }

  pub fn merge(start: &Loc, chars: &[char]) -> (Self, Loc) {
    let map = HashMap::new();
    let mut line = start.line;
    let mut col = start.col;
    let mut obj = LocMap { data: map };
    let mut last: Loc = start.clone();
    for (index, cc) in chars.iter().enumerate() {
      let loc: Loc;
      if *cc != '\r' && *cc != '\n' {
        loc = Loc {
          col,
          line,
          char: *cc,
          index,
        };
        col += 1;
      } else {
        loc = Loc {
          col,
          line,
          char: *cc,
          index,
        };
        col = 1;
        line += 1;
      }
      if index == chars.len() - 1 {
        last = loc.clone();
      }
      obj.data.insert(index, loc);
    }
    (obj, last)
  }
}
