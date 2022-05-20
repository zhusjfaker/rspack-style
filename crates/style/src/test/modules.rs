use crate::extend::string::StringExtend;
use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_css_module_render() {
  let filepath = path_resolve("assets/css_modules/index.module.less");
  let app = Application::default();
  // {
  //   app.context.lock().unwrap().option.hooks.content_interceptor = None;
  // }
  let res = app.render(filepath.as_str()).unwrap();
  let target_code = r#"
.x {
  display: inline-block;
  width: 20px;
}

h2,h3 {
  font-size: 10px;
  display: block;
}

h2 .a_css_modules_index_module_11640143319789901418,h3 .a_css_modules_index_module_11640143319789901418 {
  display: block;
  box-sizing: border-box;
}

h2 .m_css_modules_index_module_11640143319789901418 .tap #h2,h3 .m_css_modules_index_module_11640143319789901418 .tap #h2 {
  word-break: break-all;
  width: 40px;
}

.kol_css_modules_index_module_11640143319789901418 h2 .m_css_modules_index_module_11640143319789901418 .tap #h2,.kol_css_modules_index_module_11640143319789901418 h3 .m_css_modules_index_module_11640143319789901418 .tap #h2 {
  width: 100px;
}

.u_css_modules_index_module_11640143319789901418 h2,.u_css_modules_index_module_11640143319789901418 h3 {
  display: inline-block;
  width: 20px;
}

h2 .b,h3 .b {
  display: inline-block;
  width: 20px;
}

.c_css_modules_index_module_11640143319789901418 h2 ,.c_css_modules_index_module_11640143319789901418 h3  {
  display: inline-block;
  width: 20px;
}
  "#;
  println!("{}", res);
  assert_eq!(
    target_code.to_string().simple_compare(),
    res.simple_compare()
  );
}

#[test]
fn test_less_css_module_js_content_render() {
  let filepath = path_resolve("assets/css_modules/lib.module.less");
  let app = Application::default();
  // {
  //   app.context.lock().unwrap().option.hooks.content_interceptor = None;
  // }
  // todo fix  same key but hashvalue diff in different less file
  // example -> .a 1.less .a 2.less
  let (css, js) = app.render_into_hashmap(filepath.as_str()).unwrap();
  println!("css_map ->{:#?}", css);
  println!("js ->{:#?}", js);

}
