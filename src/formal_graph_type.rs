use std::{collections::HashMap, fmt::Display};

use either::Either;

use crate::{
    edge::Edge, evidence::Evidence, label_property_spec::LabelPropertySpec, node::Node,
    pgs_error::PgsError, record::Record, type_name::TypeName,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalGraphType {
    map: HashMap<TypeName, LabelPropertySpec>,
}

impl FormalGraphType {
    pub fn new() -> Self {
        FormalGraphType {
            map: HashMap::new(),
        }
    }

    pub fn with_type_name(
        mut self,
        type_name: &str,
        label_property_spec: LabelPropertySpec,
    ) -> Self {
        self.map.insert(type_name.to_string(), label_property_spec);
        self
    }

    pub fn add(&mut self, type_name: &str, label_property_spec: LabelPropertySpec) {
        self.map.insert(type_name.to_string(), label_property_spec);
    }

    pub fn get(&self, type_name: &str) -> Option<&LabelPropertySpec> {
        self.map.get(type_name)
    }

    pub fn conforms_node(
        &self,
        type_name: &TypeName,
        node: &Node,
    ) -> Either<Vec<PgsError>, Vec<Evidence>> {
        if let Some(label_property_spec) = self.get(type_name) {
            match label_property_spec.semantics(self) {
                Ok(base_type) => base_type.conforms(node.labels(), node.content()),
                Err(e) => return Either::Left(vec![e]),
            }
        } else {
            Either::Left(vec![PgsError::MissingType(type_name.clone())])
        }
    }

    pub fn conforms_edge(
        &self,
        type_name: &TypeName,
        edge: &Edge,
    ) -> Either<Vec<PgsError>, Vec<Evidence>> {
        if let Some(label_property_spec) = self.get(type_name) {
            match label_property_spec.semantics(self) {
                Ok(base_type) => base_type.conforms(edge.labels(), edge.content()),
                Err(e) => return Either::Left(vec![e]),
            }
        } else {
            Either::Left(vec![PgsError::MissingType(type_name.clone())])
        }
    }
}

impl Display for FormalGraphType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.map.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
