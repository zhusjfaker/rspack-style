use rspack_style::extend::string::StringExtend;
use rspack_style::sourcemap::loc::{Loc, LocMap};
use rspack_style::style_core::applicationn::Application;
use rspack_style::util::file::{path_resolve, readfile};

///
/// 测试字典方法
///
#[test]
fn test_loc() {
  let content = readfile(path_resolve("assets/loc.less").as_str()).unwrap();
  let obj = LocMap::new(&content.to_char_vec());
  let c = obj.get(0).unwrap();
  let x = obj.getloc(4, 10).unwrap();
  assert_eq!(c.char, '@');
  assert_eq!(x.char, '@');
}

#[test]
fn test_loc_rule() {
  let filepath = path_resolve("assets/loc_rule.less");
  let app = Application::default();
  let file = app.parse(filepath.as_str()).unwrap().to_less().unwrap();
  let json = serde_json::to_string_pretty(&file).unwrap();
  println!("json->{}", json);
  let list = file.collect_loc_list();
  assert_eq!(
    list.contains(&Some(Loc {
      line: 4,
      col: 5,
      char: 'h',
      index: 5
    })),
    true
  );
  assert_eq!(
    list.contains(&Some(Loc {
      line: 3,
      col: 4,
      char: 'b',
      index: 1
    })),
    true
  );
  println!("list->{:#?}", list);
}
