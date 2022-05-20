use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

#[test]
fn test_less_import_hook() {
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  app.context.lock().unwrap().option.hooks.import_alias = Some(Arc::new(|filepath, importpath| {
    println!("{},{}", filepath, importpath);
    Ok(importpath)
  }));
  let res = app.render(filepath.as_str()).unwrap();
  println!("{}", res);
}

#[test]
fn test_less_content_hook() {
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  let res = app.render(filepath.as_str()).unwrap();
  println!("rust_res: -> \n {}", res);
}

#[test]
#[ignore]
fn test_js_lib_copy() {
  ///
  /// 复制js-lib
  /// 检查有文件的话 不进行调用
  ///
  fn copy_js_lib(target_dir: &str) {
    // 清空dir
    let command = format!("rm -rf {}/**", target_dir);
    let mut rm_task = Command::new("sh");
    rm_task.arg("-c").arg(command);
    rm_task.current_dir(target_dir).status().unwrap();
    // 拷贝js-lib
    let js_lib_dir = format!("{}/**", path_resolve("js-lib"));
    let command = format!("cp -rf {} {}", js_lib_dir, target_dir);
    let mut cp_task = Command::new("sh");
    cp_task.arg("-c").arg(command);
    cp_task.current_dir(target_dir).status().unwrap();
  }

  let target_dir = path_resolve(".rspack-style");
  copy_js_lib(target_dir.as_str());

  let res = Path::new(path_resolve(".rspack-style/dist/main.js").as_str()).exists();

  assert_eq!(res, true);
}
