use crate::extend::enum_extend::EnumExtend;

#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenVarKeyAllow {
  #[strum(serialize = ":")]
  Colon,

  #[strum(serialize = "-")]
  Dash,
}

impl EnumExtend for TokenVarKeyAllow {}

impl Into<String> for TokenVarKeyAllow {
  fn into(self) -> String {
    self.to_string()
  }
}
