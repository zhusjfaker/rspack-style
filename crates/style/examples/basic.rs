use std::time::Instant;

use rspack_style::new_less::applicationn::Application;
use rspack_style::new_less::file::path_resolve;
use rspack_style::new_less::option::ParseOption;
fn main() {
  let filepath = path_resolve("assets/arco-pro/42.less");
  let start = Instant::now();
  for i in 0..50 {
    let mut options = ParseOption::default();
    options.hooks.content_interceptor = None;
    let app = Application::new(options, None).unwrap();
    app.render(filepath.as_str()).unwrap();
  }
  println!("{:?}", start.elapsed());
}
