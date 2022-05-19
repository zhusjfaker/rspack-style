use crate::new_less::applicationn::Application;
use crate::new_less::file::path_resolve;

#[test]
fn test_less_json_deserializer() {
  // 处理过程
  let filepath = path_resolve("assets/demo.less");
  let app = Application::default();
  let info = app.parse(filepath.as_str()).unwrap();
  let json = serde_json::to_string_pretty(&info).unwrap();
  let res = app
    .context
    .lock()
    .unwrap()
    .recovery_parse_object(filepath.as_str())
    .unwrap();
  let back_json = serde_json::to_string_pretty(&res).unwrap();
  assert_eq!(json, back_json);
}
