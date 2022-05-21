use rspack_style::extend::string::StringExtend;
use rspack_style::extend::time::wastetime;
use rspack_style::extend::vec_str::VecCharOptionalExtend;
use rspack_style::new_less::applicationn::Application;
use rspack_style::new_less::comment::{skip_comment, Comment};
use rspack_style::new_less::file::{path_resolve, readfile};

#[test]
fn test_comment_remove() {
  let record = wastetime("test_less");
  let filepath = path_resolve("assets/comment.less");
  let app = Application::default();
  {
    app.context.lock().unwrap().option.hooks.content_interceptor = None;
  }
  let node = app.parse(filepath.as_str()).unwrap();
  let content = node.info.borrow().rm_comment();
  record();
  let target = r#"
@height: 300px;

textarea {
  width: @height;
}
    "#;
  assert_eq!(
    content.simple_compare(),
    target.to_string().simple_compare()
  );
}

#[test]
fn test_skip_comment() {
  let start_record = wastetime("test_less");
  // 处理过程
  let filepath = path_resolve("assets/demo.less");
  let conetnt = readfile(&filepath).unwrap().to_char_vec();
  let mut i = 0;
  let mut skipcall = skip_comment();
  while i < conetnt.len() {
    let word = conetnt.try_getword(i, 2).unwrap();
    let char_val = conetnt.get(i).unwrap();
    let old_i = i;
    let skip_res = skipcall(word, char_val.clone(), &mut i);
    if !skip_res && old_i == i {
      print!("{}", char_val);
    }
    i += 1;
  }
  start_record();
  println!("........");
}
