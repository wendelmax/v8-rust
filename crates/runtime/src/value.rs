// Value types for ECMAScript

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::object::Object;
use super::function::Function;

#[derive(Debug, Clone)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    BigInt(String),
    Object(Object),
    Function(Function),
    Array(Vec<Value>),
    RegExp(String, String), // pattern, flags
}

impl Value {
    pub fn is_undefined(&self) -> bool {
        matches!(self, Value::Undefined)
    }
    
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
    
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }
    
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }
    
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    
    pub fn is_symbol(&self) -> bool {
        matches!(self, Value::Symbol(_))
    }
    
    pub fn is_bigint(&self) -> bool {
        matches!(self, Value::BigInt(_))
    }
    
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }
    
    pub fn is_function(&self) -> bool {
        matches!(self, Value::Function(_))
    }
    
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }
    
    pub fn is_regexp(&self) -> bool {
        matches!(self, Value::RegExp(_, _))
    }
    
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
            Value::BigInt(s) => {
                if let Ok(n) = s.parse::<f64>() {
                    n
                } else {
                    f64::NAN
                }
            }
            Value::Object(_) => f64::NAN,
            Value::Function(_) => f64::NAN,
            Value::Array(_) => f64::NAN,
            Value::RegExp(_, _) => f64::NAN,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Value::Undefined => "undefined".to_string(),
            Value::Null => "null".to_string(),
            Value::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
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
            Value::Symbol(s) => format!("Symbol({})", s),
            Value::BigInt(s) => format!("{}n", s),
            Value::Object(_) => "[object Object]".to_string(),
            Value::Function(_) => "[object Function]".to_string(),
            Value::Array(_) => "[object Array]".to_string(),
            Value::RegExp(pattern, flags) => format!("/{}/{}", pattern, flags),
        }
    }
    
    pub fn to_object(&self) -> Option<Object> {
        match self {
            Value::Object(obj) => Some(obj.clone()),
            Value::Function(func) => Some(Object::from_function(func.clone())),
            Value::Array(arr) => Some(Object::from_array(arr.clone())),
            Value::String(s) => Some(Object::from_string(s.clone())),
            Value::Number(n) => Some(Object::from_number(*n)),
            Value::Boolean(b) => Some(Object::from_boolean(*b)),
            Value::Symbol(s) => Some(Object::from_symbol(s.clone())),
            Value::BigInt(s) => Some(Object::from_bigint(s.clone())),
            Value::RegExp(pattern, flags) => Some(Object::from_regexp(pattern.clone(), flags.clone())),
            Value::Undefined | Value::Null => None,
        }
    }
    
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Null, Value::Null) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::BigInt(a), Value::BigInt(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => std::ptr::eq(a.as_ref(), b.as_ref()),
            (Value::Function(a), Value::Function(b)) => std::ptr::eq(a.as_ref(), b.as_ref()),
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::RegExp(a_p, a_f), Value::RegExp(b_p, b_f)) => a_p == b_p && a_f == b_f,
            _ => false,
        }
    }
    
    pub fn strict_equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Null, Value::Null) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => {
                if a.is_nan() && b.is_nan() {
                    false
                } else {
                    a == b
                }
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::BigInt(a), Value::BigInt(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => std::ptr::eq(a.as_ref(), b.as_ref()),
            (Value::Function(a), Value::Function(b)) => std::ptr::eq(a.as_ref(), b.as_ref()),
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::RegExp(a_p, a_f), Value::RegExp(b_p, b_f)) => a_p == b_p && a_f == b_f,
            _ => false,
        }
    }
    
    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
            (Value::String(a), b) => Value::String(format!("{}{}", a, b.to_string())),
            (a, Value::String(b)) => Value::String(format!("{}{}", a.to_string(), b)),
            (Value::BigInt(a), Value::BigInt(b)) => {
                if let (Ok(a_val), Ok(b_val)) = (a.parse::<i64>(), b.parse::<i64>()) {
                    Value::BigInt((a_val + b_val).to_string())
                } else {
                    Value::String(format!("{}{}", a, b))
                }
            }
            (a, b) => {
                let a_num = a.to_number();
                let b_num = b.to_number();
                Value::Number(a_num + b_num)
            }
        }
    }
    
    pub fn subtract(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (Value::BigInt(a), Value::BigInt(b)) => {
                if let (Ok(a_val), Ok(b_val)) = (a.parse::<i64>(), b.parse::<i64>()) {
                    Value::BigInt((a_val - b_val).to_string())
                } else {
                    Value::Number(f64::NAN)
                }
            }
            (a, b) => {
                let a_num = a.to_number();
                let b_num = b.to_number();
                Value::Number(a_num - b_num)
            }
        }
    }
    
    pub fn multiply(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::BigInt(a), Value::BigInt(b)) => {
                if let (Ok(a_val), Ok(b_val)) = (a.parse::<i64>(), b.parse::<i64>()) {
                    Value::BigInt((a_val * b_val).to_string())
                } else {
                    Value::Number(f64::NAN)
                }
            }
            (a, b) => {
                let a_num = a.to_number();
                let b_num = b.to_number();
                Value::Number(a_num * b_num)
            }
        }
    }
    
    pub fn divide(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    if *a == 0.0 {
                        Value::Number(f64::NAN)
                    } else if a.is_sign_positive() {
                        Value::Number(f64::INFINITY)
                    } else {
                        Value::Number(f64::NEG_INFINITY)
                    }
                } else {
                    Value::Number(a / b)
                }
            }
            (a, b) => {
                let a_num = a.to_number();
                let b_num = b.to_number();
                Value::Number(a_num / b_num)
            }
        }
    }
    
    pub fn modulo(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    Value::Number(f64::NAN)
                } else {
                    Value::Number(a % b)
                }
            }
            (a, b) => {
                let a_num = a.to_number();
                let b_num = b.to_number();
                Value::Number(a_num % b_num)
            }
        }
    }
    
    pub fn exponentiate(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a.powf(*b)),
            (a, b) => {
                let a_num = a.to_number();
                let b_num = b.to_number();
                Value::Number(a_num.powf(b_num))
            }
        }
    }
    
    pub fn bitwise_and(&self, other: &Value) -> Value {
        let a = self.to_number() as i32;
        let b = other.to_number() as i32;
        Value::Number((a & b) as f64)
    }
    
    pub fn bitwise_or(&self, other: &Value) -> Value {
        let a = self.to_number() as i32;
        let b = other.to_number() as i32;
        Value::Number((a | b) as f64)
    }
    
    pub fn bitwise_xor(&self, other: &Value) -> Value {
        let a = self.to_number() as i32;
        let b = other.to_number() as i32;
        Value::Number((a ^ b) as f64)
    }
    
    pub fn left_shift(&self, other: &Value) -> Value {
        let a = self.to_number() as i32;
        let b = other.to_number() as i32;
        Value::Number((a << b) as f64)
    }
    
    pub fn right_shift(&self, other: &Value) -> Value {
        let a = self.to_number() as i32;
        let b = other.to_number() as i32;
        Value::Number((a >> b) as f64)
    }
    
    pub fn unsigned_right_shift(&self, other: &Value) -> Value {
        let a = self.to_number() as u32;
        let b = other.to_number() as u32;
        Value::Number((a >> b) as f64)
    }
    
    pub fn logical_and(&self, other: &Value) -> Value {
        if self.to_boolean() {
            other.clone()
        } else {
            self.clone()
        }
    }
    
    pub fn logical_or(&self, other: &Value) -> Value {
        if self.to_boolean() {
            self.clone()
        } else {
            other.clone()
        }
    }
    
    pub fn logical_not(&self) -> Value {
        Value::Boolean(!self.to_boolean())
    }
    
    pub fn bitwise_not(&self) -> Value {
        let a = self.to_number() as i32;
        Value::Number((!a) as f64)
    }
    
    pub fn unary_plus(&self) -> Value {
        Value::Number(self.to_number())
    }
    
    pub fn unary_minus(&self) -> Value {
        Value::Number(-self.to_number())
    }
    
    pub fn typeof_(&self) -> String {
        match self {
            Value::Undefined => "undefined".to_string(),
            Value::Null => "object".to_string(),
            Value::Boolean(_) => "boolean".to_string(),
            Value::Number(_) => "number".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Symbol(_) => "symbol".to_string(),
            Value::BigInt(_) => "bigint".to_string(),
            Value::Function(_) => "function".to_string(),
            Value::Object(_) | Value::Array(_) | Value::RegExp(_, _) => "object".to_string(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for Value {}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
} 