use rspack_style::extend::string::StringExtend;
use rspack_style::less::style_rule::StyleRuleNode;
use rspack_style::less::var::HandleResult;
use rspack_style::style_core::context::Context;

#[test]
fn test_style_rule_parse() {
  let list = vec![
    // r#"box-sizing: border-box;"#.to_string(),
    // r#"font-size: 10px;"#.to_string(),
    r#"font -size: 10px;"#.to_string(),
  ];
  let mut haserror = 0;
  list.into_iter().for_each(|tt| {
    match StyleRuleNode::new(tt.to_char_vec(), None, None, None, Context::default()) {
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
fn test_style_rule_error_parse() {
  let mut haserror = 0;
  let list = vec![r#"font-size:: 10px;"#.to_string()];
  list.into_iter().for_each(|tt| {
    match StyleRuleNode::new(tt.to_char_vec(), None, None, None, Context::default()) {
      HandleResult::Success(obj) => {
        haserror += 1;
        println!("{:#?}", obj);
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
