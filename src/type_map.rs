use std::fmt::Display;

use crate::{
    formal_graph_type::FormalGraphType,
    pg::PropertyGraph,
    pgs_error::PgsError,
    validation_result::{ResultAssociation, ValidationResult},
};

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

    pub fn validate(
        &self,
        schema: &FormalGraphType,
        graph: &PropertyGraph,
    ) -> Result<ValidationResult, PgsError> {
        let mut result = ValidationResult::new();
        for association in &self.associations {
            let node_id = association.node_id();
            let type_name = association.type_name();
            let node =
                graph
                    .get_node_by_label(node_id)
                    .map_err(|_| PgsError::MissingNodeLabel {
                        label: node_id.to_string(),
                    })?;
            let conforms_result = schema.conforms_node(&type_name, node);
            result.add_association(ResultAssociation {
                node_id: node_id.clone(),
                type_name: type_name.clone(),
                conforms: conforms_result.is_right(),
                details: conforms_result,
            });
        }
        Ok(result) // Assuming validation passes for now
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
