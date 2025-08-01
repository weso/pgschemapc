use std::{collections::HashSet, fmt::Display};

use crate::{boolean_expr::BooleanExpr, card::Card, record::Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    StringType(Card),
    IntegerType(Card),
    DateType(Card),
    Intersection(Box<ValueType>, Box<ValueType>),
    Union(Box<ValueType>, Box<ValueType>),
    Cond(BooleanExpr),
}

impl ValueType {
    pub fn integer() -> Self {
        ValueType::IntegerType(Card::One)
    }

    pub fn string() -> Self {
        ValueType::StringType(Card::One)
    }

    pub fn date() -> Self {
        ValueType::DateType(Card::One)
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
    pub fn conforms(&self, values: &HashSet<Value>) -> bool {
        match self {
            ValueType::StringType(card) => {
                card.contains(values.len()) && values.iter().all(|v| v.is_string())
            }
            ValueType::IntegerType(card) => {
                card.contains(values.len()) && values.iter().all(|v| v.is_integer())
            }
            ValueType::DateType(card) => {
                card.contains(values.len()) && values.iter().all(|v| v.is_date())
            }
            ValueType::Intersection(a, b) => a.conforms(values) && b.conforms(values),
            ValueType::Union(a, b) => a.conforms(values) || b.conforms(values),
            ValueType::Cond(cond) => values.iter().all(|v| cond.check(v)), // Conditions are not checked here
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::StringType(card) => write!(f, "String({})", card),
            ValueType::IntegerType(card) => write!(f, "Integer({})", card),
            ValueType::DateType(card) => write!(f, "Date({})", card),
            ValueType::Intersection(a, b) => write!(f, "({} ∩ {})", a, b),
            ValueType::Union(a, b) => write!(f, "({} ∪ {})", a, b),
            ValueType::Cond(expr) => write!(f, "Condition({})", expr),
        }
    }
}
