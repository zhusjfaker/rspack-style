use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum StyleFileNode {
  Less(crate::less::filenode::FileNode),
  Css(crate::css::filenode::FileNode),
}

impl StyleFileNode {
  pub fn to_less(self) -> Option<crate::less::filenode::FileNode> {
    if let StyleFileNode::Less(node) = self {
      Some(node)
    } else {
      None
    }
  }

  pub fn to_css(self) -> Option<crate::css::filenode::FileNode> {
    if let StyleFileNode::Css(node) = self {
      Some(node)
    } else {
      None
    }
  }
}
