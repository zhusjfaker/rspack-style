use rspack_style::extend::string::StringExtend;
use rspack_style::style_core::applicationn::Application;
use rspack_style::util::file::{path_resolve, readfile};

#[test]
fn test_css_minify_render() {
  let filepath = path_resolve("assets/css/index.css");
  let app = Application::default();
  app.set_minify(true);
  let res = app.render(filepath.as_str()).unwrap();
  let filepath = path_resolve("assets/css/index.min.css");
  let content = readfile(filepath.as_str()).unwrap();
  println!("{}", res);
  assert_eq!(content.simple_compare(), res.simple_compare());
  assert_eq!(false, res.contains("\n"));
}