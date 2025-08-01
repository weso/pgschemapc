use crate::record::Key;
use crate::record::Record;
use crate::value_type::ValueType;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordType {
    map: HashMap<Key, ValueType>,
}
impl Hash for RecordType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the contents of the map
        for (k, v) in &self.map {
            k.hash(state);
            v.hash(state);
        }
    }
}
impl RecordType {
    pub fn new() -> Self {
        RecordType {
            map: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        RecordType {
            map: HashMap::new(),
        }
    }

    pub fn conforms(&self, record: &Record, open: bool) -> bool {
        for (key, value_set) in record.iter() {
            if let Some(value_type) = self.map.get(key) {
                return value_type.conforms(value_set);
            } else {
                if !open {
                    return false; // Key not found in RecordType and open is false
                } else {
                    // If open, we can ignore keys not in RecordType
                    continue;
                }
            }
        }
        true
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
    use crate::{record::Value, value_type::ValueType};

    use super::*;

    #[test]
    fn test_record_type_conforms() {
        let mut record_type = RecordType::new();
        record_type
            .map
            .insert(Key::new("name"), ValueType::string());
        record_type
            .map
            .insert(Key::new("age"), ValueType::integer());

        let mut record = Record::new();
        record.insert(Key::new("name"), Value::str("Alice"));
        record.insert(Key::new("age"), Value::int(42));

        assert!(record_type.conforms(&record, true));
    }

    #[test]
    fn test_record_type_not_conforms() {
        let mut record_type = RecordType::new();
        record_type
            .map
            .insert(Key::new("name"), ValueType::string());

        let mut record = Record::new();
        record.insert(Key::new("name"), Value::int(42));

        assert!(!record_type.conforms(&record, false));
    }
}
