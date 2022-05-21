use crate::extend::enum_extend::EnumExtend;

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

impl Into<String> for TokenValueAllow {
  fn into(self) -> String {
    self.to_string()
  }
}
