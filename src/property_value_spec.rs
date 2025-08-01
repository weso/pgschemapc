use crate::formal_base_type::FormalBaseType;
use crate::record::Key;
use crate::record::Record;
use crate::record::Value;
use crate::record_type::RecordType;
use crate::record_type::ValueType;
use crate::semantics_error::SemanticsError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValueSpec {
    Closed(PropertyValue),
    Open(PropertyValue),
}

impl PropertyValueSpec {
    pub fn semantics(&self) -> Result<FormalBaseType, SemanticsError> {
        todo!()
        /*match self {
            PropertyValueSpec::Closed(pv) => pv.conforms(record, false),
            PropertyValueSpec::Open(pv) => pv.conforms(record, true),
        }*/
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValue {
    EachOf(Box<PropertyValue>, Box<PropertyValue>),
    OneOf(Box<PropertyValue>, Box<PropertyValue>),
    Property(Key, TypeSpec),
    OptionalProperty(Key, TypeSpec),
    Empty,
}

impl PropertyValue {
    pub fn semantics(&self) -> HashSet<RecordType> {
        let mut semantics = HashSet::new();
        match self {
            PropertyValue::EachOf(left, right) => {
                let left_semantics = left.semantics();
                let right_semantics = right.semantics();
                // left_semantics.combine(&right_semantics)
                todo!()
            }
            PropertyValue::OneOf(left, right) => {
                let left_semantics = left.semantics();
                let right_semantics = right.semantics();
                // left_semantics.combine(&right_semantics)
                todo!()
            }
            PropertyValue::Property(_, type_spec) => {
                // semantics.insert(type_spec.to_record_type());
                todo!()
            }
            PropertyValue::OptionalProperty(_, type_spec) => {
                // semantics.insert(type_spec.to_record_type());
                todo!()
            }
            PropertyValue::Empty => {}
        }
        semantics
    }
}

impl PropertyValueSpec {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSpec {
    type_def: Type,
    card: Option<Card>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Conjunction(Box<Type>, Box<Type>),
    Disjunction(Box<Type>, Box<Type>),
    Type(ValueType),
    Cond(ValueType, Cond),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Card {
    ZeroOrOne,
    One,
    ZeroOrMore,
    OneOrMore,
    Range(u32, Max),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cond {
    True,
    False,
    Not(Box<Cond>),
    And(Box<Cond>, Box<Cond>),
    Or(Box<Cond>, Box<Cond>),
    GreaterThan(Value),
    Equals(Value),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Max {
    Unbounded,
    Bounded(u32),
}
