//! Object system for V8-Rust JavaScript engine
//! 
//! This module provides the core object system for JavaScript objects.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::value::Value;

/// Property descriptor for object properties
#[derive(Debug, Clone)]
pub struct PropertyDescriptor {
    pub value: Option<Value>,
    pub writable: Option<bool>,
    pub enumerable: Option<bool>,
    pub configurable: Option<bool>,
}

impl PropertyDescriptor {
    /// Create a new empty property descriptor
    pub fn new() -> Self {
        Self {
            value: None,
            writable: None,
            enumerable: None,
            configurable: None,
        }
    }
    
    /// Create a data descriptor
    pub fn data_descriptor(value: Value, writable: bool, enumerable: bool, configurable: bool) -> Self {
        Self {
            value: Some(value),
            writable: Some(writable),
            enumerable: Some(enumerable),
            configurable: Some(configurable),
        }
    }
    
    /// Check if this is a data descriptor
    pub fn is_data_descriptor(&self) -> bool {
        self.value.is_some() || self.writable.is_some()
    }
}

/// JavaScript object
#[derive(Debug, Clone)]
pub struct Object {
    pub properties: HashMap<String, PropertyDescriptor>,
    pub prototype: Option<Rc<RefCell<Object>>>,
    pub extensible: bool,
}

impl Object {
    /// Create a new empty object
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            prototype: None,
            extensible: true,
        }
    }
    
    /// Create an object with a prototype
    pub fn with_prototype(prototype: Rc<RefCell<Object>>) -> Self {
        Self {
            properties: HashMap::new(),
            prototype: Some(prototype),
            extensible: true,
        }
    }
    
    /// Set a property on the object
    pub fn set_property(&mut self, name: String, value: Value) {
        let descriptor = PropertyDescriptor::data_descriptor(value, true, true, true);
        self.properties.insert(name, descriptor);
    }
    
    /// Get a property from the object
    pub fn get_property(&self, name: &str) -> Option<Value> {
        if let Some(descriptor) = self.properties.get(name) {
            descriptor.value.clone()
        } else if let Some(ref prototype) = self.prototype {
            prototype.borrow().get_property(name)
        } else {
            None
        }
    }
    
    /// Check if the object has a property
    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name) || 
        self.prototype.as_ref().map_or(false, |p| p.borrow().has_property(name))
    }
    
    /// Delete a property from the object
    pub fn delete_property(&mut self, name: &str) -> bool {
        if let Some(descriptor) = self.properties.get(name) {
            if descriptor.configurable.unwrap_or(true) {
                self.properties.remove(name);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    
    /// Define a property on the object
    pub fn define_property(&mut self, name: String, descriptor: PropertyDescriptor) -> bool {
        self.properties.insert(name, descriptor);
        true
    }
    
    /// Get all own property names
    pub fn get_own_property_names(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }
    
    /// Prevent extensions on the object
    pub fn prevent_extensions(&mut self) {
        self.extensible = false;
    }
    
    /// Check if the object is extensible
    pub fn is_extensible(&self) -> bool {
        self.extensible
    }
} 