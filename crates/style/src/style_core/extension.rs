use crate::util::str_enum::{EnumToString, StringToEnum};
use serde::Serialize;
use std::path::Path;
use std::slice::Iter;

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(tag = "type", content = "value")]
pub enum StyleExtension {
  Css,
  Less,
  Sass,
  Scss,
}

impl StyleExtension {
  pub fn from_filepath(filepath: &str) -> Option<Self> {
    let path = Path::new(filepath);
    return path
      .extension()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string()
      .to_enum::<Self>();
  }
}

impl EnumToString for StyleExtension {
  fn to_str(&self) -> &'static str {
    match self {
      StyleExtension::Css => "css",
      StyleExtension::Less => "less",
      StyleExtension::Sass => "sass",
      StyleExtension::Scss => "scss",
    }
  }

  fn iterator() -> Iter<'static, StyleExtension> {
    static STYLE: [StyleExtension; 4] = [
      StyleExtension::Css,
      StyleExtension::Less,
      StyleExtension::Sass,
      StyleExtension::Scss,
    ];
    STYLE.iter()
  }

  fn is(cc: &str) -> bool {
    for ext in Self::iterator() {
      if cc == ext.to_str() {
        return true;
      }
    }
    false
  }

  fn into(cc: &str) -> Option<Self> {
    for ext in Self::iterator() {
      if cc == ext.to_str() {
        return Some(ext.clone());
      }
    }
    None
  }
}
