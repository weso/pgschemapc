use std::collections::HashMap;

use crate::{
    label_property_spec::LabelPropertySpec, node::Node, record::Record,
    semantics_error::SemanticsError, type_name::TypeName,
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

    pub fn add(&mut self, type_name: TypeName, label_property_spec: LabelPropertySpec) {
        self.map.insert(type_name, label_property_spec);
    }

    pub fn get(&self, type_name: &str) -> Option<&LabelPropertySpec> {
        self.map.get(type_name)
    }

    pub fn conforms_node(&self, type_name: &TypeName, node: &Node) -> Result<bool, SemanticsError> {
        if let Some(label_property_spec) = self.get(type_name) {
            let base_type = label_property_spec.semantics(self)?;
            base_type.conforms(node.labels(), node.content())
        } else {
            Err(SemanticsError::MissingType(type_name.clone()))
        }
    }
}
