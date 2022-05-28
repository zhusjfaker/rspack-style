use crate::util::str_enum::EnumToChar;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum TokenSelectChar {
  ClassToken,
  IdToken,
  AttrBegin,
  AttrEnd,
  LeftBrackets,
  RightBrackets,
  WildCard,
  Colon,
}

impl EnumToChar for TokenSelectChar {
  fn to_str(&self) -> char {
    match self {
      TokenSelectChar::ClassToken => '.',
      TokenSelectChar::IdToken => '#',
      TokenSelectChar::AttrBegin => '[',
      TokenSelectChar::AttrEnd => ']',
      TokenSelectChar::LeftBrackets => '(',
      TokenSelectChar::RightBrackets => ')',
      TokenSelectChar::WildCard => '*',
      TokenSelectChar::Colon => ':',
    }
  }

  fn iterator() -> Iter<'static, TokenSelectChar> {
    static TOKENS: [TokenSelectChar; 8] = [
      TokenSelectChar::ClassToken,
      TokenSelectChar::IdToken,
      TokenSelectChar::AttrBegin,
      TokenSelectChar::AttrEnd,
      TokenSelectChar::LeftBrackets,
      TokenSelectChar::RightBrackets,
      TokenSelectChar::WildCard,
      TokenSelectChar::Colon,
    ];
    TOKENS.iter()
  }

  fn is(cc: &char) -> bool {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return true;
      }
    }
    false
  }

  fn into(cc: &char) -> Option<TokenSelectChar> {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return Some(token.clone());
      }
    }
    None
  }
}

///
/// Select 允许的连接符
///
#[derive(Debug, Serialize, Eq, PartialEq, Clone, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TokenCombinaChar {
  Comma,
  Space,
  NewLineOs,
  NewLineWindos,
  ExtendChar,
  ColumnChar,
  BrotherMatchChar,
  AddChar,
}

impl EnumToChar for TokenCombinaChar {
  fn to_str(&self) -> char {
    match self {
      TokenCombinaChar::Comma => ',',
      TokenCombinaChar::Space => ' ',
      TokenCombinaChar::NewLineOs => '\n',
      TokenCombinaChar::NewLineWindos => '\r',
      TokenCombinaChar::ExtendChar => '>',
      TokenCombinaChar::ColumnChar => '|',
      TokenCombinaChar::BrotherMatchChar => '~',
      TokenCombinaChar::AddChar => '+',
    }
  }

  fn iterator() -> Iter<'static, TokenCombinaChar> {
    static TOKENS: [TokenCombinaChar; 8] = [
      TokenCombinaChar::Comma,
      TokenCombinaChar::Space,
      TokenCombinaChar::NewLineOs,
      TokenCombinaChar::NewLineWindos,
      TokenCombinaChar::ExtendChar,
      TokenCombinaChar::ColumnChar,
      TokenCombinaChar::BrotherMatchChar,
      TokenCombinaChar::AddChar,
    ];
    TOKENS.iter()
  }

  fn is(cc: &char) -> bool {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return true;
      }
    }
    false
  }

  fn into(cc: &char) -> Option<TokenCombinaChar> {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return Some(token.clone());
      }
    }
    None
  }
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum TokenAllowChar {
  LeftSlant,
  Underscore,
  Dash,
  Percent,
}

impl EnumToChar for TokenAllowChar {
  fn to_str(&self) -> char {
    match self {
      TokenAllowChar::LeftSlant => '\\',
      TokenAllowChar::Underscore => '_',
      TokenAllowChar::Dash => '-',
      TokenAllowChar::Percent => '%',
    }
  }

  fn iterator() -> Iter<'static, TokenAllowChar> {
    static TOKENS: [TokenAllowChar; 4] = [
      TokenAllowChar::LeftSlant,
      TokenAllowChar::Underscore,
      TokenAllowChar::Dash,
      TokenAllowChar::Percent,
    ];
    TOKENS.iter()
  }

  fn is(cc: &char) -> bool {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return true;
      }
    }
    false
  }

  fn into(cc: &char) -> Option<TokenAllowChar> {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return Some(token.clone());
      }
    }
    None
  }
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum TokenKeyWordChar {
  PranedRefer,
  VarRefer,
}

impl EnumToChar for TokenKeyWordChar {
  fn to_str(&self) -> char {
    match self {
      TokenKeyWordChar::PranedRefer => '&',
      TokenKeyWordChar::VarRefer => '@',
    }
  }

  fn iterator() -> Iter<'static, TokenKeyWordChar> {
    static TOKENS: [TokenKeyWordChar; 2] =
      [TokenKeyWordChar::PranedRefer, TokenKeyWordChar::VarRefer];
    TOKENS.iter()
  }

  fn is(cc: &char) -> bool {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return true;
      }
    }
    false
  }

  fn into(cc: &char) -> Option<TokenKeyWordChar> {
    for token in Self::iterator() {
      if *cc == token.to_str() {
        return Some(token.clone());
      }
    }
    None
  }
}
