use crate::extend::string::StringExtend;
use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_render() {
  let filepath = path_resolve("assets/test.less");
  let app = Application::default();
  let res = app.render(filepath.as_str()).unwrap();
  // let context1 = Context::new(Default::default(), Some(filepath.clone())).unwrap();
  // let info = context1.parse(filepath).unwrap();
  // let json = serde_json::to_string_pretty(&info).unwrap();
  // println!("{}", json);
  println!("{}", res);
}

#[test]
fn test_css_render_content() {
  let filepath = "/Users/zhushijie/Desktop/github/rspack/examples/arco-pro/src/style/layout.module.less";
  let content = r#"
.layout {
  width: 100%;
  height: 100%;
}
.layout-navbar {
  position: fixed;
  width: 100%;
  min-width: 1100px;
  top: 0;
  left: 0;
  height: 60px;
  z-index: 100;
}
.layout-navbar-hidden {
  height: 0;
}
.layout-sider {
  position: fixed;
  height: 100%;
  top: 0;
  left: 0;
  z-index: 99;
  box-sizing: border-box;
}
.layout-sider ::-webkit-scrollbar {
  width: 12px;
  height: 4px;
}
.layout-sider ::-webkit-scrollbar-thumb {
  border: 4px solid transparent;
  background-clip: padding-box;
  border-radius: 7px;
  background-color: var(--color-text-4);
}
.layout-sider ::-webkit-scrollbar-thumb:hover {
  background-color: var(--color-text-3);
}
.layout-sider::after {
  content: '';
  display: block;
  position: absolute;
  top: 0;
  right: -1px;
  width: 1px;
  height: 100%;
  background-color: var(--color-border);
}
.layout-sider > :global(.arco-layout-sider-children) {
  overflow-y: hidden;
}
.layout-sider .collapse-btn {
  height: 24px;
  width: 24px;
  background-color: var(--color-fill-1);
  color: var(--color-text-3);
  border-radius: 2px;
  cursor: pointer;
  display: flex;
  justify-content: center;
  align-items: center;
  position: absolute;
  bottom: 12px;
  right: 12px;
}
.layout-sider .collapse-btn:hover {
  background-color: var(--color-fill-3);
}
.menu-wrapper {
  overflow: auto;
  height: 100%;
}
.menu-wrapper :global(.arco-menu-item-inner > a::after),
.menu-wrapper :global(.arco-menu-item > a::after) {
  content: '';
  display: block;
  position: absolute;
  width: 100%;
  height: 100%;
  left: 0;
  right: 0;
  top: 0;
  bottom: 0;
}
.menu-wrapper :global(.arco-menu-inline-header) {
  font-weight: 500;
}
.icon {
  font-size: 18px;
  vertical-align: text-bottom;
}
.icon-empty {
  width: 12px;
  height: 18px;
  display: inline-block;
}
.layout-content {
  background-color: var(--color-fill-2);
  min-width: 1100px;
  min-height: 100vh;
  transition: padding-left 0.2s;
  box-sizing: border-box;
}
.layout-content-wrapper {
  padding: 16px 20px 0px 20px;
}
.layout-breadcrumb {
  margin-bottom: 16px;
}
.spin {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  min-height: calc(100vh - 60px);
}
  "#;
  let app = Application::default();
  {
    app.context.lock().unwrap().option.hooks.content_interceptor = None;
  }
  let res = app.render_content_into_hashmap(content, filepath).unwrap();
  println!("{:#?}", res);
}

#[test]
fn test_less_render_into_map() {
  let filepath1 = path_resolve("assets/render_map/index.css");
  let filepath2 = path_resolve("assets/render_map/lib_2.css");
  let filepath3 = path_resolve("assets/render_map/lib_1.css");
  let filepath4 = path_resolve("assets/render_map/lib_3.css");
  let app = Application::default();
  let res1 = app.render_into_hashmap(filepath1.as_str()).unwrap();
  let res2 = app.render_into_hashmap(filepath2.as_str()).unwrap();
  let res3 = app.render_into_hashmap(filepath3.as_str()).unwrap();
  let res4 = app.render_into_hashmap(filepath4.as_str()).unwrap();
  println!("{:#?}->{:#?}->{:#?}->{:#?}", res1, res2, res3, res4);
}

#[test]
fn test_keyframe_at_select_render() {
  let filepath = path_resolve("assets/keyframes.less");
  let app = Application::default();
  let res = app.render(filepath.as_str()).unwrap();
  println!("{}", res);
  let target_code = r#"
.a, .b {
  width: 20px;
}

@media screen and ( max-width: 900px){
  @keyframes identifier{
      0% {
        top: 0;
        left: 0;
      }
      30% {
        top: 50px;
      }
      68%,
      72% {
        left: 50px;
      }
      100% {
        top: 100px;
        left: 100%;
      }
  }
}
@keyframes popanit{
    0% {
      top: 0;
      left: 0;
    }
    30% {
      top: 50px;
    }
    68%,
    72% {
      left: 50px;
    }
    100% {
      top: 100px;
      left: 100%;
    }
}
  "#;
  assert_eq!(
    res.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_demo_render() {
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  let res = app.render(filepath.as_str()).unwrap();
  println!("{}", res);
  let target_code = r#"
  h2 {
  font-size: 10px;
  display: block;
}
h2 .a {
  display: block;
  box-sizing: border-box;
}

textarea {
  width: 400px;
  height: 300px;
  font-size: 12px;
  border: 601px solid #fff;
}
textarea .a {
  font-size: 12px;
}
textarea .a .c {
  font-size: 12px;
}
textarea .b {
  font-size: 12px;
}
.a {
  font-size: 12px;
}
@media screen and (max-width: 900px) {
  .a {
    font-size: 12px;
  }
}
@media screen and (min-width: 900px) {
  .xyz {
    font-size: 12px;
  }
}
@media screen and (min-width: 900px) and screen and (max-width: 900px) {
  .xyz {
    color: red;
  }
}
.ace {
  font-size: 10px;
}
.ace .b {
  font-size: 20px;
} 
  "#;
  assert_eq!(
    res.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_string_const_support_var_render() {
  let filepath = path_resolve("assets/stringconst.less");
  let app = Application::default();
  let res = app.render(filepath.as_str()).unwrap();
  println!("{}", res);
  let target_code = r#"
.d {
  width: 20px-anchor;
  display: xyz block;
  height: "20px";
}
  "#;
  assert_eq!(
    res.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_select_support_var_render() {
  let filepath = path_resolve("assets/select_var.less");
  let app = Application::default();
  let res = app.render(filepath.as_str()).unwrap();
  println!("{}", res);
  let target_code = r#"
.a {
  height: 20px;
}

.a h2 {
  width: 10px;
}

@-webkit-keyframes nprogress-spinner {
  0%   { -webkit-transform: rotate(0deg); }
  100% { -webkit-transform: rotate(360deg); }
}
    "#;
  assert_eq!(
    res.simple_compare(),
    target_code.to_string().simple_compare()
  );
}

#[test]
fn test_multi_file_render() {
  let filepath1 = path_resolve("assets/demo.less");
  let filepath2 = path_resolve("assets/multi_same_import.less");
  let app = Application::default();
  let map1 = app.render_into_hashmap(filepath1.as_str()).unwrap();
  let map2 = app.render_into_hashmap(filepath2.as_str()).unwrap();
  println!("map1->{:#?} \n map2->{:#?}", map1, map2)
}

#[test]
fn test_select_mixin_render() {
  let filepath = path_resolve("assets/mixin.less");
  let app = Application::default();
  let res = app.render(filepath.as_str()).unwrap();
  println!("{}", res);
  //   let target_code = r#"
  // .a {
  //   height: 20px;
  // }
  //
  // .a h2 {
  //   width: 10px;
  // }
  //
  // @-webkit-keyframes nprogress-spinner {
  //   0%   { -webkit-transform: rotate(0deg); }
  //   100% { -webkit-transform: rotate(360deg); }
  // }
  //     "#;
  //   assert_eq!(
  //     res.simple_compare(),
  //     target_code.to_string().simple_compare()
  //   );
}
