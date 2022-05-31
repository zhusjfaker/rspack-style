use rspack_style::style_core::applicationn::Application;
use rspack_style::util::file::path_resolve;

#[test]
fn arco_global_css_render() {
  let filepath = path_resolve("assets/arco-pro-css/src-style-global.css");
  let app = Application::default();
  app.render(filepath.as_str()).unwrap();
}
