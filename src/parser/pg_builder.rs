use std::collections::HashSet;

use rustemo::Parser;

use crate::{
    key::Key,
    parser::{
        pg::PgParser,
        pg_actions::{LabelPropertySpec, Node, Property, SingleValue, Values},
    },
    pg::PropertyGraph,
    pgs_error::PgsError,
    record::Record,
    type_name::LabelName,
    value::Value,
};

pub struct PgBuilder {}

impl PgBuilder {
    pub fn new() -> Self {
        PgBuilder {}
    }
    pub fn parse_pg(&self, input: &str) -> Result<PropertyGraph, PgsError> {
        let pg_content = PgParser::new()
            .parse(input)
            .map_err(|e| PgsError::PGParserError {
                error: e.to_string(),
            })?;
        let mut pg = PropertyGraph::new();
        get_nodes(pg_content, &mut pg)?;
        Ok(pg)
    }
}

fn get_nodes(nodes: Vec<Node>, pg: &mut PropertyGraph) -> Result<(), PgsError> {
    for node in nodes {
        get_node(node, pg)?;
    }
    Ok(())
}

fn get_node(node: Node, pg: &mut PropertyGraph) -> Result<(), PgsError> {
    let id = get_id(node.id)?;
    let (labels, record) = get_properties(node.label_property_spec)?;
    pg.add_node(id, labels, record);
    Ok(())
}

fn get_id(id: String) -> Result<String, PgsError> {
    Ok(id)
}

fn get_properties(
    label_property_spec: LabelPropertySpec,
) -> Result<(HashSet<LabelName>, Record), PgsError> {
    let labels = if let Some(label_spec) = label_property_spec.label_spec_opt {
        let labels = get_labels(label_spec)?;
        Ok(labels)
    } else {
        Ok(HashSet::new())
    }?;
    let record = if let Some(property_spec) = label_property_spec.property_spec_opt {
        let record = get_property_spec(property_spec)?;
        Ok(record)
    } else {
        Ok(Record::new())
    }?;
    Ok((labels, record))
}

fn get_labels(labels: Vec<String>) -> Result<HashSet<LabelName>, PgsError> {
    let mut result = HashSet::new();
    for label in labels {
        result.insert(label);
    }
    Ok(result)
}

fn get_property_spec(property_spec: Vec<Property>) -> Result<Record, PgsError> {
    let mut record = Record::new();
    for property in property_spec {
        let (key, values) = get_property(property)?;
        record.insert_values(Key::new(key.as_str()), values)
    }
    Ok(record)
}

fn get_property(property: Property) -> Result<(String, HashSet<Value>), PgsError> {
    let key = property.key;
    let values = get_values(property.values)?;
    Ok((key, values))
}

fn get_values(values: Values) -> Result<HashSet<Value>, PgsError> {
    let mut result = HashSet::new();
    match values {
        Values::SingleValue(value) => {
            let value = get_value(value)?;
            result.insert(value);
        }
        Values::ListValue(values) => {
            for value in values {
                let value = get_value(value)?;
                result.insert(value);
            }
        }
    }
    Ok(result)
}

fn get_value(value: SingleValue) -> Result<Value, PgsError> {
    match value {
        SingleValue::StringValue(s) => Ok(Value::str(s.as_str())),
        SingleValue::NumberValue(str_number_) => {
            let number = str_number_.parse::<i32>().map_err(|_| {
                PgsError::InvalidNumber(format!("Invalid number value: {}", str_number_))
            })?;
            Ok(Value::int(number))
        }
    }
}
