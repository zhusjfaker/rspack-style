use criterion::{criterion_group, criterion_main, Criterion};
use rspack_style::style_core::applicationn::Application;
use rspack_style::util::file::path_resolve;

fn criterion_benchmark(c: &mut Criterion) {
  // c.bench_function("render-arco-pro-css", |bench| {
  //   bench.iter(|| {
  //     let dir = path_resolve("assets/arco-pro-css");
  //     let app = Application::default();
  //     for entry in std::path::Path::new(&dir).read_dir().unwrap() {
  //       let file = entry.unwrap().path();
  //       if file.is_file() && file.extension().unwrap().to_str().unwrap() == "css" {
  //         let css_file = file.to_str().unwrap();
  //         app.render(css_file).unwrap();
  //       }
  //     }
  //   });
  // });

  c.bench_function("render-arco-pro-css", |bench| {
    bench.iter(|| {
      let filepath = path_resolve("assets/arco-pro-css/src-style-global.css");
      let app = Application::default();
      app.render(filepath.as_str()).unwrap();
    });
  });

  c.bench_function("render-arco-pro-css", |bench| {
    bench.iter(|| {
      let filepath = path_resolve("assets/arco-pro-css/src-style-global.css");
      let app = Application::default();
      app.set_minify(true);
      app.render(filepath.as_str()).unwrap();
    });
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
