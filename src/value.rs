use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    String(String),
    Integer(i32),
    Date(String), // Simplified for this example
}

impl Value {
    pub fn str(s: &str) -> Self {
        Value::String(s.to_string())
    }

    pub fn int(i: i32) -> Self {
        Value::Integer(i)
    }

    pub fn date(d: &str) -> Self {
        Value::Date(d.to_string())
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    pub fn is_date(&self) -> bool {
        matches!(self, Value::Date(_))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Date(d) => write!(f, "{}", d),
        }
    }
}
