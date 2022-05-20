pub trait VecCharExtend {
  fn poly(&self) -> String;
  fn trim(&self) -> Self;
  fn trim_start(&self) -> Self;
}

pub trait VecCharOptionalExtend {
  fn try_getword(&self, index: usize, wordlength: usize) -> Result<String, String>;
}

impl VecCharOptionalExtend for Vec<char> {
  fn try_getword(&self, index: usize, wordlength: usize) -> Result<String, String> {
    if index < self.len() {
      let start = index;
      let mut end = index + wordlength;
      if end > self.len() {
        end = self.len();
      }
      Ok(self[start..end].to_vec().iter().collect::<String>())
    } else {
      Err("find index is over vec range!".to_string())
    }
  }
}

impl VecCharOptionalExtend for &[char] {
  fn try_getword(&self, index: usize, wordlength: usize) -> Result<String, String> {
    if index < self.len() {
      let start = index;
      let mut end = index + wordlength;
      if end > self.len() {
        end = self.len();
      }
      Ok(self[start..end].to_vec().iter().collect::<String>())
    } else {
      Err("find index is over vec range!".to_string())
    }
  }
}

impl VecCharExtend for Vec<char> {
  fn poly(&self) -> String {
    self.iter().collect::<String>()
  }

  fn trim(&self) -> Vec<char> {
    let mut start = 0;
    let mut end = self.len();
    for (index, val) in self.iter().enumerate() {
      if *val != ' ' && *val != '\r' && *val != '\n' {
        start = index;
        break;
      }
    }
    for (index, val) in self.iter().rev().enumerate() {
      if *val != ' ' && *val != '\r' && *val != '\n' {
        end = self.len() - (index);
        break;
      }
    }
    self[start..end].to_vec()
  }

  fn trim_start(&self) -> Vec<char> {
    let mut start = 0;
    let end = self.len();
    for (index, val) in self.iter().enumerate() {
      if *val != ' ' && *val != '\r' && *val != '\n' {
        start = index;
        break;
      }
    }
    self[start..end].to_vec()
  }
}
