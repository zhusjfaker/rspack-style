use std::time::Instant;

use rspack_style::new_less::applicationn::Application;
use rspack_style::new_less::file::path_resolve;
fn main() {
  let filepath = path_resolve("assets/arco-pro/3.less");
  let app = Application::default();
  let start = Instant::now();
  app.render(filepath.as_str()).unwrap();
}
