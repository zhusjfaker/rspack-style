use rspack_style::style_core::applicationn::Application;
use rspack_style::util::file::path_resolve;

#[test]
fn arco_global_css_render() {
  let filepath = path_resolve("assets/arco-pro-css/src-style-global.css");
  let app = Application::default();
  app.render(filepath.as_str()).unwrap();
}

#[test]
fn arco_all_css_render() {
  let mut res = vec![];
  let dir = path_resolve("assets/arco-pro-css");
  let app = Application::default();
  for entry in std::path::Path::new(&dir).read_dir().unwrap() {
    let file = entry.unwrap().path();
    if file.is_file() && file.extension().unwrap().to_str().unwrap() == "css" {
      let css_file = file.to_str().unwrap();
      let css_render_res = app.render(css_file);
      println!("path -> \n {}", css_file);
      res.push(css_render_res.is_ok());
    }
  }
  assert!(!res.contains(&false))
}
