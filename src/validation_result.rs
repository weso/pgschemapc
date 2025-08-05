use std::fmt::Display;

use either::Either;

use crate::{evidence::Evidence, pgs_error::PgsError};

#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub associations: Vec<ResultAssociation>,
}

impl ValidationResult {
    pub fn new() -> Self {
        ValidationResult {
            is_valid: true,
            associations: Vec::new(),
        }
    }

    pub fn add_association(&mut self, association: ResultAssociation) {
        self.associations.push(association);
    }

    pub fn is_empty(&self) -> bool {
        self.associations.is_empty()
    }
}

#[derive(Debug)]
pub struct ResultAssociation {
    pub node_id: String,
    pub type_name: String,
    pub conforms: bool,
    pub details: Either<Vec<PgsError>, Vec<Evidence>>,
}

impl Display for ValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ValidationResult(is_valid: {}, associations: {:?})",
            self.is_valid, self.associations
        )
    }
}
impl Display for ResultAssociation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ResultAssociation(node_id: {}, type_name: {}, conforms: {}, details: {:?})",
            self.node_id, self.type_name, self.conforms, self.details
        )
    }
}
