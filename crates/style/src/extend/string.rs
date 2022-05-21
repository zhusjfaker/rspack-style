#[allow(non_snake_case)]
pub trait StringExtend {
 fn to_char_vec(&self) -> Vec<char>;
  fn simple_compare(&self) -> std::string::String;
}

#[allow(non_snake_case)]
impl StringExtend for String {
  fn to_char_vec(&self) -> Vec<char> {
    self.chars().collect::<Vec<char>>()
  }

  fn simple_compare(&self) -> String {
    let mut new_str = self.replace(' ', "");
    new_str = new_str.trim().replace('\n', "").replace('\r', "");
    new_str
  }
}
