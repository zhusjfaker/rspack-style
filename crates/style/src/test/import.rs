use crate::extend::string::StringExtend;
use crate::new_less::context::Context;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::import::ImportNode;
use crate::new_less::var::HandleResult;

#[test]
fn test_rel_path() {
  let a = "../test/a.txt".to_string();
  let b = "./test/a.txt".to_string();
  assert_eq!(FileInfo::is_relative_path(&a), true);
  assert_eq!(FileInfo::is_relative_path(&b), true);
}

#[test]
fn test_import_parse() {
  let import_list = vec![r#"@import'./assets/index.less';"#.to_string()];
  let mut haserror = 0;
  import_list.into_iter().for_each(|tt| {
    match ImportNode::new(
      tt.tocharlist(),
      None,
      None,
      None,
      Context::default(),
      &mut vec![],
    ) {
      HandleResult::Success(obj) => {
        haserror += 0;
        let json = serde_json::to_string_pretty(&obj).unwrap();
        println!("{}", json);
      }
      HandleResult::Fail(msg) => {
        haserror += 1;
        println!("{:?}", msg);
      }
      HandleResult::Swtich => {
        haserror += 1;
        println!("{:?}", "swtich case ....");
      }
    }
  });
  assert_eq!(haserror, 0);
}

#[test]
fn test_import_error_parse() {
  let mut haserror = 0;
  let import_list = vec![r#"@import './a.less";"#.to_string()];
  import_list.into_iter().for_each(|tt| {
    match ImportNode::new(
      tt.tocharlist(),
      None,
      None,
      None,
      Context::default(),
      &mut vec![],
    ) {
      HandleResult::Success(_) => {
        haserror += 1;
      }
      HandleResult::Fail(msg) => {
        haserror += 0;
        println!("{:?}", msg);
      }
      HandleResult::Swtich => {}
    };
  });
  assert_eq!(haserror, 0)
}
