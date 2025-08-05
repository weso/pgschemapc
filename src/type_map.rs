use std::fmt::Display;

/// Defines associations between node IDs and type names that can be used to trigger validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeMap {
    associations: Vec<Association>,
}

impl TypeMap {
    pub fn new() -> Self {
        TypeMap {
            associations: Vec::new(),
        }
    }

    pub fn add_association(&mut self, association: Association) {
        self.associations.push(association);
    }
}

impl Display for TypeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ass in &self.associations {
            writeln!(f, "  {}", ass)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Association {
    node_id: String,
    type_name: String,
}

impl Association {
    pub fn new(node_id: String, type_name: String) -> Self {
        Association { node_id, type_name }
    }

    pub fn node_id(&self) -> &String {
        &self.node_id
    }

    pub fn type_name(&self) -> &String {
        &self.type_name
    }
}

impl Display for Association {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{},", self.node_id, self.type_name)
    }
}
