use strum::IntoEnumIterator;

pub trait EnumExtend {
  fn enum_vec() -> Vec<String>
  where
    Self: IntoEnumIterator,
    Self: std::fmt::Display,
  {
    let mut list = vec![];
    for e in Self::iter() {
      list.push(e.to_string());
    }
    list
  }

  fn is(char: &str) -> bool
  where
    Self: IntoEnumIterator,
    Self: std::fmt::Display,
  {
    let list = Self::enum_vec();
    list.into_iter().any(|x| x == char)
  }
}
