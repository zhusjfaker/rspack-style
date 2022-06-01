use rspack_style::extend::string::StringExtend;
use rspack_style::style_core::applicationn::Application;
use rspack_style::util::file::path_resolve;

#[test]
fn test_less_css_module_render() {
  let filepath = path_resolve("assets/css_modules/index.module.less");
  let app = Application::default();
  let res = app.render(filepath.as_str()).unwrap().0;
  let target_code = r#"
.x {
  display: inline-block;
  width: 20px;
}

h2,h3 {
  font-size: 10px;
  display: block;
}

h2 .a_css_modules_index_module_16312054692010661328,h3 .a_css_modules_index_module_16312054692010661328 {
  display: block;
  box-sizing: border-box;
}

h2 .m_css_modules_index_module_16312054692010661328 .tap #h2,h3 .m_css_modules_index_module_16312054692010661328 .tap #h2 {
  word-break: break-all;
  width: 40px;
}

.kol_css_modules_index_module_16312054692010661328 h2 .m_css_modules_index_module_16312054692010661328 .tap #h2,.kol_css_modules_index_module_16312054692010661328 h3 .m_css_modules_index_module_16312054692010661328 .tap #h2 {
  width: 100px;
}

.u_css_modules_index_module_16312054692010661328 h2,.u_css_modules_index_module_16312054692010661328 h3 {
  display: inline-block;
  width: 20px;
}

h2 .b,h3 .b {
  display: inline-block;
  width: 20px;
}

.c_css_modules_index_module_16312054692010661328 h2 ,.c_css_modules_index_module_16312054692010661328 h3  {
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
  let (_, js_content) = app.render_into_hashmap(filepath.as_str()).unwrap();
  let (css, js) = app.render(filepath.as_str()).unwrap();
  let target_js_code = r#"
      const style = {
            abc: "abc_css_modules_lib_module_18422443650085235901",
            max: "max_css_modules_lib_module_18422443650085235901",
            min: "min_css_modules_lib_module_18422443650085235901",
            uiz: "uiz_css_modules_lib_module_18422443650085235901",
    };
    export default style;
  "#;
  assert_eq!(
    js_content.simple_compare(),
    target_js_code.to_string().simple_compare()
  );
  assert_eq!(js_content, js);
  println!("{}", css);
  println!("{}", js_content);
  let target_code = r#"
.abc_css_modules_lib_module_18422443650085235901 {
  width: 30px;
}

.abc_css_modules_lib_module_18422443650085235901 .uiz_css_modules_lib_module_18422443650085235901 {
  pointer-events: auto;
}

h2 {
  display: inline-block;
}

.pdc {
  height: 20px;
}


.min_css_modules_lib_module_18422443650085235901 {
  white-space: break-spaces;
}


.max_css_modules_lib_module_18422443650085235901 {
  margin-bottom: 200px;
}


.abc_css_modules_lib_module_18422443650085235901 {
  width: 30px;
}

.abc_css_modules_lib_module_18422443650085235901 .uiz_css_modules_lib_module_18422443650085235901 {
  pointer-events: auto;
}

h2 {
  display: inline-block;
}

.pdc {
  height: 20px;
}
"#;
  assert_eq!(
    css.simple_compare(),
    target_code.to_string().simple_compare()
  );
}
