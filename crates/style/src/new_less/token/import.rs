use crate::extend::enum_extend::EnumExtend;
use crate::extend::str_into::StringInto;

///
/// Select 合词字符串
///
#[derive(EnumString, Display, Debug, EnumIter, PartialEq)]
pub enum TokenImport {
  #[strum(serialize = r#"'"#)]
  Apost,

  #[strum(serialize = r#"""#)]
  Quote,
}

impl EnumExtend for TokenImport {}

impl StringInto for TokenImport {}
