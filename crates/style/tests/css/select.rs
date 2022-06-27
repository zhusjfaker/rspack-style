use rspack_style::css::select::NewSelector;
use rspack_style::extend::string::StringExtend;

#[test]
fn test_css_select_var_error_parse() {
  let ss = "@{abc} .b";
  let mut obj = NewSelector::new(ss.to_string().to_char_vec(), None, None, None, None);
  assert!(obj.parse().is_err());
}
