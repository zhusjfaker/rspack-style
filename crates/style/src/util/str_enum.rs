use std::slice::Iter;

pub trait EnumToChar {
  fn to_str(&self) -> char;
  fn iterator() -> Iter<'static, Self>
  where
    Self: Sized;
  fn is(cc: &char) -> bool;
  fn into(cc: &char) -> Option<Self>
  where
    Self: Sized;
}

pub trait EnumToString {
  fn to_str(&self) -> &'static str;
  fn iterator() -> Iter<'static, Self>
  where
    Self: Sized;
  fn is(cc: &str) -> bool;
  fn into(cc: &str) -> Option<Self>
  where
    Self: Sized;
}

pub trait StringToEnum
where
  Self: ToString,
{
  fn to_enum<T>(&self) -> Option<T>
  where
    T: EnumToString;
}

pub trait CharToEnum {
  fn to_enum<T>(&self) -> Option<T>
  where
    T: EnumToChar;
}

impl StringToEnum for String {
  fn to_enum<T: EnumToString>(&self) -> Option<T> {
    let value = self.to_string();
    EnumToString::into(value.as_str())
  }
}

impl CharToEnum for char {
  fn to_enum<T: EnumToChar>(&self) -> Option<T> {
    EnumToChar::into(self)
  }
}
