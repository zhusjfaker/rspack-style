use rspack_style::extend::time::wastetime;
use rspack_style::style_core::applicationn::Application;
use rspack_style::util::file::path_resolve;

#[test]
fn test_less_parse() {
  let record = wastetime("test_less_parse");
  // 处理过程
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  let info = app.parse(filepath.as_str()).unwrap();
  record();
  let json = serde_json::to_string_pretty(&info).unwrap();
  println!("{}", json);
}
