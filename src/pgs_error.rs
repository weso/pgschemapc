use thiserror::Error;

use crate::{card::Card, key::Key, type_name::TypeName, value::Value};

#[derive(Error, Debug, PartialEq, Clone)]
pub enum PgsError {
    #[error("Error parsing as number: {0}")]
    InvalidNumber(String),

    #[error("Not found type: {0}")]
    MissingType(TypeName),

    #[error("Not found node with label: {label}")]
    MissingLabel { label: String },

    #[error("Key not found in RecordType: {key} in Closed record type {record_type}")]
    KeyNotFoundClosedRecordType { key: Key, record_type: String },

    #[error("Cardinality doesn't match: {expected}, count {count}")]
    CardinalityMismatch { expected: Card, count: usize },

    #[error("Predicate {predicate_name} failed with value {value}")]
    PredicateFailed {
        predicate_name: String,
        value: Value,
    },

    #[error("Missing keys in record type: {record_type}, keys: {keys:?}")]
    MissingKeys { keys: String, record_type: String },

    #[error("Extra keys in closed record type: {record_type}, keys: {keys:?}")]
    ExtraKeysNotOpen { keys: String, record_type: String },

    #[error("Parser error parsing property graph schema: {error}")]
    ParserError { error: String },

    #[error("Parser error parsing property graph: {error}")]
    PGParserError { error: String },

    #[error("Labels do not match: record labels {record_labels}, type labels {type_labels}")]
    LabelsDifferent {
        record_labels: String,
        type_labels: String,
    },

    #[error(
        "Record does not conform to type content:\n Record:\n{record}\n Types:\n{type_content}"
    )]
    RecordContentFails {
        record: String,
        type_content: String,
    },

    #[error("Type mismatch in operation {operation}: expected {expected}, found {found}")]
    TypeMismatch {
        operation: String,
        expected: String,
        found: String,
    },

    #[error("Condition ({condition}) for value: {value}")]
    ConditionFailed { condition: String, value: String },
}
