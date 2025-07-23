// Function system for ECMAScript

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::value::Value;
use super::object::Object;
use super::context::Context;
use super::scope::Scope;

pub type NativeFunction = fn(&[Value]) -> Result<Value, String>;

#[derive(Debug, Clone)]
pub enum FunctionBody {
    Native(NativeFunction),
    UserDefined {
        params: Vec<String>,
        body: String, // AST representation
        scope: Rc<RefCell<Scope>>,
    },
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub body: FunctionBody,
    pub prototype: Rc<RefCell<Object>>,
    pub length: usize,
    pub constructor: bool,
    pub generator: bool,
    pub async: bool,
    pub strict: bool,
}

impl Function {
    pub fn new(name: String, body: FunctionBody) -> Self {
        let length = match &body {
            FunctionBody::Native(_) => 0,
            FunctionBody::UserDefined { params, .. } => params.len(),
        };
        
        Self {
            name,
            body,
            prototype: Rc::new(RefCell::new(Object::new())),
            length,
            constructor: false,
            generator: false,
            async: false,
            strict: false,
        }
    }
    
    pub fn native(name: &str, func: NativeFunction) -> Self {
        Self::new(name.to_string(), FunctionBody::Native(func))
    }
    
    pub fn user_defined(name: String, params: Vec<String>, body: String, scope: Rc<RefCell<Scope>>) -> Self {
        let body = FunctionBody::UserDefined {
            params,
            body,
            scope,
        };
        Self::new(name, body)
    }
    
    pub fn constructor() -> Self {
        let mut func = Self::native("Function", |args| {
            // TODO: Implement Function constructor
            Ok(Value::Undefined)
        });
        func.constructor = true;
        func
    }
    
    pub fn call(&self, this: Value, args: &[Value], context: &mut Context) -> Result<Value, String> {
        match &self.body {
            FunctionBody::Native(native_func) => {
                native_func(args)
            }
            FunctionBody::UserDefined { params, body, scope } => {
                // TODO: Implement user-defined function execution
                // This will require:
                // 1. Creating a new execution context
                // 2. Setting up the scope chain
                // 3. Binding parameters
                // 4. Executing the function body
                Ok(Value::Undefined)
            }
        }
    }
    
    pub fn construct(&self, args: &[Value], context: &mut Context) -> Result<Object, String> {
        if !self.constructor {
            return Err("Function is not a constructor".to_string());
        }
        
        // Create a new object with this function's prototype
        let mut obj = Object::with_prototype(self.prototype.clone());
        
        // Call the function with the new object as 'this'
        let this = Value::Object(obj.clone());
        self.call(this, args, context)?;
        
        Ok(obj)
    }
    
    pub fn bind(&self, this: Value, args: &[Value]) -> Result<Function, String> {
        // TODO: Implement function binding
        // This should create a new function with bound 'this' and partial arguments
        Ok(self.clone())
    }
    
    pub fn apply(&self, this: Value, args: &[Value], context: &mut Context) -> Result<Value, String> {
        self.call(this, args, context)
    }
    
    pub fn get_prototype(&self) -> Rc<RefCell<Object>> {
        self.prototype.clone()
    }
    
    pub fn set_prototype(&mut self, prototype: Rc<RefCell<Object>>) {
        self.prototype = prototype;
    }
    
    pub fn is_constructor(&self) -> bool {
        self.constructor
    }
    
    pub fn is_generator(&self) -> bool {
        self.generator
    }
    
    pub fn is_async(&self) -> bool {
        self.async
    }
    
    pub fn is_strict(&self) -> bool {
        self.strict
    }
    
    pub fn get_length(&self) -> usize {
        self.length
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// Built-in function constructors
pub mod builtins {
    use super::*;
    
    pub fn object_constructor(args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Ok(Value::Object(Object::new()));
        }
        
        let arg = &args[0];
        match arg {
            Value::Null | Value::Undefined => Ok(Value::Object(Object::new())),
            Value::Object(obj) => Ok(Value::Object(obj.clone())),
            Value::Boolean(b) => Ok(Value::Object(Object::from_boolean(*b))),
            Value::Number(n) => Ok(Value::Object(Object::from_number(*n))),
            Value::String(s) => Ok(Value::Object(Object::from_string(s.clone()))),
            Value::Symbol(s) => Ok(Value::Object(Object::from_symbol(s.clone()))),
            Value::BigInt(s) => Ok(Value::Object(Object::from_bigint(s.clone()))),
            Value::Array(arr) => Ok(Value::Object(Object::from_array(arr.clone()))),
            Value::Function(func) => Ok(Value::Object(Object::from_function(func.clone()))),
            Value::RegExp(pattern, flags) => Ok(Value::Object(Object::from_regexp(pattern.clone(), flags.clone()))),
        }
    }
    
    pub fn array_constructor(args: &[Value]) -> Result<Value, String> {
        if args.len() == 1 && args[0].is_number() {
            let length = args[0].to_number() as usize;
            let mut arr = Vec::new();
            for _ in 0..length {
                arr.push(Value::Undefined);
            }
            Ok(Value::Array(arr))
        } else {
            Ok(Value::Array(args.to_vec()))
        }
    }
    
    pub fn string_constructor(args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            Ok(Value::String("".to_string()))
        } else {
            Ok(Value::String(args[0].to_string()))
        }
    }
    
    pub fn number_constructor(args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            Ok(Value::Number(0.0))
        } else {
            Ok(Value::Number(args[0].to_number()))
        }
    }
    
    pub fn boolean_constructor(args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            Ok(Value::Boolean(false))
        } else {
            Ok(Value::Boolean(args[0].to_boolean()))
        }
    }
    
    pub fn symbol_constructor(args: &[Value]) -> Result<Value, String> {
        let description = if args.is_empty() {
            "".to_string()
        } else {
            args[0].to_string()
        };
        Ok(Value::Symbol(description))
    }
    
    pub fn regexp_constructor(args: &[Value]) -> Result<Value, String> {
        let pattern = if args.is_empty() {
            "".to_string()
        } else {
            args[0].to_string()
        };
        
        let flags = if args.len() > 1 {
            args[1].to_string()
        } else {
            "".to_string()
        };
        
        Ok(Value::RegExp(pattern, flags))
    }
    
    pub fn date_constructor(args: &[Value]) -> Result<Value, String> {
        // TODO: Implement Date constructor
        Ok(Value::Object(Object::new()))
    }
    
    pub fn error_constructor(args: &[Value]) -> Result<Value, String> {
        let message = if args.is_empty() {
            "".to_string()
        } else {
            args[0].to_string()
        };
        
        let mut obj = Object::new();
        obj.set_property("message".to_string(), Value::String(message));
        obj.set_property("name".to_string(), Value::String("Error".to_string()));
        
        Ok(Value::Object(obj))
    }
    
    pub fn promise_constructor(args: &[Value]) -> Result<Value, String> {
        // TODO: Implement Promise constructor
        Ok(Value::Object(Object::new()))
    }
    
    pub fn map_constructor(args: &[Value]) -> Result<Value, String> {
        // TODO: Implement Map constructor
        Ok(Value::Object(Object::new()))
    }
    
    pub fn set_constructor(args: &[Value]) -> Result<Value, String> {
        // TODO: Implement Set constructor
        Ok(Value::Object(Object::new()))
    }
    
    pub fn weakmap_constructor(args: &[Value]) -> Result<Value, String> {
        // TODO: Implement WeakMap constructor
        Ok(Value::Object(Object::new()))
    }
    
    pub fn weakset_constructor(args: &[Value]) -> Result<Value, String> {
        // TODO: Implement WeakSet constructor
        Ok(Value::Object(Object::new()))
    }
    
    pub fn proxy_constructor(args: &[Value]) -> Result<Value, String> {
        // TODO: Implement Proxy constructor
        Ok(Value::Object(Object::new()))
    }
}

// Function prototype methods
pub mod prototype {
    use super::*;
    
    pub fn call(this: Value, args: &[Value], context: &mut Context) -> Result<Value, String> {
        if let Value::Function(func) = this {
            let this_arg = if args.is_empty() {
                Value::Undefined
            } else {
                args[0].clone()
            };
            
            let call_args = if args.len() > 1 {
                &args[1..]
            } else {
                &[]
            };
            
            func.call(this_arg, call_args, context)
        } else {
            Err("Function.prototype.call called on non-function".to_string())
        }
    }
    
    pub fn apply(this: Value, args: &[Value], context: &mut Context) -> Result<Value, String> {
        if let Value::Function(func) = this {
            let this_arg = if args.is_empty() {
                Value::Undefined
            } else {
                args[0].clone()
            };
            
            let apply_args = if args.len() > 1 {
                if let Value::Array(arr) = &args[1] {
                    arr.as_slice()
                } else {
                    &[]
                }
            } else {
                &[]
            };
            
            func.apply(this_arg, apply_args, context)
        } else {
            Err("Function.prototype.apply called on non-function".to_string())
        }
    }
    
    pub fn bind(this: Value, args: &[Value], _context: &mut Context) -> Result<Value, String> {
        if let Value::Function(func) = this {
            let this_arg = if args.is_empty() {
                Value::Undefined
            } else {
                args[0].clone()
            };
            
            let bind_args = if args.len() > 1 {
                &args[1..]
            } else {
                &[]
            };
            
            match func.bind(this_arg, bind_args) {
                Ok(bound_func) => Ok(Value::Function(bound_func)),
                Err(e) => Err(e),
            }
        } else {
            Err("Function.prototype.bind called on non-function".to_string())
        }
    }
    
    pub fn to_string(this: Value, _args: &[Value], _context: &mut Context) -> Result<Value, String> {
        if let Value::Function(func) = this {
            Ok(Value::String(format!("function {}() {{ [native code] }}", func.get_name())))
        } else {
            Err("Function.prototype.toString called on non-function".to_string())
        }
    }
} 