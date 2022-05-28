use rspack_style::util::file::{path_resolve, readfile};
use rspack_style::util::hash::StyleHash;

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
