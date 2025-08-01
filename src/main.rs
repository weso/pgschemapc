use crate::record::Value;

mod formal_base_type;
mod label_property_spec;
mod property_value_spec;
mod record;
mod record_type;
mod semantics_error;

pub type Name = String;
pub type TypeName = String;

// src/main.rs
fn main() {
    println!("Hello, world!");
    let mut record = record::Record::new();
    record.insert("key1".to_string(), Value::str("value1"));
    record.insert("key1".to_string(), Value::str("value2"));
    record.insert("key2".to_string(), Value::str("value1"));
    println!("Record: {}", record);
}
