use crate::extend::string::StringExtend;
use crate::new_less::select::{NewSelector, Paradigm};

#[test]
fn test_select_paradigm_parse() {
  let demo_select_list = vec![r#".a .b"#.to_string(), r#"> & .a"#.to_string()];

  let mut has_error = 0;
  demo_select_list.into_iter().for_each(|tt| {
    let mut obj = NewSelector::new(tt.tocharlist(), None, None, None, None);
    match obj.parse(None) {
      Ok(_) => {
        println!("{:#?}", obj.paradigm_vec);
      }
      Err(_) => {
        has_error += 1;
      }
    };
  });

  assert_eq!(has_error, 0);
}

#[test]
fn test_select_parse() {
  let demo_select_list = vec![
    r#".a .b"#.to_string(),
    r#".a>.b"#.to_string(),
    r#"h1>.b"#.to_string(),
    r#"h1>#b1"#.to_string(),
    r#"h1~#b"#.to_string(),
    r#"h1~textarea"#.to_string(),
    r#"h1~*textarea"#.to_string(),
    r#"h1~img"#.to_string(),
    r#"*h1~*textarea"#.to_string(),
    r#".a.b"#.to_string(),
    r#"*.a+*.b"#.to_string(),
    r#">a"#.to_string(),
    r#">.b"#.to_string(),
    r#".b > a"#.to_string(),
    r#"p::first-line"#.to_string(),
    r#"selector:pseudo-class"#.to_string(),
    r#".a[id="xyz"]"#.to_string(),
    r#":global"#.to_string(),
    r#":global(.abc)"#.to_string(),
  ];

  let target = r#"
.a .b
.a>.b
h1>.b
h1>#b1
h1~#b
h1~textarea
h1~*textarea
h1~img
*h1~*textarea
.a.b
*.a+*.b
>a
>.b
.b>a
p::first-line
selector:pseudo-class
.a[id="xyz"]
:global
:global(.abc)
  "#;

  let mut base = "".to_string();
  demo_select_list.into_iter().for_each(|tt| {
    let mut obj = NewSelector::new(tt.tocharlist(), None, None, None, None);
    match obj.parse(None) {
      Ok(_) => {
        println!("{:#?}", obj.paradigm_vec);
      }
      Err(msg) => {
        println!("{:?}", msg);
      }
    }
    let value;
    if obj.paradigm_vec.len() < 2 {
      value = obj
        .paradigm_vec
        .iter()
        .map(|x| x.tostr())
        .collect::<Vec<String>>()
        .join("");
    } else {
      value = obj
        .paradigm_vec
        .iter()
        .map(|x| x.tostr())
        .collect::<Vec<String>>()
        .join(" -> ");
    }
    base += &value;
    println!("{:?}", value);
  });
  assert_eq!(base.simple_compare(), target.to_string().simple_compare());
}

#[test]
fn test_select_error_parse() {
  let mut haserror = 0;
  let demo_select_list = vec![
    r#"."#.to_string(),
    r#"$"#.to_string(),
    r#".b > > a"#.to_string(),
    r#".b$"#.to_string(),
    r#".b.c!"#.to_string(),
    r#".a[*id="xyz"]>.c"#.to_string(),
    r#"(@id)>.c"#.to_string(),
    r#"(id>.c"#.to_string(),
    r#".a[="xyz"]>.c"#.to_string(),
    r#".a[id="xyz">.c"#.to_string(),
    r#".a[id="xyz>.c"#.to_string(),
    // ------
    // r#".b@"#.to_string(),
    // ------
  ];
  demo_select_list.into_iter().for_each(|tt| {
    let mut obj = NewSelector::new(tt.tocharlist(), None, None, None, None);
    match obj.parse(None) {
      Ok(_) => {
        haserror += 1;
      }
      Err(msg) => {
        haserror += 0;
        println!("{:?}", msg);
      }
    }
  });
  assert_eq!(haserror, 0)
}
