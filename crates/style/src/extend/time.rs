use std::time::Instant;

///
///  统计程序耗时
///
pub fn wastetime(fn_name: &str) -> Box<dyn Fn()> {
  let now = Instant::now();
  let name = fn_name.to_string();
  let call = move || {
    let end = now.elapsed();
    let f = end.as_micros() as f32;
    println!("Running {}() took {} ms", name, f * 0.001);
  };
  Box::new(call)
}
