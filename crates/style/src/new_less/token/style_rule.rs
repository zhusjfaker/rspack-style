use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;

#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenStyleRuleKeyAllow {
  #[strum(serialize = ":")]
  Colon,

  #[strum(serialize = "-")]
  Dash,
}

impl EnumExtend for TokenStyleRuleKeyAllow {}

impl StringInto for TokenStyleRuleKeyAllow {}
