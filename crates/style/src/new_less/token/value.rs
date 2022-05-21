use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;

#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenValueAllow {
  #[strum(serialize = "[")]
  LeftBrackets,

  #[strum(serialize = "]")]
  RightBrackets,

  #[strum(serialize = "(")]
  LeftParentheses,

  #[strum(serialize = ")")]
  RightParentheses,

  #[strum(serialize = r#"\"#)]
  Backslash,
}

impl EnumExtend for TokenValueAllow {}

impl StringInto for TokenValueAllow {}
