use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::{
    Name, TypeName,
    formal_base_type::FormalBaseType,
    property_value_spec::{PropertyValue, PropertyValueSpec},
    record_type::RecordType,
    semantics_error::SemanticsError,
};

// In the PGSchema paper, LabelPropertySpec is denoted by F
pub enum LabelPropertySpec {
    Label(Name),
    Ref(TypeName),
    Optional(Box<LabelPropertySpec>),
    And(Box<LabelPropertySpec>, Box<LabelPropertySpec>),
    Or(Box<LabelPropertySpec>, Box<LabelPropertySpec>),
    Open(Box<LabelPropertySpec>),
    Content(Box<LabelPropertySpec>, PropertyValueSpec),
}

impl LabelPropertySpec {
    pub fn semantics(
        &self,
        graph_type: &FormalGraphType,
    ) -> Result<FormalBaseType, SemanticsError> {
        match self {
            LabelPropertySpec::Label(label) => Ok(FormalBaseType::from_label(label.clone())),
            LabelPropertySpec::Ref(type_name) => {
                if let Some(label_property_spec) = graph_type.get(type_name) {
                    label_property_spec.semantics(graph_type)
                } else {
                    Err(SemanticsError::MissingType(type_name.clone()))
                }
            }
            LabelPropertySpec::Optional(label_property_spec) => {
                let base_type = label_property_spec.semantics(graph_type)?;
                Ok(base_type.union(&FormalBaseType::type_0()))
            }
            LabelPropertySpec::And(label_property_spec, label_property_spec1) => {
                let base_type = label_property_spec.semantics(graph_type)?;
                let base_type1 = label_property_spec1.semantics(graph_type)?;
                Ok(base_type.combine(&base_type1))
            }
            LabelPropertySpec::Or(label_property_spec, label_property_spec1) => {
                let base_type = label_property_spec.semantics(graph_type)?;
                let base_type1 = label_property_spec1.semantics(graph_type)?;
                Ok(base_type.union(&base_type1))
            }
            LabelPropertySpec::Open(label_property_spec) => todo!(),
            LabelPropertySpec::Content(label_property_spec, property_value_spec) => {
                let base_type = label_property_spec.semantics(graph_type)?;
                let property_value_semantics = property_value_spec.semantics()?;
                let result = base_type.combine(&property_value_semantics);
                Ok(result)
            }
        }
    }
}

struct FormalGraphType {
    map: HashMap<TypeName, LabelPropertySpec>,
}

impl FormalGraphType {
    pub fn new() -> Self {
        FormalGraphType {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, type_name: TypeName, label_property_spec: LabelPropertySpec) {
        self.map.insert(type_name, label_property_spec);
    }

    pub fn get(&self, type_name: &TypeName) -> Option<&LabelPropertySpec> {
        self.map.get(type_name)
    }
}
