use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeId {
    pub id: u32,
}

impl NodeId {
    pub fn new(id: u32) -> Self {
        NodeId { id }
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.id)
    }
}
