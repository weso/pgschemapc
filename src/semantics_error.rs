use thiserror::Error;

use crate::{TypeName, card::Card, key::Key, value::Value};

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
}
