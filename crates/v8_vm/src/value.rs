//! Value type for the V8-Rust VM

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Object(usize), // handle para o heap
    Array(usize),  // handle para o heap
    Function(usize), // handle para o heap
    Null,
    Undefined,
}

impl Value {
    pub fn is_primitive(&self) -> bool {
        matches!(self, Value::Number(_) | Value::String(_) | Value::Boolean(_) | Value::Null | Value::Undefined)
    }
    pub fn as_number(&self) -> Option<f64> {
        if let Value::Number(n) = self { Some(*n) } else { None }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Boolean(b) = self { Some(*b) } else { None }
    }
    pub fn as_string(&self) -> Option<&str> {
        if let Value::String(s) = self { Some(s) } else { None }
    }
    pub fn to_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Boolean(b) => if *b { 1.0 } else { 0.0 },
            Value::String(s) => s.parse::<f64>().unwrap_or(f64::NAN),
            Value::Null => 0.0,
            Value::Undefined => f64::NAN,
            _ => f64::NAN,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Null => "null".to_string(),
            Value::Undefined => "undefined".to_string(),
            Value::Object(_) => "[object Object]".to_string(),
            Value::Array(_) => "[object Array]".to_string(),
            Value::Function(_) => "[function]".to_string(),
        }
    }
    pub fn to_boolean(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Null | Value::Undefined => false,
            _ => true,
        }
    }
} 