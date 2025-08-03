use std::{collections::HashSet, fmt::Display};

use either::Either;

use crate::{
    boolean_expr::BooleanExpr, card::Card, evidence::Evidence, semantics_error::SemanticsError,
    value::Value,
};

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
    pub fn integer(card: Card) -> Self {
        ValueType::IntegerType(card)
    }

    pub fn string(card: Card) -> Self {
        ValueType::StringType(card)
    }

    pub fn date(card: Card) -> Self {
        ValueType::DateType(card)
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
    pub fn conforms(&self, values: &HashSet<Value>) -> Either<Vec<SemanticsError>, Vec<Evidence>> {
        match self {
            ValueType::StringType(card) => {
                if !card.contains(values.len()) {
                    return Either::Left(vec![SemanticsError::CardinalityMismatch {
                        expected: card.clone(),
                        count: values.len(),
                    }]);
                }
                check_all(values, |v| v.is_string(), "is_string")
            }
            ValueType::IntegerType(card) => {
                if !card.contains(values.len()) {
                    return Either::Left(vec![SemanticsError::CardinalityMismatch {
                        expected: card.clone(),
                        count: values.len(),
                    }]);
                }
                check_all(values, |v| v.is_integer(), "is_integer")
            }
            ValueType::DateType(card) => {
                if !card.contains(values.len()) {
                    return Either::Left(vec![SemanticsError::CardinalityMismatch {
                        expected: card.clone(),
                        count: values.len(),
                    }]);
                }
                check_all(values, |v| v.is_date(), "is_date")
            }
            ValueType::Intersection(a, b) => todo!(), // a.conforms(values) && b.conforms(values),
            ValueType::Union(a, b) => todo!(),        // a.conforms(values) || b.conforms(values),
            ValueType::Cond(cond) => todo!(), // values.iter().all(|v| cond.check(v)), // Conditions are not checked here
        }
    }
}

fn check_all<F>(
    values: &HashSet<Value>,
    predicate: F,
    predicate_name: &str,
) -> Either<Vec<SemanticsError>, Vec<Evidence>>
where
    F: Fn(&Value) -> bool,
{
    if values.iter().all(&predicate) {
        Either::Right(vec![]) // All values conform
    } else {
        Either::Left(vec![SemanticsError::PredicateFailed {
            predicate_name: predicate_name.to_string(),
            value: values
                .iter()
                .find(|v| !predicate(v))
                .cloned()
                .unwrap_or(Value::String("".to_string())),
        }])
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
