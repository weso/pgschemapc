use std::fmt::Display;

use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BooleanExpr {
    And(Box<BooleanExpr>, Box<BooleanExpr>),
    Or(Box<BooleanExpr>, Box<BooleanExpr>),
    Not(Box<BooleanExpr>),
    Equals(String),
    GreaterThan(String),
}

impl BooleanExpr {
    pub fn check(&self, value: &Value) -> bool {
        match self {
            BooleanExpr::And(a, b) => a.check(value) && b.check(value),
            BooleanExpr::Or(a, b) => a.check(value) || b.check(value),
            BooleanExpr::Not(expr) => !expr.check(value),
            BooleanExpr::Equals(v) => match value {
                Value::String(s) => s == v,
                _ => false,
            },
            BooleanExpr::GreaterThan(v) => match value {
                Value::Integer(i) => i.to_string() > *v,
                _ => false,
            },
        }
    }
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
