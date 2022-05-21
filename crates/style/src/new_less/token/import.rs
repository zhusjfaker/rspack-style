use crate::extend::enum_extend::EnumExtend;

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

impl Into<String> for TokenImport {
  fn into(self) -> String {
    format!("{}", self)
  }
}
