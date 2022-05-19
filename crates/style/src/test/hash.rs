use crate::new_less::file::{path_resolve, readfile};
use crate::new_less::hash::StyleHash;

#[test]
fn test_content_hash() {
  let content = "hello world";
  let hash_value = StyleHash::generate_hash_by_content(content);
  println!("{}", hash_value);
  let hash_value2 = StyleHash::generate_hash_by_content(content);
  println!("{}", hash_value2);
  assert_eq!(hash_value, hash_value2);
}

#[test]
fn test_css_module_hash() {
  let filepath = path_resolve("assets/demo.less");
  let content = readfile(&filepath).unwrap();
  let css_module_hash = StyleHash::generate_css_module_hash(&filepath, &content);
  println!("{}", css_module_hash);
}
