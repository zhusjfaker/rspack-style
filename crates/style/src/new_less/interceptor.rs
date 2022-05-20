use crate::new_less::file::path_resolve;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, fs};

pub struct LessInterceptor;

impl LessInterceptor {
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

  ///
  /// js-lib 文件管理
  /// 用来支持 less.js cwd require.resolve js-plugin
  ///
  fn filemanger() -> Result<String, String> {
    let cwd = env::current_dir().unwrap();
    let temp_rspack_style_dir = cwd.join(".rspack-style");
    let js_main_file = temp_rspack_style_dir.join("dist/main.js");

    return if temp_rspack_style_dir.exists() && temp_rspack_style_dir.is_dir() {
      if js_main_file.exists() {
        // 主文件存在 则返回
        Ok(js_main_file.into_os_string().into_string().unwrap())
      } else {
        // 主文件不存在 则 从 自己 js-lib 中进行复制
        Self::copy_js_lib(temp_rspack_style_dir.to_str().unwrap());
        Ok(js_main_file.into_os_string().into_string().unwrap())
      }
    } else if !temp_rspack_style_dir.exists() {
      fs::create_dir_all(temp_rspack_style_dir.clone()).unwrap();
      Self::copy_js_lib(temp_rspack_style_dir.to_str().unwrap());
      Ok(js_main_file.into_os_string().into_string().unwrap())
    } else {
      // maybe .rspack-style is link or file
      Err(format!(
        "rspack-style LessInterceptor filemanger call ->  {} must be dir",
        temp_rspack_style_dir
          .into_os_string()
          .into_string()
          .unwrap()
      ))
    };
  }

  pub fn handle(filepath: &str, content: &str) -> Result<String, String> {
    let path = Path::new(filepath);
    if path.extension() == Some("less".to_string().as_ref()) {
      let self_dir = Path::new(filepath)
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
      let cwd = path_resolve("");
      let include_path = vec![
        Value::String(self_dir),
        Value::String(cwd.clone()),
        Value::String("node_modules".to_string()),
      ];
      let mut option_map = HashMap::new();
      option_map.insert("paths", Value::Array(include_path));
      option_map.insert("filename", Value::String(filepath.to_string()));
      let mut content_map = HashMap::new();
      content_map.insert("content".to_string(), content.to_string());
      let content_arg = serde_json::to_string(&content_map).unwrap();
      let option_arg = serde_json::to_string(&option_map).unwrap();
      let js_file = Self::filemanger()?;
      let mut task = Command::new("node");
      task.arg(js_file.as_str());
      task.arg("--content");
      task.arg(content_arg.as_str());
      task.arg("--option");
      task.arg(option_arg.as_str());
      task.current_dir(cwd);
      task.stdout(Stdio::piped());

      // let _test_node_content = format!("node {} --content {} --option {}", js_file, content_arg, option_arg);

      let output = task
        .output()
        .expect(format!("{}->less.js callback is failed", filepath).as_str());
      let status = output.status.code().unwrap();
      let less_content = std::str::from_utf8(&*output.stdout).unwrap().to_string();
      return if status > 0 {
        Err(format!(
          "parse less file ->{} \n has error in interceptor,\n ex is \n {}",
          filepath, less_content
        ))
      } else {
        Ok(less_content)
      };
      // let content = "";
    }
    Ok(content.to_string())
  }
}
