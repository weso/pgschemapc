use crate::card::Card;
use crate::formal_base_type::FormalBaseType;
use crate::record::Key;

use crate::record::Value;
use crate::record_type::RecordType;

use crate::semantics_error::SemanticsError;
use crate::value_type::ValueType;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValueSpec {
    Closed(PropertyValue),
    Open(PropertyValue),
}

impl PropertyValueSpec {
    pub fn open(property_value: PropertyValue) -> Self {
        PropertyValueSpec::Open(property_value)
    }

    pub fn closed(property_value: PropertyValue) -> Self {
        PropertyValueSpec::Closed(property_value)
    }

    pub fn semantics(&self) -> Result<FormalBaseType, SemanticsError> {
        let content_semantics = match self {
            PropertyValueSpec::Closed(pv) => pv.semantics(),
            PropertyValueSpec::Open(pv) => pv.semantics(),
        };
        Ok(FormalBaseType::new().with_content(content_semantics))
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
    pub fn property(key: Key, type_spec: TypeSpec) -> Self {
        PropertyValue::Property(key, type_spec)
    }
    pub fn each_of(left: PropertyValue, right: PropertyValue) -> Self {
        PropertyValue::EachOf(Box::new(left), Box::new(right))
    }

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

impl TypeSpec {
    pub fn string() -> Self {
        TypeSpec {
            type_def: Type::Type(ValueType::string()),
            card: None,
        }
    }

    pub fn integer() -> Self {
        TypeSpec {
            type_def: Type::Type(ValueType::integer()),
            card: None,
        }
    }

    pub fn date() -> Self {
        TypeSpec {
            type_def: Type::Type(ValueType::date()),
            card: None,
        }
    }

    pub fn zero_or_more(self) -> Self {
        TypeSpec {
            type_def: self.type_def,
            card: Some(Card::ZeroOrMore),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Conjunction(Box<Type>, Box<Type>),
    Disjunction(Box<Type>, Box<Type>),
    Type(ValueType),
    Cond(ValueType, Cond),
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
