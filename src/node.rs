use std::{collections::HashSet, fmt::Display};

use crate::{LabelName, node_id::NodeId, record::Record};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: NodeId,
    pub label: HashSet<LabelName>,
    pub properties: Record,
}

impl Node {
    pub fn new(id: u32) -> Self {
        Node {
            id: NodeId::new(id),
            label: HashSet::new(),
            properties: Record::new(),
        }
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label.insert(label.to_string());
        self
    }

    pub fn with_content(mut self, content: &Record) -> Self {
        self.properties = content.clone();
        self
    }

    pub fn labels(&self) -> &HashSet<LabelName> {
        &self.label
    }

    pub fn content(&self) -> &Record {
        &self.properties
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node({} {:?} [{:?}]",
            self.id, self.label, self.properties
        )
    }
}
