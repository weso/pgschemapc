use super::pgs_actions::CreateType;
use rustemo::Parser;

use crate::{
    card::Card,
    formal_graph_type::FormalGraphType,
    key::Key,
    label_property_spec::LabelPropertySpec as PGLabelPropertySpec,
    parser::{
        pgs::PgsParser,
        pgs_actions::{
            LabelPropertySpec, LabelSpec, Properties, Property, PropertySpec, SimpleType, TypeSpec,
        },
    },
    pgs_error::PgsError,
    property_value_spec::{
        PropertyValue as PGPropertyValue, PropertyValueSpec as PGPropertyValueSpec,
        TypeSpec as PGTypeSpec,
    },
};

pub struct PgBuilder {}

impl PgBuilder {
    pub fn new() -> Self {
        PgBuilder {}
    }
    pub fn parse_pgs(&self, input: &str) -> Result<FormalGraphType, PgsError> {
        let mut result = FormalGraphType::new();
        let pgs_content = PgsParser::new()
            .parse(input)
            .map_err(|e| PgsError::ParserError {
                error: e.to_string(),
            })?;
        match pgs_content {
            CreateType::CreateNodeType(node_type) => {
                let type_name = node_type.type_name;
                let label_property_spec = get_label_property_spec(node_type.label_property_spec)?;
                result.add(type_name.as_str(), label_property_spec);
            }
            CreateType::CreateEdgeType(edge_type) => todo!(),
            CreateType::CreateGraphType(_) => todo!(),
        }
        Ok(result)
    }
}

fn get_label_property_spec(
    label_property_spec: LabelPropertySpec,
) -> Result<PGLabelPropertySpec, PgsError> {
    if let Some(label_spec) = label_property_spec.label_spec_opt {
        let label_spec = get_label_spec(label_spec)?;
        if let Some(property_spec) = label_property_spec.property_spec_opt {
            let property_value_spec = get_property_value_spec(property_spec)?;
            Ok(PGLabelPropertySpec::content(
                label_spec,
                PGPropertyValueSpec::closed(property_value_spec),
            ))
        } else {
            Ok(label_spec)
        }
    } else {
        Ok(PGLabelPropertySpec::new())
    }
}

fn get_label_spec(label_spec: LabelSpec) -> Result<PGLabelPropertySpec, PgsError> {
    Ok(PGLabelPropertySpec::Label(label_spec))
}

fn get_property_value_spec(property_value_spec: Properties) -> Result<PGPropertyValue, PgsError> {
    get_property_value(property_value_spec)
}

fn get_property_value(property_value: Properties) -> Result<PGPropertyValue, PgsError> {
    match property_value {
        Properties::Paren(properties) => get_property_value_spec(*properties),
        PropertySpec::EachOf(each_of) => {
            let left = get_property_value_spec(*each_of.left)?;
            let right = get_property_value_spec(*each_of.right)?;
            Ok(PGPropertyValue::each_of(left, right))
        }
        PropertySpec::OneOf(one_of) => {
            let left = get_property_value_spec(*one_of.left)?;
            let right = get_property_value_spec(*one_of.right)?;
            Ok(PGPropertyValue::one_of(left, right))
        }
        PropertySpec::Property(property) => get_property(property),
    }
}

fn get_property(property: Property) -> Result<PGPropertyValue, PgsError> {
    let key = property.key;
    let type_spec = get_type_spec(property.type_spec)?;
    Ok(PGPropertyValue::property(Key::new(key.as_str()), type_spec))
}

fn get_type_spec(type_spec: TypeSpec) -> Result<PGTypeSpec, PgsError> {
    match type_spec {
        super::pgs_actions::SimpleType::STRING_NAME => Ok(PGTypeSpec::string(Card::One)),
        super::pgs_actions::SimpleType::INTEGER_NAME => Ok(PGTypeSpec::integer(Card::One)),
        super::pgs_actions::SimpleType::DATE_NAME => Ok(PGTypeSpec::date(Card::One)),
    }
}
