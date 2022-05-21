use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

///
/// 内置结构体
///
pub struct HookData<T> {
  pub value: Option<T>,
}

pub type HookType<T> = (Weak<RefCell<HookData<T>>>, Box<dyn Fn(T)>);
// (Weak<RefCell<HookData<String>>>, Box<dyn Fn(&str)>)

///
/// 仿照 React-hooks 把String 放到 heap 上 来执行
///
pub fn create_hooks_str(init: Option<String>) -> HookType<String> {
  let content = Rc::new(RefCell::new(HookData { value: init }));
  let weak_ref = Rc::downgrade(&content);
  let change: Box<dyn Fn(String)> = Box::new(move |txt: String| {
    let value_now = content.deref().borrow().deref().value.clone();
    content.borrow_mut().deref_mut().value =
      Some(value_now.unwrap_or_else(|| "".to_string()) + txt.as_str());
  });
  (weak_ref, change)
}
