use crate::extend::string::StringExtend;
use crate::new_less::media::MediaQuery;
use crate::new_less::var::HandleResult;

#[test]
fn test_media_parse() {
  let demo_select_list = vec![
    r#"@media screen and ( max-width: 900px)"#.to_string(),
    r#"@media screen and ( max-width  : 900px)"#.to_string(),
    r#"@media screen and (min-width: 900px) and screen and (max-width: 900px)"#.to_string(),
  ];
  let mut haserror = 0;
  demo_select_list.into_iter().for_each(|tt| {
    match MediaQuery::new(tt.tocharlist(), None, None, None) {
      HandleResult::Success(obj) => {
        haserror += 0;
        println!("{:#?}", obj);
      }
      HandleResult::Fail(msg) => {
        haserror += 1;
        println!("{:?}", msg);
      }
      HandleResult::Swtich => {}
    };
  });
  assert_eq!(haserror, 0);
}

#[test]
fn test_media_error_parse() {
  let mut haserror = 0;
  let demo_select_list = vec![
    r#"@media screen and ( a: 900px:)"#.to_string(),
    r#" "#.to_string(),
    r#""#.to_string(),
  ];
  demo_select_list.into_iter().for_each(|tt| {
    match MediaQuery::new(tt.tocharlist(), None, None, None) {
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
