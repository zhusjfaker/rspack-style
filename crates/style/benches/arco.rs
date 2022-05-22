use rspack_style::new_less::file::path_resolve;
use rspack_style::new_less::{applicationn::Application, option::ParseOption};

use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! less_bench {
  ($name:expr, $criterion:ident) => {
    let arco_path = format!("assets/arco-pro/{}.less", $name);
    $criterion.bench_function(&arco_path, |b| {
      b.iter(|| {
        let filepath = path_resolve(&arco_path);
        let mut options = ParseOption::default();
        options.hooks.content_interceptor = None;
        let app = Application::new(options, None).unwrap();
        app.render(filepath.as_str()).unwrap();
      })
    });
  };
}

fn criterion_benchmark(c: &mut Criterion) {
  less_bench!(1, c);
  less_bench!(2, c);
  less_bench!(3, c);
  less_bench!(4, c);
  less_bench!(5, c);
  less_bench!(6, c);
  less_bench!(7, c);
  less_bench!(8, c);
  less_bench!(9, c);
  less_bench!(10, c);
  less_bench!(11, c);
  less_bench!(12, c);
  less_bench!(13, c);
  less_bench!(14, c);
  less_bench!(15, c);
  less_bench!(16, c);
  less_bench!(17, c);
  less_bench!(18, c);
  less_bench!(19, c);
  less_bench!(20, c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
