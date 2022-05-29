use crate::css::value::ValueNode;
use crate::extend::vec_str::VecCharExtend;
use crate::token::ident::IdentType;
use std::cmp::Ordering;

impl ValueNode {
  ///
  /// 代码转化 都 转化成 无变量 实参
  /// 用于 (变量计算)
  ///
  pub fn get_no_var_ident_list(&self) -> Result<Vec<IdentType>, String> {
    let list = self.word_ident_list.clone();
    if list.is_empty() {
      return Err(format!(
        "code_gen content {} is has error, value ident is empty!",
        self.charlist.poly()
      ));
    }
    Ok(list)
  }

  fn get_safe(index: usize, list: &Vec<IdentType>) -> Option<&IdentType> {
    if index < list.len() {
      list.get(index)
    } else {
      None
    }
  }

  fn get_mut_safe(index: usize, list: &mut Vec<IdentType>) -> Option<&mut IdentType> {
    if index < list.len() {
      list.get_mut(index)
    } else {
      None
    }
  }

  fn scan_calc_expr_replace(list: &mut Vec<IdentType>) -> Result<(), String> {
    // 寻找可能的锚点
    let mut index = 0;
    let mut calc_vec = vec![];
    while index < list.len() {
      let current = Self::get_safe(index, list).unwrap();
      let next = Self::get_safe(index + 1, list);
      if *current == IdentType::Word("calc".into())
        && next == Some(&IdentType::Brackets("(".into()))
      {
        calc_vec.push(index + 1);
      }
      index += 1;
    }
    for index in calc_vec {
      let mut cur = index;
      let mut level = 0;
      while cur < list.len() {
        let current = Self::get_mut_safe(cur, list).unwrap();
        // 增减开始
        if current == &IdentType::Brackets("(".into()) {
          level += 1;
        } else if current == &IdentType::Brackets(")".into()) {
          level -= 1;
        }
        // 处理逻辑
        match level.cmp(&0) {
          Ordering::Equal => {
            break;
          }
          Ordering::Greater => {
            if let IdentType::Operator(op) = current {
              *current = IdentType::Word(op.clone());
            }
          }
          Ordering::Less => {}
        }

        cur += 1;
      }
    }

    Ok(())
  }

  ///
  /// 代码生成
  ///
  pub fn code_gen(&self) -> Result<String, String> {
    let mut no_var_list = self.get_no_var_ident_list()?;
    Self::scan_calc_expr_replace(&mut no_var_list)?;
    let res = Self::group_calc_ident_value(no_var_list)?;
    Ok(res)
  }

  ///
  /// 计算 提纯后 根据所有 词的 性质进行组合
  /// 用于 (运算)
  ///
  pub fn group_calc_ident_value(list: Vec<IdentType>) -> Result<String, String> {
    // 非计算词性
    let mut nature_list: Vec<IdentType> = vec![];
    // 计算词性
    let mut calc_list: Vec<IdentType> = vec![];
    // 下标

    // 逆向查找第一个 非空格 的元素
    // 左值 重要
    let find_no_space_node_rev = |nlist: &Vec<IdentType>| {
      for item in nlist.iter().rev() {
        if !matches!(item, IdentType::Space) {
          return Some(item.clone());
        }
      }
      None
    };

    // 遍历 范式
    for now in list {
      // 比对词性
      // let now = list.get(index).unwrap().clone();
      match now {
        IdentType::Operator(op) => {
          if !calc_list.is_empty() {
            let last_calc_item = find_no_space_node_rev(&calc_list).unwrap();
            if matches!(last_calc_item, IdentType::Number(..)) {
              calc_list.push(IdentType::Operator(op));
            } else {
              return Err(format!("operatar char is repeat {}", op));
            }
          } else {
            nature_list.push(IdentType::Word(op));
          }
        }
        IdentType::Number(..) => {
          if calc_list.is_empty() {
            calc_list.push(now);
          } else {
            let last_calc_item = find_no_space_node_rev(&calc_list).unwrap();
            if matches!(last_calc_item, IdentType::Operator(..))
              || matches!(last_calc_item, IdentType::Brackets(..))
            {
              calc_list.push(now);
            } else if matches!(last_calc_item, IdentType::Number(..)) {
              let calc_number = IdentType::calc_value(&calc_list.clone())?;
              nature_list.push(calc_number);
              calc_list.clear();
              calc_list.push(now);
            }
          }
        }
        IdentType::Var(_) => {
          return Err("get_no_var_ident_list -> func has error!".to_string());
        }
        IdentType::Prop(_) => {
          return Err("$abc is not support".to_string());
        }
        IdentType::InsertVar(_) => {
          return Err("@{abc} is not support".to_string());
        }
        IdentType::StringConst(op)
        | IdentType::Word(op)
        | IdentType::Color(op)
        | IdentType::KeyWord(op) => {
          if !calc_list.is_empty() {
            let calc_number = IdentType::calc_value(&calc_list.clone())?;
            nature_list.push(calc_number);
            calc_list.clear();
          }
          nature_list.push(IdentType::Word(op));
        }
        IdentType::Space => {
          if !calc_list.is_empty() {
            calc_list.push(now);
          } else {
            nature_list.push(now);
          }
        }
        IdentType::Escaping(_) => {
          return Err("(min-width: 768px) | ~'min-width: 768px'  is not support".to_string());
        }
        IdentType::Brackets(br) => {
          if !calc_list.is_empty() {
            if br == "(" || br == "[" {
              calc_list.push(IdentType::Brackets(br));
            } else {
              let last_bracket = {
                let mut ident: Option<&IdentType> = None;
                for item in calc_list.iter().rev() {
                  if matches!(item, IdentType::Brackets(..)) {
                    ident = Some(item);
                  }
                }
                ident
              };
              if let Some(IdentType::Brackets(cc)) = last_bracket {
                if cc == "(" || cc == "[" {
                  calc_list.push(IdentType::Brackets(br));
                } else {
                  if !calc_list.is_empty() {
                    let calc_number = IdentType::calc_value(&calc_list)?;
                    nature_list.push(calc_number);
                    calc_list.clear();
                  }
                  nature_list.push(IdentType::Brackets(br));
                }
              } else {
                if !calc_list.is_empty() {
                  let calc_number = IdentType::calc_value(&calc_list)?;
                  nature_list.push(calc_number);
                  calc_list.clear();
                }
                nature_list.push(IdentType::Brackets(br));
              }
            }
          } else {
            if !calc_list.is_empty() {
              let calc_number = IdentType::calc_value(&calc_list)?;
              nature_list.push(calc_number);
              calc_list.clear();
            }
            nature_list.push(IdentType::Brackets(br));
          }
        }
      }
      // index += 1;
    }
    // while index < list.len() {}
    if !calc_list.is_empty() {
      let calc_number = IdentType::calc_value(&calc_list)?;
      nature_list.push(calc_number);
      calc_list.clear();
    }

    let mut res = String::new();
    for (index, item) in nature_list.iter().enumerate() {
      let last = if index > 0 {
        Some(nature_list.get(index - 1).unwrap().clone())
      } else {
        None
      };
      // let last = Some(nature_list.get(index - 1).unwrap().clone());

      match item {
        IdentType::Number(value, unit) => {
          if matches!(last, Some(IdentType::Word(..)))
            || matches!(last, Some(IdentType::Number(..)))
          {
            res += " ";
          }
          res.push_str(value);
          res.push_str(&unit.clone().unwrap_or_default());
        }
        IdentType::Word(char) => {
          if matches!(last, Some(IdentType::Word(..)))
            || matches!(last, Some(IdentType::Number(..)))
          {
            res.push(' ');
          }
          res.push_str(char);
        }
        IdentType::Space => {
          if !matches!(last, Some(IdentType::Space)) {
            res.push(' ');
          }
        }
        IdentType::Brackets(br) => {
          // todo fix single number situation
          res.push_str(br);
        }
        _ => {}
      }
    }

    Ok(res)
  }
}
