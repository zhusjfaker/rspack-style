use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;
use crate::new_less::option::ParseOption;
use unit_test::Bencher;

#[bench]
fn parse_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn parse_var_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/var.less");
    let app = Application::default();
    app.parse(filepath.as_str()).unwrap();
  });
}

#[bench]
fn parse_var_recovery_bench(bench: &mut Bencher) {
  let filepath = path_resolve("assets/var.less");
  let app = Application::default();
  app.parse(filepath.as_str()).unwrap();
  bench.iter(|| {
    app
      .context
      .lock()
      .unwrap()
      .recovery_parse_object(filepath.as_str())
      .unwrap();
  });
}

#[bench]
fn render_less_arco_pro_bench(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/arco-pro/13.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_less_arco_pro_bench_without_sourcemap(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/arco-pro/13.less");
    let app = Application::new(
      ParseOption {
        include_path: vec![],
        sourcemap: false,
        tabspaces: 2,
        modules: None,
        hooks: Default::default(),
      },
      Some(filepath.clone()),
    )
    .unwrap();
    app.render(filepath.as_str()).unwrap();
  });
}
