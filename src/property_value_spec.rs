use tracing::span::Record;

use crate::boolean_expr::BooleanExpr;
use crate::card::Card;
use crate::formal_base_type::FormalBaseType;
use crate::key::Key;
use crate::record_type::RecordType;

use crate::semantics_error::SemanticsError;
use crate::value_type::ValueType;
use std::collections::HashSet;

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

    pub fn optional_property(key: Key, type_spec: TypeSpec) -> Self {
        PropertyValue::OptionalProperty(key, type_spec)
    }

    pub fn each_of(left: PropertyValue, right: PropertyValue) -> Self {
        PropertyValue::EachOf(Box::new(left), Box::new(right))
    }

    pub fn semantics(&self) -> HashSet<RecordType> {
        match self {
            PropertyValue::EachOf(left, right) => {
                let left_semantics = left.semantics();
                let right_semantics = right.semantics();
                combine_semantics_sets(left_semantics, right_semantics)
            }
            PropertyValue::OneOf(left, right) => {
                let left_semantics = left.semantics();
                let right_semantics = right.semantics();
                //left_semantics.union(&right_semantics)
                union_semantics_sets(left_semantics, right_semantics)
            }
            PropertyValue::Property(p, type_spec) => {
                let record = RecordType::new().with_key_value(p.str(), type_spec.to_value_type());
                HashSet::from_iter(vec![record])
            }
            PropertyValue::OptionalProperty(p, type_spec) => {
                let record = RecordType::new().with_key_value(p.str(), type_spec.to_value_type());
                HashSet::from_iter(vec![record, RecordType::empty()])
            }
            PropertyValue::Empty => HashSet::new(),
        }
    }
}

fn combine_semantics_sets(
    left: HashSet<RecordType>,
    right: HashSet<RecordType>,
) -> HashSet<RecordType> {
    let mut combined = HashSet::new();
    for l in left {
        for r in &right {
            combined.insert(l.combine(r));
        }
    }
    combined
}

fn union_semantics_sets(
    left: HashSet<RecordType>,
    right: HashSet<RecordType>,
) -> HashSet<RecordType> {
    // TODO: Replace by union semantics
    let mut combined = HashSet::new();
    for l in left {
        for r in &right {
            combined.insert(l.combine(r));
        }
    }
    combined
}

impl PropertyValueSpec {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSpec {
    type_def: Type,
}

impl TypeSpec {
    pub fn string(card: Card) -> Self {
        TypeSpec {
            type_def: Type::Type(ValueType::string(card)),
        }
    }

    pub fn integer(card: Card) -> Self {
        TypeSpec {
            type_def: Type::Type(ValueType::integer(card)),
        }
    }

    pub fn date(card: Card) -> Self {
        TypeSpec {
            type_def: Type::Type(ValueType::date(card)),
        }
    }

    pub fn to_value_type(&self) -> ValueType {
        self.type_def.to_value_type()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Conjunction(Box<Type>, Box<Type>),
    Disjunction(Box<Type>, Box<Type>),
    Type(ValueType),
    Cond(ValueType, BooleanExpr),
}

impl Type {
    pub fn to_value_type(&self) -> ValueType {
        match self {
            Type::Type(value_type) => value_type.clone(),
            Type::Conjunction(a, b) => {
                let avt: ValueType = (*a).to_value_type();
                let bvt: ValueType = (*b).to_value_type();
                ValueType::intersection(avt, bvt)
            }
            Type::Disjunction(a, b) => ValueType::union(a.to_value_type(), b.to_value_type()),
            Type::Cond(value_type, cond) => {
                let vt_cond = ValueType::cond(cond.clone());
                ValueType::intersection(value_type.clone(), vt_cond)
            }
        }
    }
}
