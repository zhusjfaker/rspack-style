use crate::extend::enum_extend::EnumExtend;

#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenStyleRuleKeyAllow {
  #[strum(serialize = ":")]
  Colon,

  #[strum(serialize = "-")]
  Dash,
}

impl EnumExtend for TokenStyleRuleKeyAllow {}

impl Into<String> for TokenStyleRuleKeyAllow {
  fn into(self) -> String {
    self.to_string()
  }
}
