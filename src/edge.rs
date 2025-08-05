use std::{collections::HashSet, fmt::Display};

use crate::{node_id::NodeId, record::Record, type_name::LabelName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub id: NodeId,
    pub source: NodeId,
    pub target: NodeId,
    pub label: HashSet<LabelName>,
    pub properties: Record,
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Edge({} from {} to {} with labels {:?} and properties {:?})",
            self.id, self.source, self.target, self.label, self.properties
        )
    }
}
