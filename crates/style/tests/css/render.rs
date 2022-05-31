use rspack_style::extend::string::StringExtend;
use rspack_style::style_core::applicationn::Application;
use rspack_style::util::file::{path_resolve, readfile};

#[test]
fn test_css_minify_render() {
  let filepath = path_resolve("assets/css/index.css");
  let app = Application::default();
  app.set_minify(true);
  let res = app.render(filepath.as_str()).unwrap().0;
  let filepath = path_resolve("assets/css/index.min.css");
  let content = readfile(filepath.as_str()).unwrap();
  println!("{}", res);
  assert_eq!(content.simple_compare(), res.simple_compare());
  assert_eq!(false, res.contains("\n"));
}

#[test]
fn test_css_module_render() {
  let filepath = path_resolve("assets/css/test.module.css");
  let app = Application::default();
  let (css_map, js_content) = app.render_into_hashmap(filepath.as_str()).unwrap();
  let (css, js) = app.render(filepath.as_str()).unwrap();
  let target_css_code = r#"
  .a_css_test_module_12534504311691276445 {
    width: 30px;
  }
  .xyz .b_css_test_module_12534504311691276445 .c .d {
    height: 30px;
  }
"#;
  let target_js_code = r#"
    const style = {
        a: "a_css_test_module_12534504311691276445",
        b: "b_css_test_module_12534504311691276445",
    };
    export default style;
"#;
  println!("{}", css);
  println!("{}", js);
  println!("{:#?}", css_map);
  assert_eq!(
    target_js_code.to_string().simple_compare(),
    js.simple_compare()
  );
  assert_eq!(js, js_content);
  assert_eq!(
    css.simple_compare(),
    target_css_code.to_string().simple_compare()
  );
}
