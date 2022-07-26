use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum StyleFileNode {
  Less(crate::less::filenode::FileNode),
}

impl StyleFileNode {
  pub fn to_less(self) -> Option<crate::less::filenode::FileNode> {
    let StyleFileNode::Less(node) = self;
    Some(node)
  }
}
