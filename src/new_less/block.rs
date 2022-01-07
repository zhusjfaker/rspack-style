use crate::new_less::loc::Loc;

#[derive(Debug, Clone)]
pub enum OriginBlockType {
  Comment,
  Style_Rule,
  Var,
  Import,
}


#[derive(Debug, Clone)]
pub struct OriginBlock {
  pub block_type: OriginBlockType,
  pub content: String,
  pub loc: Loc,
}