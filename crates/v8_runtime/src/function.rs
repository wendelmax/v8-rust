//! Function system for V8-Rust JavaScript engine
//! 
//! This module provides the core function system for JavaScript functions.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::value::Value;
use super::object::Object;

/// Function type
#[derive(Debug, Clone)]
pub enum FunctionType {
    Native(NativeFunction),
    User(UserFunction),
}

/// Native function (built-in)
pub type NativeFunction = fn(&[Value]) -> Result<Value, String>;

/// User-defined function
#[derive(Debug, Clone)]
pub struct UserFunction {
    pub params: Vec<String>,
    pub body: String, // For now, just store as string
    pub scope: Rc<RefCell<HashMap<String, Value>>>,
}

/// JavaScript function
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub function_type: FunctionType,
    pub prototype: Rc<RefCell<Object>>,
    pub length: usize,
}

impl Function {
    /// Create a new native function
    pub fn native(name: &str, func: NativeFunction) -> Self {
        Self {
            name: name.to_string(),
            function_type: FunctionType::Native(func),
            prototype: Rc::new(RefCell::new(Object::new())),
            length: 0, // Will be set based on function signature
        }
    }
    
    /// Create a new user function
    pub fn user(name: &str, params: Vec<String>, body: String) -> Self {
        Self {
            name: name.to_string(),
            function_type: FunctionType::User(UserFunction {
                params: params.clone(),
                body,
                scope: Rc::new(RefCell::new(HashMap::new())),
            }),
            prototype: Rc::new(RefCell::new(Object::new())),
            length: params.len(),
        }
    }
    
    /// Call the function
    pub fn call(&self, this: Value, args: &[Value]) -> Result<Value, String> {
        match &self.function_type {
            FunctionType::Native(func) => {
                func(args)
            }
            FunctionType::User(user_func) => {
                // For now, return undefined for user functions
                // This will be implemented when we have a proper interpreter
                Ok(Value::Undefined)
            }
        }
    }
    
    /// Construct the function (new operator)
    pub fn construct(&self, args: &[Value]) -> Result<Object, String> {
        // For now, create a new object
        // This will be implemented properly when we have constructor support
        Ok(Object::new())
    }
    
    /// Get the function name
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    /// Get the function length (number of parameters)
    pub fn get_length(&self) -> usize {
        self.length
    }
} 