use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecCharExtend;
use crate::new_less::fileinfo::FileWeakRef;
use crate::new_less::ident::IdentType;
use crate::new_less::node::{NodeWeakRef, StyleNode};
use crate::new_less::rgb::rgb_calc;
use crate::new_less::value::ValueNode;
use crate::new_less::var::VarRuleNode;
use std::cmp::Ordering;
use std::ops::Deref;

impl ValueNode {
  ///
  /// 查找变量
  /// 用于 (变量计算)
  ///
  pub fn get_var_by_key(
    &self,
    key: &str,
    rule_info: NodeWeakRef,
    file_info: FileWeakRef,
  ) -> Result<ValueNode, String> {
    if let Some(rule_ref) = rule_info {
      let rule = rule_ref.upgrade().unwrap();
      let nodelist = &rule.borrow().block_node;
      for item in nodelist {
        if let StyleNode::Var(VarRuleNode::Var(var)) = item.deref() {
          if var.key.as_ref().unwrap() == key {
            return Ok(var.value.as_ref().unwrap().clone());
          }
        }
      }
      return if rule.borrow().parent.is_some() {
        // 非顶层 向上递归
        self.get_var_by_key(key, rule.borrow().parent.clone(), None)
      } else {
        // 顶层 同层 引用递归 查看下半段代码
        self.get_var_by_key(key, None, self.fileinfo.clone())
      };
    }
    // 到达顶层后 取当前文件 的 顶层变量 或者 其他引用 文件的 顶层变量
    else if let Some(file_ref) = file_info {
      // 若没有则已经到达 顶层 则按照 顶层处理
      let fileinfo_ref = file_ref.upgrade().unwrap();
      let nodelist = &fileinfo_ref.borrow().block_node;
      for item in nodelist {
        if let StyleNode::Var(VarRuleNode::Var(var)) = item.deref() {
          if var.key.as_ref().unwrap() == key {
            return Ok(var.value.as_ref().unwrap().clone());
          }
        }
      }
      // 获取 其他 引用文件 顶层变量
      let top_level_other_vars = fileinfo_ref.borrow().collect_vars();
      for var in top_level_other_vars {
        if var.key.as_ref().unwrap() == key {
          return Ok(var.value.as_ref().unwrap().clone());
        }
      }
    };

    Err(format!("no var key {} has found", key))
  }

  fn scan_var_ident_from_string_const(&self, txt: &str) -> Result<String, String> {
    let list = txt.to_string().tocharlist();
    let mut res = "".to_string();
    let mut index = 0;
    let mut var = "".to_string();
    while index < list.len() {
      let current = list.get(index).unwrap();
      let next = if index < list.len() - 1 {
        list.get(index + 1)
      } else {
        None
      };
      if *current == '@' && next == Some(&'{') {
        if var.is_empty() {
          index += 1;
          var += "@{"
        } else {
          return Err(format!(
            "{} is contains repeat @ in the {} index",
            txt, index
          ));
        }
      } else if var.len() > 0 {
        var.push(*current);
        if *current == '}' {
          let var_ident = format!("@{}", var.tocharlist()[2..var.len() - 1].to_vec().poly());
          let var_node_value = self.get_var_by_key(
            var_ident.as_str(),
            self.parent.clone(),
            self.fileinfo.clone(),
          )?;
          res += var_node_value.code_gen()?.as_str();
          var = "".to_string();
        }
      } else {
        res.push(*current);
      }
      index += 1;
    }
    Ok(res)
  }

  ///
  /// 递归净化 所有表达式 的 var
  /// 用于 (变量计算)
  ///
  pub fn pure_list(&self, list: &mut Vec<IdentType>) -> Result<(), String> {
    let mut handle_vec: Vec<(usize, Vec<IdentType>)> = vec![];
    let mut string_handle_vec: Vec<(Vec<usize>, Vec<IdentType>)> = vec![];
    for (index, ident) in list.iter().enumerate() {
      if let IdentType::Var(ident_var) = ident {
        let var_node_value =
          self.get_var_by_key(ident_var, self.parent.clone(), self.fileinfo.clone())?;
        handle_vec.push((index, var_node_value.word_ident_list.clone()));
      } else if let IdentType::StringConst(ident_var) = ident {
        // calc ~"" | ~''
        let prev_ident = if index > 0 { list.get(index - 1) } else { None };
        if prev_ident != Some(&IdentType::Word('~'.to_string())) {
          let no_var_str_const = self.scan_var_ident_from_string_const(ident_var)?;
          string_handle_vec.push((vec![index], vec![IdentType::StringConst(no_var_str_const)]));
        } else {
          let no_var_str_const = self.scan_var_ident_from_string_const(ident_var)?;
          string_handle_vec.push((
            vec![index - 1, index],
            vec![IdentType::Word(
              no_var_str_const.replace("'", "").replace("\"", ""),
            )],
          ));
        }
      }
    }
    // 把当前 所有的 变量 -> 代数 ident 插到 目前  ident_list vec 上
    for (index, ident_list) in handle_vec {
      list.remove(index);
      let mut setp = 0;
      ident_list.iter().for_each(|x| {
        list.insert(index + setp, x.clone());
        setp += 1;
      });
    }
    for (index, ident_list) in string_handle_vec {
      let mut remove_count = 0;
      index.iter().for_each(|num| {
        list.remove(*num - remove_count);
        remove_count += 1;
      });
      let mut setp = 0;
      ident_list.iter().for_each(|x| {
        list.insert(index[0] + setp, x.clone());
        setp += 1;
      });
    }

    // let _json = serde_json::to_string_pretty(&list).unwrap();
    // 如果 当前 还有变量 则继续递归 演算
    if list.iter().any(|x| matches!(x, IdentType::Var(_))) {
      self.pure_list(list)?;
    };
    Ok(())
  }

  ///
  /// 代码转化 都 转化成 无变量 实参
  /// 用于 (变量计算)
  ///
  pub fn get_no_var_ident_list(&self) -> Result<Vec<IdentType>, String> {
    let mut list = self.word_ident_list.clone();
    if list.is_empty() {
      return Err(format!(
        "code_gen content {} is has error, value ident is empty!",
        self.charlist.poly()
      ));
    }
    // 把 表达式中 含有 var 声明的 全部进行 查找替换
    self.pure_list(&mut list)?;
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

  ///
  /// 匹配计算
  /// rgb(255 255 255)
  ///
  fn match_rgb_expr_calc(
    mut index: usize,
    list: &Vec<IdentType>,
  ) -> (Option<usize>, Vec<&IdentType>) {
    let mut res: (Option<usize>, Vec<&IdentType>) = (None, vec![]);
    index += 1;
    while index < list.len() {
      let current = Self::get_safe(index, list).unwrap();
      if let IdentType::Number(_, unit) = current {
        if res.1.len() < 4 && unit.is_none() {
          res.1.push(current);
        } else {
          break;
        }
      } else if current == &IdentType::Brackets(')'.to_string()) {
        res.0 = Some(index);
        break;
      } else if !matches!(current, IdentType::Space) && current != &IdentType::Word(','.to_string())
      {
        break;
      }
      index += 1;
    }
    // 匹配结果不符合则重置
    if res.0.is_none() || res.1.len() != 3 {
      res = (None, vec![])
    }
    res
  }

  ///
  /// 匹配计算
  /// rgb(255,255,255)
  ///
  fn match_rgba_expr_calc(
    mut index: usize,
    list: &Vec<IdentType>,
  ) -> (Option<usize>, Vec<&IdentType>) {
    let mut res: (Option<usize>, Vec<&IdentType>) = (None, vec![]);
    index += 2;
    while index < list.len() {
      let current = Self::get_safe(index, list).unwrap();
      if matches!(current, IdentType::Number(..)) {
        if res.1.len() < 5 {
          res.1.push(current);
        } else {
          break;
        }
      } else if current == &IdentType::Brackets(')'.to_string()) {
        res.0 = Some(index);
        break;
      } else if !matches!(current, IdentType::Space)
        && current != &IdentType::Word(','.to_string())
        && current != &IdentType::Operator('/'.to_string())
      {
        break;
      }
      index += 1;
    }
    if res.0.is_none() || res.1.len() < 3 || res.1.len() > 4 {
      res = (None, vec![])
    }
    res
  }

  ///
  /// 扫描词性中 符合 rgb(255,255,255)
  ///
  ///
  pub fn scan_rgb_expr_calc_replace(list: &mut Vec<IdentType>) -> Result<(), String> {
    // 寻找可能的锚点
    let mut index = 0;
    let mut perhaps_rgb_vec = vec![];
    let mut perhaps_rgba_vec = vec![];
    while index < list.len() {
      let current = Self::get_safe(index, list).unwrap();
      let next = Self::get_safe(index + 1, list);
      if *current == IdentType::Word("rgb".to_string())
        && next == Some(&IdentType::Brackets('('.to_string()))
      {
        perhaps_rgb_vec.push(index);
        perhaps_rgba_vec.push(index);
      } else if *current == IdentType::Word("rgba".to_string())
        && next == Some(&IdentType::Brackets('('.to_string()))
      {
        perhaps_rgba_vec.push(index);
      }
      index += 1;
    }
    let mut extra = 0;
    let mut rm_vec: Vec<(usize, usize)> = vec![];
    for start in perhaps_rgb_vec {
      if let (Some(mut end), corlor_list) = Self::match_rgb_expr_calc(start + 1 + extra, list) {
        // 计算 替换 词根
        let rgb_value = rgb_calc(corlor_list)?;
        let final_color_word = IdentType::Color(rgb_value);
        list.insert(start, final_color_word);
        extra += 1;
        end += extra;
        rm_vec.push((start + extra, end));
      }
    }

    let mut rm_count = 0;
    for (rs, re) in rm_vec {
      let start = rs - rm_count;
      let mut end = re - rm_count;
      while end > start - 1 {
        list.remove(end);
        end -= 1
      }
      rm_count += re - rs + 1;
    }

    let mut extra = 0;
    let mut rm_vec: Vec<(usize, usize)> = vec![];
    for start in perhaps_rgba_vec {
      if let (Some(mut end), corlor_list) = Self::match_rgba_expr_calc(start + extra, list) {
        let mut color_txt = "".to_string();
        if corlor_list.len() == 3 {
          color_txt += "rgb("
        } else {
          color_txt += "rgba("
        }
        for (index, ident) in corlor_list.iter().enumerate() {
          if let IdentType::Number(val, unit) = ident {
            if index != corlor_list.len() - 1 {
              color_txt += format!("{}, ", val).as_str();
            } else if *unit == Some('%'.to_string()) {
              let num = val.parse::<f64>().unwrap() / 100_f64;
              color_txt += format!("{:.1}", num).as_str();
            } else {
              color_txt += val;
            }
          } else {
            return Err(format!(
              "{:#?} must be num in the list ->{:#?}",
              ident, list
            ));
          }
        }
        color_txt += ")";

        list.insert(start, IdentType::Word(color_txt));
        extra += 1;
        end += extra;
        rm_vec.push((start + extra, end));
      }
    }

    let mut rm_count = 0;
    for (rs, re) in rm_vec {
      let start = rs - rm_count;
      let mut end = re - rm_count;
      while end > start - 1 {
        list.remove(end);
        end -= 1
      }
      rm_count += re - rs + 1;
    }

    Ok(())
  }

  fn scan_calc_expr_replace(list: &mut Vec<IdentType>) -> Result<(), String> {
    // 寻找可能的锚点
    let mut index = 0;
    let mut calc_vec = vec![];
    while index < list.len() {
      let current = Self::get_safe(index, list).unwrap();
      let next = Self::get_safe(index + 1, list);
      if *current == IdentType::Word("calc".to_string())
        && next == Some(&IdentType::Brackets('('.to_string()))
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
        if current == &IdentType::Brackets('('.to_string()) {
          level += 1;
        } else if current == &IdentType::Brackets(')'.to_string()) {
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
    Self::scan_rgb_expr_calc_replace(&mut no_var_list)?;
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
    let mut index = 0;

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
    while index < list.len() {
      // 比对词性
      let now = list.get(index).unwrap().clone();
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
              let calc_number = IdentType::calc_value(calc_list.clone())?;
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
            let calc_number = IdentType::calc_value(calc_list.clone())?;
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
                    let calc_number = IdentType::calc_value(calc_list.clone())?;
                    nature_list.push(calc_number);
                    calc_list.clear();
                  }
                  nature_list.push(IdentType::Brackets(br));
                }
              } else {
                if !calc_list.is_empty() {
                  let calc_number = IdentType::calc_value(calc_list.clone())?;
                  nature_list.push(calc_number);
                  calc_list.clear();
                }
                nature_list.push(IdentType::Brackets(br));
              }
            }
          } else {
            if !calc_list.is_empty() {
              let calc_number = IdentType::calc_value(calc_list.clone())?;
              nature_list.push(calc_number);
              calc_list.clear();
            }
            nature_list.push(IdentType::Brackets(br));
          }
        }
      }
      index += 1;
    }
    if !calc_list.is_empty() {
      let calc_number = IdentType::calc_value(calc_list.clone())?;
      nature_list.push(calc_number);
      calc_list.clear();
    }

    let mut res: Vec<String> = vec![];
    for (index, item) in nature_list.iter().enumerate() {
      let last = if index > 0 {
        Some(nature_list.get(index - 1).unwrap().clone())
      } else {
        None
      };

      match item {
        IdentType::Number(value, unit) => {
          let add_char =
            "".to_string() + value + unit.clone().unwrap_or_else(|| "".to_string()).as_str();
          if matches!(last, Some(IdentType::Word(..)))
            || matches!(last, Some(IdentType::Number(..)))
          {
            res.push(" ".to_string());
          }
          res.push(add_char);
        }
        IdentType::Word(char) => {
          if matches!(last, Some(IdentType::Word(..)))
            || matches!(last, Some(IdentType::Number(..)))
          {
            res.push(" ".to_string());
          }
          res.push(char.to_string());
        }
        IdentType::Space => {
          if !matches!(last, Some(IdentType::Space)) {
            res.push(" ".to_string());
          }
        }
        IdentType::Brackets(br) => {
          // todo fix single number situation
          res.push(br.to_string());
        }
        _ => {}
      }
    }

    Ok(res.join(""))
  }
}
