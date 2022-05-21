use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;
use unit_test::Bencher;

#[bench]
fn render_1_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/1.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_2_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/2.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_3_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/3.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_4_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/4.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_5_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/5.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_6_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/6.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_7_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/7.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_8_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/8.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_9_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/9.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_10_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/10.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_11_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/11.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_12_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/12.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_13_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/13.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_14_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/14.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_15_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/15.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_16_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/16.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_17_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/17.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_18_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/18.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_19_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/19.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}

#[bench]
fn render_20_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/20.less");
    let app = Application::default();
    app.render(filepath.as_str()).unwrap();
  });
}
