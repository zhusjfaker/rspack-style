use regex::Regex;

///
/// 替换所有的换行 为 空格
///
pub fn merge_wrap(content: &str) -> String {
  content.replace('\n', " ")
}

///
/// 合并空格
///
pub fn merge_spaces(content: &str) -> String {
  let re = Regex::new("\\s+").unwrap();
  re.replace_all(content, " ").to_string()
}
