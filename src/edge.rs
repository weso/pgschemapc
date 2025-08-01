use std::collections::HashSet;

use crate::{LabelName, node_id::NodeId, record::Record};

pub struct Edge {
    pub id: NodeId,
    pub source: NodeId,
    pub target: NodeId,
    pub label: HashSet<LabelName>,
    pub properties: Record,
}
