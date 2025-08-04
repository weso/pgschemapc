use std::collections::HashMap;

use crate::record::Record;

/// Simple representation of a property graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyGraph {
    nodes: HashMap<String, Record>,
    edges: HashMap<String, (Record, Record, Record)>,
}

impl PropertyGraph {
    /// Creates a new empty PropertyGraph.
    pub fn new() -> Self {
        PropertyGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: HashMap<String, Record>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn with_edges(mut self, edges: HashMap<String, (Record, Record, Record)>) -> Self {
        self.edges = edges;
        self
    }

    /// Adds a node to the PropertyGraph.
    pub fn add_node(&mut self, id: String, properties: Record) {
        self.nodes.insert(id, properties);
    }

    /// Adds an edge to the PropertyGraph.
    pub fn add_edge(&mut self, id: String, source: Record, content: Record, target: Record) {
        self.edges.insert(id, (source, content, target));
    }
}
