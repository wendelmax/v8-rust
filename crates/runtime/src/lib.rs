// Runtime para JavaScript - ECMAScript completo

pub mod value;
pub mod object;
pub mod function;
pub mod array;
pub mod string;
pub mod number;
pub mod boolean;
pub mod symbol;
pub mod error;
pub mod global;
pub mod context;
pub mod scope;
pub mod environment;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub use value::Value;
pub use object::Object;
pub use function::Function;
pub use context::Context;
pub use scope::Scope;
pub use environment::Environment;

#[derive(Debug, Clone)]
pub struct Runtime {
    pub global_object: Rc<RefCell<Object>>,
    pub contexts: HashMap<String, Context>,
}

impl Runtime {
    pub fn new() -> Self {
        let global = Object::new();
        let mut runtime = Runtime {
            global_object: Rc::new(RefCell::new(global)),
            contexts: HashMap::new(),
        };
        
        // Initialize global objects and functions
        runtime.init_global_objects();
        runtime
    }
    
    fn init_global_objects(&mut self) {
        let global = self.global_object.clone();
        
        // Global constructors
        global.borrow_mut().set_property("Object".to_string(), Value::Object(Object::constructor()));
        global.borrow_mut().set_property("Array".to_string(), Value::Object(Array::constructor()));
        global.borrow_mut().set_property("Function".to_string(), Value::Object(Function::constructor()));
        global.borrow_mut().set_property("String".to_string(), Value::Object(String::constructor()));
        global.borrow_mut().set_property("Number".to_string(), Value::Object(Number::constructor()));
        global.borrow_mut().set_property("Boolean".to_string(), Value::Object(Boolean::constructor()));
        global.borrow_mut().set_property("Symbol".to_string(), Value::Object(Symbol::constructor()));
        global.borrow_mut().set_property("Error".to_string(), Value::Object(Error::constructor()));
        global.borrow_mut().set_property("RegExp".to_string(), Value::Object(RegExp::constructor()));
        global.borrow_mut().set_property("Date".to_string(), Value::Object(Date::constructor()));
        global.borrow_mut().set_property("Math".to_string(), Value::Object(Math::object()));
        global.borrow_mut().set_property("JSON".to_string(), Value::Object(JSON::object()));
        global.borrow_mut().set_property("Promise".to_string(), Value::Object(Promise::constructor()));
        global.borrow_mut().set_property("Map".to_string(), Value::Object(Map::constructor()));
        global.borrow_mut().set_property("Set".to_string(), Value::Object(Set::constructor()));
        global.borrow_mut().set_property("WeakMap".to_string(), Value::Object(WeakMap::constructor()));
        global.borrow_mut().set_property("WeakSet".to_string(), Value::Object(WeakSet::constructor()));
        global.borrow_mut().set_property("Proxy".to_string(), Value::Object(Proxy::constructor()));
        global.borrow_mut().set_property("Reflect".to_string(), Value::Object(Reflect::object()));
        
        // Global functions
        global.borrow_mut().set_property("eval".to_string(), Value::Function(Function::native("eval", eval)));
        global.borrow_mut().set_property("parseInt".to_string(), Value::Function(Function::native("parseInt", parseInt)));
        global.borrow_mut().set_property("parseFloat".to_string(), Value::Function(Function::native("parseFloat", parseFloat)));
        global.borrow_mut().set_property("isNaN".to_string(), Value::Function(Function::native("isNaN", is_nan)));
        global.borrow_mut().set_property("isFinite".to_string(), Value::Function(Function::native("isFinite", is_finite)));
        global.borrow_mut().set_property("decodeURI".to_string(), Value::Function(Function::native("decodeURI", decode_uri)));
        global.borrow_mut().set_property("decodeURIComponent".to_string(), Value::Function(Function::native("decodeURIComponent", decode_uri_component)));
        global.borrow_mut().set_property("encodeURI".to_string(), Value::Function(Function::native("encodeURI", encode_uri)));
        global.borrow_mut().set_property("encodeURIComponent".to_string(), Value::Function(Function::native("encodeURIComponent", encode_uri_component)));
        global.borrow_mut().set_property("escape".to_string(), Value::Function(Function::native("escape", escape)));
        global.borrow_mut().set_property("unescape".to_string(), Value::Function(Function::native("unescape", unescape)));
        
        // Global values
        global.borrow_mut().set_property("undefined".to_string(), Value::Undefined);
        global.borrow_mut().set_property("NaN".to_string(), Value::Number(f64::NAN));
        global.borrow_mut().set_property("Infinity".to_string(), Value::Number(f64::INFINITY));
        global.borrow_mut().set_property("-Infinity".to_string(), Value::Number(f64::NEG_INFINITY));
    }
    
    pub fn create_context(&mut self, name: String) -> Context {
        let context = Context::new(self.global_object.clone());
        self.contexts.insert(name.clone(), context.clone());
        context
    }
    
    pub fn get_context(&self, name: &str) -> Option<&Context> {
        self.contexts.get(name)
    }
    
    pub fn execute(&self, code: &str) -> Result<Value, String> {
        // This will be implemented when we have the parser and VM
        // For now, return undefined
        Ok(Value::Undefined)
    }
}

// Native function implementations
fn eval(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::Undefined);
    }
    
    let arg = &args[0];
    match arg {
        Value::String(s) => {
            // TODO: Parse and execute the string as JavaScript code
            Ok(Value::Undefined)
        }
        _ => Ok(arg.clone()),
    }
}

fn parseInt(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::Number(f64::NAN));
    }
    
    let arg = &args[0];
    let radix = if args.len() > 1 {
        match &args[1] {
            Value::Number(n) => *n as i32,
            _ => 10,
        }
    } else {
        10
    };
    
    match arg {
        Value::String(s) => {
            let trimmed = s.trim_start();
            if let Ok(n) = i64::from_str_radix(trimmed, radix) {
                Ok(Value::Number(n as f64))
            } else {
                Ok(Value::Number(f64::NAN))
            }
        }
        Value::Number(n) => Ok(Value::Number(*n)),
        _ => Ok(Value::Number(f64::NAN)),
    }
}

fn parseFloat(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::Number(f64::NAN));
    }
    
    let arg = &args[0];
    match arg {
        Value::String(s) => {
            if let Ok(n) = s.parse::<f64>() {
                Ok(Value::Number(n))
            } else {
                Ok(Value::Number(f64::NAN))
            }
        }
        Value::Number(n) => Ok(Value::Number(*n)),
        _ => Ok(Value::Number(f64::NAN)),
    }
}

fn is_nan(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::Boolean(true));
    }
    
    let arg = &args[0];
    match arg {
        Value::Number(n) => Ok(Value::Boolean(n.is_nan())),
        _ => Ok(Value::Boolean(true)),
    }
}

fn is_finite(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::Boolean(false));
    }
    
    let arg = &args[0];
    match arg {
        Value::Number(n) => Ok(Value::Boolean(n.is_finite())),
        _ => Ok(Value::Boolean(false)),
    }
}

fn decode_uri(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::String("".to_string()));
    }
    
    let arg = &args[0];
    match arg {
        Value::String(s) => {
            // TODO: Implement proper URI decoding
            Ok(Value::String(s.clone()))
        }
        _ => Ok(Value::String("".to_string())),
    }
}

fn decode_uri_component(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::String("".to_string()));
    }
    
    let arg = &args[0];
    match arg {
        Value::String(s) => {
            // TODO: Implement proper URI component decoding
            Ok(Value::String(s.clone()))
        }
        _ => Ok(Value::String("".to_string())),
    }
}

fn encode_uri(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::String("".to_string()));
    }
    
    let arg = &args[0];
    match arg {
        Value::String(s) => {
            // TODO: Implement proper URI encoding
            Ok(Value::String(s.clone()))
        }
        _ => Ok(Value::String("".to_string())),
    }
}

fn encode_uri_component(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::String("".to_string()));
    }
    
    let arg = &args[0];
    match arg {
        Value::String(s) => {
            // TODO: Implement proper URI component encoding
            Ok(Value::String(s.clone()))
        }
        _ => Ok(Value::String("".to_string())),
    }
}

fn escape(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::String("".to_string()));
    }
    
    let arg = &args[0];
    match arg {
        Value::String(s) => {
            // TODO: Implement escape encoding
            Ok(Value::String(s.clone()))
        }
        _ => Ok(Value::String("".to_string())),
    }
}

fn unescape(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::String("".to_string()));
    }
    
    let arg = &args[0];
    match arg {
        Value::String(s) => {
            // TODO: Implement unescape decoding
            Ok(Value::String(s.clone()))
        }
        _ => Ok(Value::String("".to_string())),
    }
}

// Placeholder structs for constructors
struct Array;
struct Number;
struct Boolean;
struct Symbol;
struct Error;
struct RegExp;
struct Date;
struct Math;
struct JSON;
struct Promise;
struct Map;
struct Set;
struct WeakMap;
struct WeakSet;
struct Proxy;
struct Reflect;

impl Array {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Number {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Boolean {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Symbol {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Error {
    fn constructor() -> Object {
        Object::new()
    }
}

impl RegExp {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Date {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Math {
    fn object() -> Object {
        Object::new()
    }
}

impl JSON {
    fn object() -> Object {
        Object::new()
    }
}

impl Promise {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Map {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Set {
    fn constructor() -> Object {
        Object::new()
    }
}

impl WeakMap {
    fn constructor() -> Object {
        Object::new()
    }
}

impl WeakSet {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Proxy {
    fn constructor() -> Object {
        Object::new()
    }
}

impl Reflect {
    fn object() -> Object {
        Object::new()
    }
}
