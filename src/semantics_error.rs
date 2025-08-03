use std::collections::HashSet;

use thiserror::Error;

use crate::{card::Card, key::Key, type_name::TypeName, value::Value};

#[derive(Error, Debug, PartialEq)]
pub enum SemanticsError {
    #[error("Not found type: {0}")]
    MissingType(TypeName),

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
}
