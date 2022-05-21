use crate::extend::vec_str::VecCharExtend;

pub type CharWord<'a> = (Option<&'a char>, &'a char, Option<&'a char>);
pub type ScanArg<'a> = (&'a mut usize, &'a mut Vec<char>, &'a mut bool);

///
/// 遍历
///
#[inline]
pub fn traversal(
  arg_start: Option<usize>,
  charlist: &[char],
  exec: &mut dyn for<'a> FnMut(ScanArg<'a>, CharWord) -> Result<(), String>,
) -> Result<(String, usize), String> {
  let mut index = arg_start.unwrap_or(0);
  let mut temp: Vec<char> = vec![];
  let mut hasend = false;

  while index < charlist.len() {
    let prevchar = if index == 0 {
      None
    } else {
      charlist.get(index - 1)
    };
    let char = charlist.get(index).unwrap();
    let nextchar = if index + 1 < charlist.len() {
      charlist.get(index + 1)
    } else {
      None
    };
    exec(
      (&mut index, &mut temp, &mut hasend),
      (prevchar, char, nextchar),
    )?;
    if hasend {
      break;
    }
    index += 1;
  }
  let final_str = temp.poly();
  Ok((final_str, index))
}
