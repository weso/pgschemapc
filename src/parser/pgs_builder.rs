use super::pgs_actions::CreateType;
use rustemo::Parser;

use crate::{
    card::{Card as PGCard, Max as PGMax},
    formal_graph_type::FormalGraphType,
    key::Key,
    label_property_spec::LabelPropertySpec as PGLabelPropertySpec,
    parser::{
        pgs::PgsParser,
        pgs_actions::{
            BaseProperty, Card, LabelPropertySpec, LabelSpec, Labels, Max, MoreLabels, Properties,
            Property, PropertySpec, Range, SingleLabel, TypeSpec,
        },
    },
    pgs_error::PgsError,
    property_value_spec::{
        PropertyValue as PGPropertyValue, PropertyValueSpec as PGPropertyValueSpec,
        TypeSpec as PGTypeSpec,
    },
};

pub struct PgsBuilder {}

impl PgsBuilder {
    pub fn new() -> Self {
        PgsBuilder {}
    }
    pub fn parse_pgs(&self, input: &str) -> Result<FormalGraphType, PgsError> {
        let pgs_content = PgsParser::new()
            .parse(input)
            .map_err(|e| PgsError::ParserError {
                error: e.to_string(),
            })?;
        let mut schema = FormalGraphType::new();
        get_create_types(pgs_content, &mut schema)?;
        Ok(schema)
    }
}

fn get_create_types(
    create_types: Vec<CreateType>,
    schema: &mut FormalGraphType,
) -> Result<(), PgsError> {
    for create_type in create_types {
        match create_type {
            CreateType::CreateNodeType(node_type) => {
                let type_name = node_type.type_name;
                let label_property_spec = get_label_property_spec(node_type.label_property_spec)?;
                schema.add(type_name.as_str(), label_property_spec);
            }
            CreateType::CreateEdgeType(_edge_type) => todo!(),
            CreateType::CreateGraphType(_) => todo!(),
        }
    }
    Ok(())
}

fn get_label_property_spec(
    label_property_spec: LabelPropertySpec,
) -> Result<PGLabelPropertySpec, PgsError> {
    if let Some(label_spec) = label_property_spec.label_spec_opt {
        let label_spec = get_labels(label_spec)?;
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

fn get_labels(labels: Labels) -> Result<PGLabelPropertySpec, PgsError> {
    let mut label_spec = get_single_label(labels.single_label)?;
    if let Some(more_labels) = labels.more_labels_opt {
        get_more_labels(more_labels, &mut label_spec)?;
    }
    Ok(label_spec)
}

fn get_single_label(single_label: SingleLabel) -> Result<PGLabelPropertySpec, PgsError> {
    match single_label {
        SingleLabel::SingleLabel(identifier) => Ok(PGLabelPropertySpec::label(identifier)),
        SingleLabel::TypeName(type_name) => Ok(PGLabelPropertySpec::ref_(type_name)),
    }
}

fn get_more_labels(
    more_labels: MoreLabels,
    label_spec: &mut PGLabelPropertySpec,
) -> Result<(), PgsError> {
    match more_labels {
        MoreLabels::AndLabels(and_labels) => {
            let label_spec2 = get_single_label(and_labels.single_label)?;
            *label_spec = PGLabelPropertySpec::and(label_spec.clone(), label_spec2);
            if let Some(more_labels) = *and_labels.more_labels_opt {
                get_more_labels(more_labels, label_spec)?;
                Ok(())
            } else {
                Ok(())
            }
        }
        MoreLabels::OrLabels(or_labels) => {
            let label_spec2 = get_single_label(or_labels.single_label)?;
            *label_spec = PGLabelPropertySpec::or(label_spec.clone(), label_spec2);
            if let Some(more_labels) = *or_labels.more_labels_opt {
                get_more_labels(more_labels, label_spec)?;
                Ok(())
            } else {
                Ok(())
            }
        }
    }
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
        PropertySpec::BaseProperty(property) => get_base_property(property),
    }
}

fn get_base_property(base_property: BaseProperty) -> Result<PGPropertyValue, PgsError> {
    let (key, type_spec) = get_property(base_property.property)?;
    if let Some(_) = base_property.optionalopt {
        Ok(PGPropertyValue::optional_property(key, type_spec))
    } else {
        Ok(PGPropertyValue::property(key, type_spec))
    }
}

fn get_property(property: Property) -> Result<(Key, PGTypeSpec), PgsError> {
    let key = property.key;
    let card = if let Some(card) = property.card_opt {
        get_card(card)?
    } else {
        PGCard::One
    };
    let type_spec = get_type_spec(property.type_spec, card)?;
    Ok((Key::new(key.as_str()), type_spec))
}

fn get_card(card: Card) -> Result<PGCard, PgsError> {
    match card {
        Card::ZeroOrMore => Ok(PGCard::ZeroOrMore),
        Card::OneOrMore => Ok(PGCard::OneOrMore),
        Card::Range(range) => get_range(range),
        Card::Optional => Ok(PGCard::ZeroOrOne),
    }
}

fn get_range(range: Range) -> Result<PGCard, PgsError> {
    let min = get_number(range.number)?;
    let max = get_max(range.max)?;
    Ok(PGCard::range(min, max))
}

fn get_max(max: Max) -> Result<PGMax, PgsError> {
    match max {
        Max::Number(n) => {
            let n = get_number(n)?;
            Ok(PGMax::Bounded(n))
        }
        Max::Star => Ok(PGMax::Unbounded),
    }
}

fn get_number(number: String) -> Result<usize, PgsError> {
    number
        .parse::<usize>()
        .map_err(|_| PgsError::InvalidNumber(number))
}

fn get_type_spec(type_spec: TypeSpec, card: PGCard) -> Result<PGTypeSpec, PgsError> {
    match type_spec {
        super::pgs_actions::SimpleType::STRING_NAME => Ok(PGTypeSpec::string(card)),
        super::pgs_actions::SimpleType::INTEGER_NAME => Ok(PGTypeSpec::integer(card)),
        super::pgs_actions::SimpleType::DATE_NAME => Ok(PGTypeSpec::date(card)),
    }
}
