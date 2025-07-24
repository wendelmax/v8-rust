//! Value types for V8-Rust JavaScript engine
//! 
//! This module provides the core value system for JavaScript values.

use std::rc::Rc;
use std::cell::RefCell;

/// Represents a JavaScript value
#[derive(Debug, Clone)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    BigInt(String),
    Object(Rc<RefCell<Object>>),
    Function(Rc<RefCell<Function>>),
    Array(Vec<Value>),
    RegExp(String, String), // pattern, flags
}

impl Value {
    /// Check if the value is undefined
    pub fn is_undefined(&self) -> bool {
        matches!(self, Value::Undefined)
    }
    
    /// Check if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
    
    /// Check if the value is a boolean
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }
    
    /// Check if the value is a number
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }
    
    /// Check if the value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    
    /// Check if the value is a symbol
    pub fn is_symbol(&self) -> bool {
        matches!(self, Value::Symbol(_))
    }
    
    /// Check if the value is a BigInt
    pub fn is_bigint(&self) -> bool {
        matches!(self, Value::BigInt(_))
    }
    
    /// Check if the value is an object
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }
    
    /// Check if the value is a function
    pub fn is_function(&self) -> bool {
        matches!(self, Value::Function(_))
    }
    
    /// Check if the value is an array
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }
    
    /// Check if the value is a RegExp
    pub fn is_regexp(&self) -> bool {
        matches!(self, Value::RegExp(_, _))
    }
    
    /// Convert value to boolean according to ECMAScript rules
    pub fn to_boolean(&self) -> bool {
        match self {
            Value::Undefined | Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Symbol(_) => true,
            Value::BigInt(_) => true,
            Value::Object(_) => true,
            Value::Function(_) => true,
            Value::Array(_) => true,
            Value::RegExp(_, _) => true,
        }
    }
    
    /// Convert value to number according to ECMAScript rules
    pub fn to_number(&self) -> f64 {
        match self {
            Value::Undefined => f64::NAN,
            Value::Null => 0.0,
            Value::Boolean(b) => if *b { 1.0 } else { 0.0 },
            Value::Number(n) => *n,
            Value::String(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    0.0
                } else if let Ok(n) = trimmed.parse::<f64>() {
                    n
                } else {
                    f64::NAN
                }
            }
            Value::Symbol(_) => f64::NAN,
            Value::BigInt(_) => f64::NAN, // BigInt to number conversion is complex
            Value::Object(_) => f64::NAN, // Object to number conversion is complex
            Value::Function(_) => f64::NAN,
            Value::Array(_) => f64::NAN,
            Value::RegExp(_, _) => f64::NAN,
        }
    }
    
    /// Convert value to string according to ECMAScript rules
    pub fn to_string(&self) -> String {
        match self {
            Value::Undefined => "undefined".to_string(),
            Value::Null => "null".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => {
                if n.is_nan() {
                    "NaN".to_string()
                } else if n.is_infinite() {
                    if n.is_sign_negative() {
                        "-Infinity".to_string()
                    } else {
                        "Infinity".to_string()
                    }
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Symbol(s) => s.clone(),
            Value::BigInt(s) => s.clone(),
            Value::Object(_) => "[object Object]".to_string(),
            Value::Function(_) => "[object Function]".to_string(),
            Value::Array(_) => "[object Array]".to_string(),
            Value::RegExp(pattern, flags) => format!("/{}/{}", pattern, flags),
        }
    }
    
    /// Check if two values are equal (==)
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Null, Value::Null) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::BigInt(a), Value::BigInt(b)) => a == b,
            _ => false, // Simplified for now
        }
    }
    
    /// Check if two values are strictly equal (===)
    pub fn strict_equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Null, Value::Null) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::BigInt(a), Value::BigInt(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => Rc::ptr_eq(a, b),
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::RegExp(a1, a2), Value::RegExp(b1, b2)) => a1 == b1 && a2 == b2,
            _ => false,
        }
    }
    
    /// Add two values (+)
    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
            (Value::String(a), b) => Value::String(format!("{}{}", a, b.to_string())),
            (a, Value::String(b)) => Value::String(format!("{}{}", a.to_string(), b)),
            (Value::Number(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
            (Value::String(a), Value::Number(b)) => Value::String(format!("{}{}", a, b)),
            _ => Value::Number(self.to_number() + other.to_number()),
        }
    }
    
    /// Subtract two values (-)
    pub fn subtract(&self, other: &Value) -> Value {
        Value::Number(self.to_number() - other.to_number())
    }
    
    /// Multiply two values (*)
    pub fn multiply(&self, other: &Value) -> Value {
        Value::Number(self.to_number() * other.to_number())
    }
    
    /// Divide two values (/)
    pub fn divide(&self, other: &Value) -> Value {
        let divisor = other.to_number();
        if divisor == 0.0 {
            if self.to_number() == 0.0 {
                Value::Number(f64::NAN)
            } else {
                Value::Number(f64::INFINITY)
            }
        } else {
            Value::Number(self.to_number() / divisor)
        }
    }
    
    /// Modulo operation (%)
    pub fn modulo(&self, other: &Value) -> Value {
        let divisor = other.to_number();
        if divisor == 0.0 {
            Value::Number(f64::NAN)
        } else {
            Value::Number(self.to_number() % divisor)
        }
    }
    
    /// Logical AND (&&)
    pub fn logical_and(&self, other: &Value) -> Value {
        if !self.to_boolean() {
            self.clone()
        } else {
            other.clone()
        }
    }
    
    /// Logical OR (||)
    pub fn logical_or(&self, other: &Value) -> Value {
        if self.to_boolean() {
            self.clone()
        } else {
            other.clone()
        }
    }
    
    /// Logical NOT (!)
    pub fn logical_not(&self) -> Value {
        Value::Boolean(!self.to_boolean())
    }
    
    /// Get the type of the value (typeof operator)
    pub fn typeof_(&self) -> String {
        match self {
            Value::Undefined => "undefined".to_string(),
            Value::Null => "object".to_string(), // Historical quirk of JavaScript
            Value::Boolean(_) => "boolean".to_string(),
            Value::Number(_) => "number".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Symbol(_) => "symbol".to_string(),
            Value::BigInt(_) => "bigint".to_string(),
            Value::Object(_) => "object".to_string(),
            Value::Function(_) => "function".to_string(),
            Value::Array(_) => "object".to_string(),
            Value::RegExp(_, _) => "object".to_string(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.strict_equals(other)
    }
}

impl Eq for Value {}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Re-export the actual types
pub use super::object::Object;
pub use super::function::Function; 