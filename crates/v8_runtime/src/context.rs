//! Execution context for V8-Rust JavaScript engine
//! 
//! This module provides the execution context for JavaScript code.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::value::Value;
use super::object::Object;

/// Execution context
#[derive(Debug, Clone)]
pub struct Context {
    pub global_object: Rc<RefCell<Object>>,
    pub variables: HashMap<String, Value>,
    pub this_value: Value,
}

impl Context {
    /// Create a new execution context
    pub fn new(global_object: Rc<RefCell<Object>>) -> Self {
        Self {
            global_object,
            variables: HashMap::new(),
            this_value: Value::Undefined,
        }
    }
    
    /// Set a variable in the context
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    /// Get a variable from the context
    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }
    
    /// Check if a variable exists in the context
    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
    
    /// Set the this value
    pub fn set_this(&mut self, this: Value) {
        self.this_value = this;
    }
    
    /// Get the this value
    pub fn get_this(&self) -> Value {
        self.this_value.clone()
    }
    
    /// Get the global object
    pub fn get_global_object(&self) -> Rc<RefCell<Object>> {
        self.global_object.clone()
    }
} 