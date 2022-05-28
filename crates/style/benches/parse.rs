use criterion::{criterion_group, criterion_main, Criterion};
use rspack_style::style_core::applicationn::Application;
use rspack_style::style_core::hooks::ParseHooks;
use rspack_style::style_core::option::ParseOption;
use rspack_style::util::file::path_resolve;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("parse_var_bench", |bench| {
    bench.iter(|| {
      let filepath = path_resolve("assets/var.less");
      let mut options = ParseOption::default();
      options.hooks.content_interceptor = None;
      let app = Application::new(options, None).unwrap();
      app.parse(filepath.as_str()).unwrap();
    });
  });

  c.bench_function("parse_var_recovery_bench", |bench| {
    let filepath = path_resolve("assets/var.less");
    let mut options = ParseOption::default();
    options.hooks.content_interceptor = None;
    let app = Application::new(options, None).unwrap();
    app.parse(filepath.as_str()).unwrap();
    bench.iter(|| {
      app
        .context
        .lock()
        .unwrap()
        .recovery_parse_object(filepath.as_str())
        .unwrap();
    });
  });

  c.bench_function("render_less_arco_pro_bench", |bench| {
    bench.iter(|| {
      // 处理过程
      let filepath = path_resolve("assets/arco-pro/13.less");
      let mut options = ParseOption::default();
      options.hooks.content_interceptor = None;
      let app = Application::new(options, None).unwrap();
      app.render(filepath.as_str()).unwrap();
    });
  });

  c.bench_function("render_less_arco_pro_bench_without_sourcemap", |bench| {
    bench.iter(|| {
      // 处理过程
      let filepath = path_resolve("assets/arco-pro/13.less");
      let app = Application::new(
        ParseOption {
          include_path: vec![],
          sourcemap: false,
          tabspaces: 2,
          modules: None,
          hooks: ParseHooks {
            content_interceptor: None,
            ..Default::default()
          },
        },
        Some(filepath.clone()),
      )
      .unwrap();
      app.render(filepath.as_str()).unwrap();
    });
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
