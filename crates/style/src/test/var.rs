use crate::extend::string::StringExtend;
use crate::new_less::context::Context;
use crate::new_less::var::HandleResult;
use crate::new_less::var_node::VarNode;

#[test]
fn test_var_parse() {
  let vars_list = vec![r#"@width:400px;"#.to_string()];
  let mut haserror = 0;
  vars_list.into_iter().for_each(|tt| {
    match VarNode::new(tt.tocharlist(), None, None, None, Context::default()) {
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
fn test_var_error_parse() {
  let mut haserror = 0;
  let demo_select_list = vec![
    r#"@widt h:400px;"#.to_string(),
    r#"@#width:400px;"#.to_string(),
    r#"@wid,th:400px;"#.to_string(),
    r#"@widt
    h:400px;"#
      .to_string(),
    r#""#.to_string(),
  ];
  demo_select_list.into_iter().for_each(|tt| {
    match VarNode::new(tt.tocharlist(), None, None, None, Context::default()) {
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
