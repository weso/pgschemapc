use crate::record::Key;
use crate::record::Record;
use crate::record::Value;
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
                for value in value_set {
                    if !value_type.conforms(value) {
                        return false;
                    }
                }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    StringType,
    IntegerType,
    DateType,
    Intersection(Box<ValueType>, Box<ValueType>),
    Union(Box<ValueType>, Box<ValueType>),
    Cond(BooleanExpr),
}

impl ValueType {
    pub fn integer() -> Self {
        ValueType::IntegerType
    }

    pub fn string() -> Self {
        ValueType::StringType
    }

    pub fn date() -> Self {
        ValueType::DateType
    }
    pub fn intersection(a: ValueType, b: ValueType) -> Self {
        ValueType::Intersection(Box::new(a), Box::new(b))
    }
    pub fn union(a: ValueType, b: ValueType) -> Self {
        ValueType::Union(Box::new(a), Box::new(b))
    }
    pub fn cond(expr: BooleanExpr) -> Self {
        ValueType::Cond(expr)
    }
    pub fn conforms(&self, value: &Value) -> bool {
        match self {
            ValueType::StringType => value.is_string(),
            ValueType::IntegerType => value.is_integer(),
            ValueType::DateType => value.is_date(),
            ValueType::Intersection(a, b) => a.conforms(value) && b.conforms(value),
            ValueType::Union(a, b) => a.conforms(value) || b.conforms(value),
            ValueType::Cond(_) => true, // Conditions are not checked here
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::StringType => write!(f, "String"),
            ValueType::IntegerType => write!(f, "Integer"),
            ValueType::DateType => write!(f, "Date"),
            ValueType::Intersection(a, b) => write!(f, "({} ∩ {})", a, b),
            ValueType::Union(a, b) => write!(f, "({} ∪ {})", a, b),
            ValueType::Cond(expr) => write!(f, "Condition({})", expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BooleanExpr {
    And(Box<BooleanExpr>, Box<BooleanExpr>),
    Or(Box<BooleanExpr>, Box<BooleanExpr>),
    Not(Box<BooleanExpr>),
    Equals(String),      // Example for equality check
    GreaterThan(String), // Example for equality check
}

impl Display for BooleanExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BooleanExpr::And(a, b) => write!(f, "({} AND {})", a, b),
            BooleanExpr::Or(a, b) => write!(f, "({} OR {})", a, b),
            BooleanExpr::Not(expr) => write!(f, "NOT ({})", expr),
            BooleanExpr::Equals(value) => write!(f, "(== {})", value),
            BooleanExpr::GreaterThan(value) => write!(f, "(> {})", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_type_conforms() {
        let mut record_type = RecordType::new();
        record_type
            .map
            .insert("name".to_string(), ValueType::string());
        record_type
            .map
            .insert("age".to_string(), ValueType::integer());

        let mut record = Record::new();
        record.insert("name".to_string(), Value::str("Alice"));
        record.insert("age".to_string(), Value::int(42));

        assert!(record_type.conforms(&record, true));
    }

    #[test]
    fn test_record_type_not_conforms() {
        let mut record_type = RecordType::new();
        record_type
            .map
            .insert("name".to_string(), ValueType::string());

        let mut record = Record::new();
        record.insert("name".to_string(), Value::int(42));

        assert!(!record_type.conforms(&record, false));
    }
}
