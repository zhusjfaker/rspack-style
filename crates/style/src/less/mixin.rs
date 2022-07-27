use crate::less::select::NewSelector;

impl NewSelector {
  pub fn parse_selector_mixin(&mut self, key: &str, start: &usize) -> Result<(), String> {
    let list = self.paradigm_vec.last_mut().unwrap();
    list.remove(list.len() - 1);
    
    Ok(())
  }
}