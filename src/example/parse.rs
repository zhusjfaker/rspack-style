use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;
use crate::new_less::option::ParseOption;
use test::Bencher;

#[bench]
fn parse_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
    context.render(filepath).unwrap();
  });
}

#[bench]
fn parse_var_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/var.less");
    let context = Context::new(
      ParseOption {
        include_path: vec![],
        sourcemap: false,
        tabspaces: 2,
        hooks: Default::default(),
      },
      Some(filepath.clone()),
    )
    .unwrap();
    context.render(filepath).unwrap();
  });
}

#[bench]
fn render_less_arco_pro_bench(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/arco-pro/13.less");
    let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
    context.render(filepath).unwrap();
  });
}

#[bench]
fn render_less_arco_pro_bench_without_sourcemap(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/arco-pro/13.less");
    let context = Context::new(
      ParseOption {
        include_path: vec![],
        sourcemap: false,
        tabspaces: 2,
        hooks: Default::default(),
      },
      Some(filepath.clone()),
    )
    .unwrap();
    context.render(filepath).unwrap();
  });
}
