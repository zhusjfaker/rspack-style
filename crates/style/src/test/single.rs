use crate::extend::rs_hooks::{create_hooks_str, HookData};
use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecCharExtend;
use crate::extend::vec_str::VecCharOptionalExtend;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use uuid::Uuid;

#[test]
fn test_str() {
  let strore = "123456";
  let _a = &strore[0..2];
  let _b = &strore[1..3];
  let index_1 = strore.to_string().indexOf("23", Some(2));
  let index_2 = strore.to_string().indexOf("23", Some(1));
  let index_3 = strore.to_string().indexOf("23", None);
  assert_eq!(index_1, -1);
  assert_eq!(index_2, 1);
  assert_eq!(index_3, 1);
}

#[test]
fn test_slice() {
  let a = "1233284920348aljdfalkdfjalkfdj023180";
  let mut i = 0;
  let target = "332";
  let mut copy: String = "".to_string();
  loop {
    if i < a.len() {
      let word = a.to_string().tocharlist().try_getword(i, 3).unwrap();
      // println!("word is {} ...", word);
      if i == 2 {
        copy = word.clone();
      }
    } else {
      break;
    }
    i += 1;
  }
  assert_eq!(copy.as_str(), target);
}

#[derive(Clone)]
struct Student {
  pub name: String,
  pub classmate: Option<Rc<RefCell<Student>>>,
}

impl Student {
  fn new() -> Student {
    Student {
      name: "1".to_string(),
      classmate: None,
    }
  }

  fn new_classmate(mut self) -> Student {
    self.name = "abc".to_string();
    let classmate = Rc::new(RefCell::new(self));
    Student {
      name: "2".to_string(),
      classmate: Some(classmate),
    }
  }
}

#[test]
fn test_context_fn() {
  let a = Student::new();
  let b = a.new_classmate();
  println!(".......{}", b.classmate.unwrap().deref().borrow().name);
  println!(".......");
}

async fn add(num: Rc<RefCell<i32>>, flag: String) {
  let p = flag.parse::<i32>().unwrap();
  if p % 2 == 0 {
    sleep(Duration::from_secs(2)).await;
  }
  let mut i = 0;
  while i < 3 {
    *num.deref().borrow_mut().deref_mut() += 1;
    i += 1;
    let c = *num.deref().borrow().deref();
    println!("flag is {} num is {}", flag, c);
  }
}

#[test]
fn test_future() {
  let num = Rc::new(RefCell::new(1));
  let rt = tokio::runtime::Runtime::new().unwrap();
  let d = rt.block_on(async {
    let exec_times: i32 = 20;
    let mut task_list = vec![];
    let mut index = 0;
    while index < exec_times {
      let task = add(num.clone(), index.to_string());
      task_list.push(task);
      index += 1;
    }
    futures::future::join_all(task_list).await;
    num
  });
  let c = *d.deref().borrow().deref();
  println!("{}", c);
  println!("........");
}

#[test]
fn test_tokio() {
  let rt = tokio::runtime::Runtime::new().unwrap();
  rt.block_on(async {
    sleep(Duration::from_secs(2)).await;
    println!("100 ms have elapsed");
  });
}

async fn add_mutex(num: &Mutex<i32>, flag: String) {
  let p = flag.parse::<i32>().unwrap();
  if p % 2 == 0 {
    sleep(Duration::from_secs(2)).await;
  }
  let mut i = 0;
  while i < 3 {
    let mut val = num.lock().await;
    *val += 1;
    i += 1;
    println!("flag is {} num is {}", flag, val);
  }
}

#[test]
fn test_tokio_mutex() {
  let num = Mutex::new(1);
  let rt = tokio::runtime::Runtime::new().unwrap();
  rt.block_on(async {
    let exec_times: i32 = 5;
    let mut task_list = vec![];
    let mut index = 0;
    while index < exec_times {
      let task = add_mutex(&num, index.to_string());
      task_list.push(task);
      index += 1;
    }
    futures::future::join_all(task_list).await;
  });
  let c = rt.block_on(async { num.lock().await.clone().to_string() });
  println!("{}", c);
  println!("........");
}

#[test]
fn test_rc() {
  let a = Rc::new("123");
  println!("0->{}", Rc::strong_count(&a));
  let mut list = vec![];
  list.push(a.clone());
  println!("1->{}", Rc::strong_count(&a));
  {
    let mut list = vec![];
    list.push(a.clone());
    println!("2->{}", Rc::strong_count(&a));
  }
  list.remove(0);
  println!("3->{}", Rc::strong_count(&a));
}

#[test]
fn test_display() {
  let str = "我是谁".to_string();
  let test = (
    str.charAt(None).unwrap(),
    str.charAt(Some(0)).unwrap(),
    str.charAt(Some(-1)).unwrap(),
  );
  println!("{:#?}", test);
  assert_eq!(test.0.as_str(), "我");
  assert_eq!(test.1.as_str(), "我");
  assert_eq!(test.2.as_str(), "");
}

#[test]
fn test_loop() {
  let mut a: Option<String> = None;
  let mut list = vec![];
  let mut index = 0;
  while index < 10 {
    if a.is_none() {
      println!(".....");
    }
    a = Some("456".to_string());
    list.push(a.as_ref().unwrap().clone());
    index += 1;
  }
}

#[test]
fn test_clousure() {
  fn exec() -> Weak<RefCell<HookData<String>>> {
    let (a, change) = create_hooks_str(Some("aaaa".to_string()));
    let c = a.upgrade().unwrap().deref().borrow().deref().value.clone();
    println!("{:?}", c);
    change("123".to_string());
    let b = a.upgrade().unwrap().deref().borrow().deref().value.clone();
    println!("{:?}", b);
    change("456".to_string());
    let x = a.upgrade().unwrap().deref().borrow().deref().value.clone();
    println!("{:?}", x);
    println!("{}", Rc::strong_count(&a.upgrade().unwrap()));
    println!(".......");
    a
  }
  let _x = exec();
  println!(".......");
}

#[test]
fn test_clousure_loop() {
  fn call() -> Box<dyn FnMut(Option<String>) -> String> {
    let mut content = "".to_string();
    Box::new(move |txt: Option<String>| -> String {
      content = content.clone() + &txt.unwrap_or("".to_string());
      content.clone()
    })
  }
  let mut exec = call();
  let mut i = 0;
  while i < 5 {
    exec(Some("a".to_string()));
    i += 1;
  }
  let m: String = exec(None);
  assert_eq!(&m, "aaaaa");
}

#[test]
fn test_uuid() {
  let a = Uuid::new_v4().to_string();
  println!("{}", a);
  assert_eq!("f9c397f9-7528-4b8b-963b-c09c36c40ece".len(), a.len());
}

#[test]
fn test_expression() {
  let a = "1+2*((3+4*5)*6)";
  let b = "20";
  let mut ns = fasteval::EmptyNamespace;
  let c = fasteval::ez_eval(a, &mut ns).unwrap();
  let d = fasteval::ez_eval(b, &mut ns).unwrap();
  println!("....,{},{}", c, d);
  assert_eq!(&c.to_string(), "277");
  assert_eq!(&d.to_string(), "20");
}

#[test]
fn test_trim() {
  let list = vec![' ', ' ', '1', '2', ' ', '3', '\r', '\n', ' '];
  println!("{:#?}", list.trim());
  println!("{:#?}", list.trim_start());
}

#[test]
fn test_vec_equal() {
  let cc = "@importx";
  let list = cc.chars().collect::<Vec<char>>();
  let mut res = 0;
  if list[0..7] == vec!['@', 'i', 'm', 'p', 'o', 'r', 't'] {
    res = 1;
  }
  assert_eq!(res, 1);
}

#[test]
fn test_rev() {
  let list = vec![1, 2, 3];
  for (index, val) in list.iter().rev().enumerate() {
    println!("{:#?} {:#?}", list.len() - 1 - index, val)
  }
}

#[test]
fn test_splic_vec() {
  let mut list = vec!["abc", "cde", "bfg", "hsk", "dks"];
  let mut new_list = list[0..1].to_vec();
  let new1 = list[1..2].to_vec();
  let new2 = list[3..4].to_vec();
  new_list.append(&mut list[4..5].to_vec());
  println!("{:#?}{:#?}{:#?}", new_list, new1, new2);
  let b = list.get_mut(3).unwrap();
  *b = "xyz";
  let a = list.get(2).unwrap();
  let index = list.iter().position(|x| x == a).unwrap();
  println!("{:#?}", index);

  let mut list = vec![1, 2, 3, 4, 5, 6];
  let insert_list = vec![10, 9, 8];
  list.remove(5);
  for item in insert_list.iter().rev() {
    list.insert(5, item.clone());
  }
  list.remove(2);
  for item in insert_list.iter().rev() {
    list.insert(2, item.clone());
  }
  println!("{:#?}", list);
}
