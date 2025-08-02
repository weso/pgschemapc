use either::Either;

use crate::evidence::Evidence;
use crate::key::Key;
use crate::record::Record;
use crate::semantics_error::SemanticsError;
use crate::value_type::ValueType;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
// use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordType {
    map: BTreeMap<Key, ValueType>,
}

impl RecordType {
    pub fn new() -> Self {
        RecordType {
            map: BTreeMap::new(),
        }
    }

    /// Creates an empty RecordType.
    pub fn empty() -> Self {
        RecordType::new()
    }

    /// Creates a RecordType with a single key-value pair.
    pub fn with_key_value(mut self, key: &str, value_type: ValueType) -> Self {
        self.map.insert(Key::new(key), value_type);
        self
    }

    /// Checks if the RecordType conforms to the given Record.
    pub fn conforms(
        &self,
        record: &Record,
        open: bool,
    ) -> Either<Vec<SemanticsError>, Vec<Evidence>> {
        for (key, value_set) in record.iter() {
            if let Some(value_type) = self.map.get(key) {
                let result = value_type.conforms(value_set);
                if result.is_left() {
                    return result;
                } else {
                    // If the value type conforms, we can collect evidence
                    // let evidence = result.right().unwrap();
                    continue;
                }
            } else {
                if !open {
                    return Either::Left(vec![SemanticsError::KeyNotFoundClosedRecordType {
                        key: key.clone(),
                        record_type: self.to_string(),
                    }]); // Key not found in RecordType and open is false
                } else {
                    // If open, we can ignore keys not in RecordType
                    continue;
                }
            }
        }
        Either::Right(vec![])
    }

    pub fn combine(&self, other: &RecordType) -> Self {
        let mut result = RecordType::new();
        for (key, value_type) in &self.map {
            if let Some(other_value_type) = other.map.get(key) {
                let combined_type =
                    ValueType::intersection(value_type.clone(), other_value_type.clone());
                result.map.insert(key.clone(), combined_type);
            } else {
                result.map.insert(key.clone(), value_type.clone());
            }
        }
        for (key, value_type) in &other.map {
            if !self.map.contains_key(key) {
                result.map.insert(key.clone(), value_type.clone());
            }
        }
        result
    }

    /// Inserts a key-value pair into the RecordType.
    pub fn insert(&mut self, key: Key, value_type: ValueType) {
        self.map.insert(key, value_type);
    }
}

impl Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut entries: Vec<String> = self
            .map
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        entries.sort();
        write!(f, "{{{}}}", entries.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Value, card::Card, value_type::ValueType};

    use super::*;

    #[test]
    fn test_record_type_conforms() {
        let mut record_type = RecordType::new();
        record_type
            .map
            .insert(Key::new("name"), ValueType::string(Card::One));
        record_type
            .map
            .insert(Key::new("age"), ValueType::integer(Card::One));

        let mut record = Record::new();
        record.insert(Key::new("name"), Value::str("Alice"));
        record.insert(Key::new("age"), Value::int(42));

        assert!(record_type.conforms(&record, true).is_right());
    }

    #[test]
    fn test_record_type_not_conforms() {
        let record_type = RecordType::new().with_key_value("name", ValueType::string(Card::One));
        let record = Record::new().with_key_value("name", Value::int(42));
        assert!(record_type.conforms(&record, false).is_left());
    }

    #[test]
    fn test_record_type_name_age_not_conforms() {
        let record_type = RecordType::new()
            .with_key_value("name", ValueType::string(Card::One))
            .with_key_value("age", ValueType::integer(Card::One));

        let record = Record::new()
            .with_key_value("name", Value::str("Alice"))
            .with_key_value("age", Value::str("Other"));
        assert!(record_type.conforms(&record, false).is_left());
    }
}
