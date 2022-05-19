use crate::new_less::ident::IdentType;

///
/// less 计算 rgb 函数
///
pub fn rgb_calc(rgb: Vec<&IdentType>) -> Result<String, String> {
  if rgb.len() != 3 {
    return Err(
      "arg format is (IdentType,IdentType,IdentType),please check arg length!".to_string(),
    );
  }

  let mut value_list: Vec<usize> = vec![];
  for (index, ident) in rgb.into_iter().enumerate() {
    if let IdentType::Number(val, unit) = ident {
      if unit.is_some() {
        return Err(format!("arg index {} unit must be none!", index));
      }
      value_list.push(val.parse::<usize>().unwrap())
    } else {
      return Err(format!("arg index {} is not IdentType::Number", index));
    }
  }

  let inner_value = format!(
    "{:02X}{:02X}{:02X}",
    value_list[0] as f32 as u8, value_list[1] as f32 as u8, value_list[2] as f32 as u8
  );

  Ok(format!("#{}", inner_value.to_lowercase()))
}
