pub trait StringInto {
  fn tostr_value(&self) -> String
  where
    Self: std::fmt::Display,
  {
    self.to_string()
  }
}
