use rspack_style::style_core::applicationn::Application;
use std::time::Instant;

use rspack_style::style_core::option::ParseOption;
use rspack_style::util::file::path_resolve;
fn main() {
  let filepath = path_resolve("assets/arco-pro/42.less");
  let start = Instant::now();
  for _i in 0..50 {
    let mut options = ParseOption::default();
    options.hooks.content_interceptor = None;
    let app = Application::new(options, None).unwrap();
    app.render(filepath.as_str()).unwrap();
  }
  println!("{:?}", start.elapsed());
}
