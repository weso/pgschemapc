use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{edge::Edge, node::Node, record::Record, type_name::LabelName};

/// Simple representation of a property graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyGraph {
    nodes: HashMap<usize, Node>,
    edges: HashMap<usize, Edge>,
    label_names: HashMap<String, usize>,
    node_id_counter: usize,
    edge_id_counter: usize,
}

impl PropertyGraph {
    /// Creates a new empty PropertyGraph.
    pub fn new() -> Self {
        PropertyGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            label_names: HashMap::new(),
            node_id_counter: 0,
            edge_id_counter: 0,
        }
    }

    pub fn with_nodes(mut self, nodes: HashMap<usize, Node>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn with_edges(mut self, edges: HashMap<usize, Edge>) -> Self {
        self.edges = edges;
        self
    }

    /// Adds a node to the PropertyGraph.
    pub fn add_node(&mut self, name_id: String, labels: HashSet<LabelName>, record: Record) {
        let id = self.node_id_counter;
        self.node_id_counter += 1;
        self.label_names.insert(name_id, id);
        let node = Node::new(id as u32)
            .with_labels(labels)
            .with_content(&record);
        self.nodes.insert(id, node);
    }

    /// Adds an edge to the PropertyGraph.
    pub fn add_edge(&mut self, edge: Edge) {
        let id = self.edge_id_counter;
        self.edge_id_counter += 1;
        self.edges.insert(id, edge);
    }
}

impl Display for PropertyGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (node_id, node) in self.nodes.iter() {
            writeln!(f, "Node {}: {}", node_id, node)?;
        }
        for (edge_id, edge) in self.edges.iter() {
            writeln!(f, "Edge {}: {}", edge_id, edge)?;
        }
        Ok(())
    }
}
