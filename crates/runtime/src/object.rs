// Object system for ECMAScript

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::value::Value;
use super::function::Function;

#[derive(Debug, Clone)]
pub struct PropertyDescriptor {
    pub value: Option<Value>,
    pub writable: Option<bool>,
    pub enumerable: Option<bool>,
    pub configurable: Option<bool>,
    pub get: Option<Function>,
    pub set: Option<Function>,
}

impl PropertyDescriptor {
    pub fn new() -> Self {
        Self {
            value: None,
            writable: None,
            enumerable: None,
            configurable: None,
            get: None,
            set: None,
        }
    }
    
    pub fn data_descriptor(value: Value, writable: bool, enumerable: bool, configurable: bool) -> Self {
        Self {
            value: Some(value),
            writable: Some(writable),
            enumerable: Some(enumerable),
            configurable: Some(configurable),
            get: None,
            set: None,
        }
    }
    
    pub fn accessor_descriptor(get: Option<Function>, set: Option<Function>, enumerable: bool, configurable: bool) -> Self {
        Self {
            value: None,
            writable: None,
            enumerable: Some(enumerable),
            configurable: Some(configurable),
            get,
            set,
        }
    }
    
    pub fn is_data_descriptor(&self) -> bool {
        self.value.is_some() || self.writable.is_some()
    }
    
    pub fn is_accessor_descriptor(&self) -> bool {
        self.get.is_some() || self.set.is_some()
    }
    
    pub fn is_generic_descriptor(&self) -> bool {
        !self.is_data_descriptor() && !self.is_accessor_descriptor()
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub properties: HashMap<String, PropertyDescriptor>,
    pub prototype: Option<Rc<RefCell<Object>>>,
    pub extensible: bool,
    pub object_type: ObjectType,
}

#[derive(Debug, Clone)]
pub enum ObjectType {
    Object,
    Array,
    Function,
    String,
    Number,
    Boolean,
    Symbol,
    BigInt,
    RegExp,
    Date,
    Error,
    Map,
    Set,
    WeakMap,
    WeakSet,
    Promise,
    Proxy,
}

impl Object {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            prototype: None,
            extensible: true,
            object_type: ObjectType::Object,
        }
    }
    
    pub fn with_prototype(prototype: Rc<RefCell<Object>>) -> Self {
        Self {
            properties: HashMap::new(),
            prototype: Some(prototype),
            extensible: true,
            object_type: ObjectType::Object,
        }
    }
    
    pub fn constructor() -> Self {
        let mut obj = Self::new();
        obj.object_type = ObjectType::Function;
        
        // Add constructor properties
        obj.set_property("prototype".to_string(), Value::Object(Object::new()));
        obj.set_property("length".to_string(), Value::Number(1.0));
        obj.set_property("name".to_string(), Value::String("Object".to_string()));
        
        obj
    }
    
    pub fn from_function(func: Function) -> Self {
        let mut obj = Self::new();
        obj.object_type = ObjectType::Function;
        
        obj.set_property("call".to_string(), Value::Function(Function::native("call", |args| {
            // TODO: Implement function call
            Ok(Value::Undefined)
        })));
        
        obj.set_property("apply".to_string(), Value::Function(Function::native("apply", |args| {
            // TODO: Implement function apply
            Ok(Value::Undefined)
        })));
        
        obj.set_property("bind".to_string(), Value::Function(Function::native("bind", |args| {
            // TODO: Implement function bind
            Ok(Value::Undefined)
        })));
        
        obj
    }
    
    pub fn from_array(arr: Vec<Value>) -> Self {
        let mut obj = Self::new();
        obj.object_type = ObjectType::Array;
        
        // Set array elements
        for (index, value) in arr.iter().enumerate() {
            obj.set_property(index.to_string(), value.clone());
        }
        
        // Set length property
        obj.set_property("length".to_string(), Value::Number(arr.len() as f64));
        
        obj
    }
    
    pub fn from_string(s: String) -> Self {
        let mut obj = Self::new();
        obj.object_type = ObjectType::String;
        
        obj.set_property("length".to_string(), Value::Number(s.len() as f64));
        obj.set_property("0".to_string(), Value::String(s.clone()));
        
        // Add string methods
        obj.set_property("charAt".to_string(), Value::Function(Function::native("charAt", |args| {
            // TODO: Implement charAt
            Ok(Value::String("".to_string()))
        })));
        
        obj.set_property("charCodeAt".to_string(), Value::Function(Function::native("charCodeAt", |args| {
            // TODO: Implement charCodeAt
            Ok(Value::Number(0.0))
        })));
        
        obj.set_property("indexOf".to_string(), Value::Function(Function::native("indexOf", |args| {
            // TODO: Implement indexOf
            Ok(Value::Number(-1.0))
        })));
        
        obj.set_property("substring".to_string(), Value::Function(Function::native("substring", |args| {
            // TODO: Implement substring
            Ok(Value::String("".to_string()))
        })));
        
        obj.set_property("toLowerCase".to_string(), Value::Function(Function::native("toLowerCase", |args| {
            // TODO: Implement toLowerCase
            Ok(Value::String("".to_string()))
        })));
        
        obj.set_property("toUpperCase".to_string(), Value::Function(Function::native("toUpperCase", |args| {
            // TODO: Implement toUpperCase
            Ok(Value::String("".to_string()))
        })));
        
        obj
    }
    
    pub fn from_number(n: f64) -> Self {
        let mut obj = Self::new();
        obj.object_type = ObjectType::Number;
        
        obj.set_property("valueOf".to_string(), Value::Function(Function::native("valueOf", |_| {
            Ok(Value::Number(n))
        })));
        
        obj.set_property("toString".to_string(), Value::Function(Function::native("toString", |args| {
            let radix = if args.len() > 0 {
                match &args[0] {
                    Value::Number(r) => *r as i32,
                    _ => 10,
                }
            } else {
                10
            };
            
            if radix == 10 {
                Ok(Value::String(n.to_string()))
            } else {
                // TODO: Implement radix conversion
                Ok(Value::String(n.to_string()))
            }
        })));
        
        obj
    }
    
    pub fn from_boolean(b: bool) -> Self {
        let mut obj = Self::new();
        obj.object_type = ObjectType::Boolean;
        
        obj.set_property("valueOf".to_string(), Value::Function(Function::native("valueOf", |_| {
            Ok(Value::Boolean(b))
        })));
        
        obj.set_property("toString".to_string(), Value::Function(Function::native("toString", |_| {
            Ok(Value::String(if b { "true".to_string() } else { "false".to_string() }))
        })));
        
        obj
    }
    
    pub fn from_symbol(s: String) -> Self {
        let mut obj = Self::new();
        obj.object_type = ObjectType::Symbol;
        
        obj.set_property("description".to_string(), Value::String(s.clone()));
        obj.set_property("toString".to_string(), Value::Function(Function::native("toString", |_| {
            Ok(Value::String(format!("Symbol({})", s)))
        })));
        
        obj
    }
    
    pub fn from_bigint(s: String) -> Self {
        let mut obj = Self::new();
        obj.object_type = ObjectType::BigInt;
        
        obj.set_property("valueOf".to_string(), Value::Function(Function::native("valueOf", |_| {
            Ok(Value::BigInt(s.clone()))
        })));
        
        obj.set_property("toString".to_string(), Value::Function(Function::native("toString", |_| {
            Ok(Value::String(format!("{}n", s)))
        })));
        
        obj
    }
    
    pub fn from_regexp(pattern: String, flags: String) -> Self {
        let mut obj = Self::new();
        obj.object_type = ObjectType::RegExp;
        
        obj.set_property("source".to_string(), Value::String(pattern.clone()));
        obj.set_property("flags".to_string(), Value::String(flags.clone()));
        obj.set_property("global".to_string(), Value::Boolean(flags.contains('g')));
        obj.set_property("ignoreCase".to_string(), Value::Boolean(flags.contains('i')));
        obj.set_property("multiline".to_string(), Value::Boolean(flags.contains('m')));
        obj.set_property("sticky".to_string(), Value::Boolean(flags.contains('y')));
        obj.set_property("unicode".to_string(), Value::Boolean(flags.contains('u')));
        
        obj.set_property("test".to_string(), Value::Function(Function::native("test", |args| {
            // TODO: Implement regex test
            Ok(Value::Boolean(false))
        })));
        
        obj.set_property("exec".to_string(), Value::Function(Function::native("exec", |args| {
            // TODO: Implement regex exec
            Ok(Value::Null)
        })));
        
        obj
    }
    
    pub fn set_property(&mut self, name: String, value: Value) {
        let descriptor = PropertyDescriptor::data_descriptor(value, true, true, true);
        self.properties.insert(name, descriptor);
    }
    
    pub fn get_property(&self, name: &str) -> Option<Value> {
        // Check own properties first
        if let Some(descriptor) = self.properties.get(name) {
            if let Some(value) = &descriptor.value {
                return Some(value.clone());
            }
            if let Some(getter) = &descriptor.get {
                // TODO: Call getter function
                return Some(Value::Undefined);
            }
        }
        
        // Check prototype chain
        if let Some(prototype) = &self.prototype {
            return prototype.borrow().get_property(name);
        }
        
        None
    }
    
    pub fn has_property(&self, name: &str) -> bool {
        if self.properties.contains_key(name) {
            return true;
        }
        
        if let Some(prototype) = &self.prototype {
            return prototype.borrow().has_property(name);
        }
        
        false
    }
    
    pub fn delete_property(&mut self, name: &str) -> bool {
        if let Some(descriptor) = self.properties.get(name) {
            if descriptor.configurable.unwrap_or(true) {
                self.properties.remove(name);
                return true;
            }
            return false;
        }
        
        if let Some(prototype) = &self.prototype {
            return prototype.borrow_mut().delete_property(name);
        }
        
        true
    }
    
    pub fn define_property(&mut self, name: String, descriptor: PropertyDescriptor) -> bool {
        if let Some(existing) = self.properties.get(&name) {
            if !existing.configurable.unwrap_or(true) {
                return false;
            }
        }
        
        self.properties.insert(name, descriptor);
        true
    }
    
    pub fn get_own_property_names(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }
    
    pub fn get_property_names(&self) -> Vec<String> {
        let mut names = self.get_own_property_names();
        
        if let Some(prototype) = &self.prototype {
            let mut prototype_names = prototype.borrow().get_property_names();
            names.append(&mut prototype_names);
        }
        
        names
    }
    
    pub fn prevent_extensions(&mut self) {
        self.extensible = false;
    }
    
    pub fn seal(&mut self) {
        self.extensible = false;
        for descriptor in self.properties.values_mut() {
            descriptor.configurable = Some(false);
        }
    }
    
    pub fn freeze(&mut self) {
        self.extensible = false;
        for descriptor in self.properties.values_mut() {
            descriptor.configurable = Some(false);
            descriptor.writable = Some(false);
        }
    }
    
    pub fn is_extensible(&self) -> bool {
        self.extensible
    }
    
    pub fn is_sealed(&self) -> bool {
        if self.extensible {
            return false;
        }
        
        for descriptor in self.properties.values() {
            if descriptor.configurable.unwrap_or(true) {
                return false;
            }
        }
        
        true
    }
    
    pub fn is_frozen(&self) -> bool {
        if self.extensible {
            return false;
        }
        
        for descriptor in self.properties.values() {
            if descriptor.configurable.unwrap_or(true) || descriptor.writable.unwrap_or(true) {
                return false;
            }
        }
        
        true
    }
    
    pub fn call(&self, this: Value, args: &[Value]) -> Result<Value, String> {
        match self.object_type {
            ObjectType::Function => {
                // TODO: Implement function call
                Ok(Value::Undefined)
            }
            _ => Err("Object is not callable".to_string()),
        }
    }
    
    pub fn construct(&self, args: &[Value]) -> Result<Object, String> {
        match self.object_type {
            ObjectType::Function => {
                // TODO: Implement constructor call
                Ok(Object::new())
            }
            _ => Err("Object is not a constructor".to_string()),
        }
    }
} 