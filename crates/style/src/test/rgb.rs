use crate::extend::string::StringExtend;
use crate::new_less::ident::IdentType;
use crate::new_less::rgb::rgb_calc;
use crate::new_less::value::ValueNode;

#[test]
fn test_color_calc() {
  let rgb = vec![
    IdentType::Number("255".to_string(), None),
    IdentType::Number("255".to_string(), None),
    IdentType::Number("255".to_string(), None),
  ];
  let res = rgb_calc(rgb.iter().map(|x| x).collect::<Vec<&IdentType>>()).unwrap();

  let rgb2 = vec![
    IdentType::Number("220".to_string(), None),
    IdentType::Number("220".to_string(), None),
    IdentType::Number("220".to_string(), None),
  ];

  let res2 = rgb_calc(rgb2.iter().map(|x| x).collect::<Vec<&IdentType>>()).unwrap();

  let rgb3 = vec![
    IdentType::Number("112".to_string(), None),
    IdentType::Number("128".to_string(), None),
    IdentType::Number("144".to_string(), None),
  ];

  let res3 = rgb_calc(rgb3.iter().map(|x| x).collect::<Vec<&IdentType>>()).unwrap();

  let rgb4 = vec![
    IdentType::Number("119".to_string(), None),
    IdentType::Number("136".to_string(), None),
    IdentType::Number("153".to_string(), None),
  ];

  let res4 = rgb_calc(rgb4.iter().map(|x| x).collect::<Vec<&IdentType>>()).unwrap();

  println!("{},{},{},{}", res, res2, res3, res4);

  assert_eq!(res, "#ffffff".to_string());
  assert_eq!(res2, "#dcdcdc".to_string());
  assert_eq!(res3, "#708090".to_string());
  assert_eq!(res4, "#778899".to_string());
}

#[test]
fn test_color_render() {
  let vars_list = vec![
    // r#"linear-gradient(180deg, rgb(242 249 254) 0%, #e6f4fe 100%);"#.to_string(),
    // r#"linear-gradient(180deg, rgb(242 249 254 / 100%) 0%, #e6f4fe 100%);"#.to_string(),
    // r#"linear-gradient(180deg, rgb(245 254 242) 0%, rgb(230 254 238) 100%);"#.to_string(),
    r#"4px 4px 10px rgba(0 , 0 , 0 , 10%)"#.to_string(),
  ];

  let res = 0;

  vars_list.into_iter().for_each(
    |tt| match ValueNode::new(tt.tocharlist(), None, None, None) {
      Ok(mut obj) => {
        ValueNode::scan_rgb_expr_calc_replace(&mut obj.word_ident_list).unwrap();
        println!("{:#?}", obj.word_ident_list);
      }
      Err(msg) => {
        println!("{}", msg);
      }
    },
  );

  assert_eq!(res, 0);
}
