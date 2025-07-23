// Environment system for ECMAScript

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::value::Value;
use super::object::Object;
use super::scope::Scope;

#[derive(Debug, Clone)]
pub enum EnvironmentType {
    Lexical,
    Object,
    Function,
    Module,
    Global,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub environment_type: EnvironmentType,
    pub outer: Option<Rc<RefCell<Environment>>>,
    pub object: Option<Rc<RefCell<Object>>>,
    pub scope: Option<Rc<RefCell<Scope>>>,
    pub this_binding: Option<Value>,
    pub home_object: Option<Rc<RefCell<Object>>>,
    pub new_target: Option<Value>,
}

impl Environment {
    pub fn new(environment_type: EnvironmentType) -> Self {
        Self {
            environment_type,
            outer: None,
            object: None,
            scope: None,
            this_binding: None,
            home_object: None,
            new_target: None,
        }
    }
    
    pub fn with_outer(environment_type: EnvironmentType, outer: Rc<RefCell<Environment>>) -> Self {
        Self {
            environment_type,
            outer: Some(outer),
            object: None,
            scope: None,
            this_binding: None,
            home_object: None,
            new_target: None,
        }
    }
    
    pub fn new_global(global_object: Rc<RefCell<Object>>) -> Self {
        let scope = Rc::new(RefCell::new(Scope::global()));
        
        Self {
            environment_type: EnvironmentType::Global,
            outer: None,
            object: Some(global_object),
            scope: Some(scope),
            this_binding: None,
            home_object: None,
            new_target: None,
        }
    }
    
    pub fn new_object(object: Rc<RefCell<Object>>, outer: Rc<RefCell<Environment>>) -> Self {
        Self {
            environment_type: EnvironmentType::Object,
            outer: Some(outer),
            object: Some(object),
            scope: None,
            this_binding: None,
            home_object: None,
            new_target: None,
        }
    }
    
    pub fn new_function(
        function_object: Rc<RefCell<Object>>,
        this_binding: Value,
        outer: Rc<RefCell<Environment>>,
    ) -> Self {
        let scope = Rc::new(RefCell::new(Scope::function(outer.borrow().get_scope().unwrap())));
        
        Self {
            environment_type: EnvironmentType::Function,
            outer: Some(outer),
            object: Some(function_object),
            scope: Some(scope),
            this_binding: Some(this_binding),
            home_object: None,
            new_target: None,
        }
    }
    
    pub fn new_lexical(outer: Rc<RefCell<Environment>>) -> Self {
        let scope = Rc::new(RefCell::new(Scope::block(outer.borrow().get_scope().unwrap())));
        
        Self {
            environment_type: EnvironmentType::Lexical,
            outer: Some(outer),
            object: None,
            scope: Some(scope),
            this_binding: None,
            home_object: None,
            new_target: None,
        }
    }
    
    pub fn new_module(outer: Rc<RefCell<Environment>>) -> Self {
        let scope = Rc::new(RefCell::new(Scope::module(outer.borrow().get_scope().unwrap())));
        
        Self {
            environment_type: EnvironmentType::Module,
            outer: Some(outer),
            object: None,
            scope: Some(scope),
            this_binding: None,
            home_object: None,
            new_target: None,
        }
    }
    
    pub fn get_outer_environment(&self) -> Option<Rc<RefCell<Environment>>> {
        self.outer.clone()
    }
    
    pub fn set_outer_environment(&mut self, outer: Rc<RefCell<Environment>>) {
        self.outer = Some(outer);
    }
    
    pub fn get_object(&self) -> Option<Rc<RefCell<Object>>> {
        self.object.clone()
    }
    
    pub fn set_object(&mut self, object: Rc<RefCell<Object>>) {
        self.object = Some(object);
    }
    
    pub fn get_scope(&self) -> Option<Rc<RefCell<Scope>>> {
        self.scope.clone()
    }
    
    pub fn set_scope(&mut self, scope: Rc<RefCell<Scope>>) {
        self.scope = Some(scope);
    }
    
    pub fn get_this_binding(&self) -> Option<Value> {
        self.this_binding.clone()
    }
    
    pub fn set_this_binding(&mut self, this: Value) {
        self.this_binding = Some(this);
    }
    
    pub fn get_home_object(&self) -> Option<Rc<RefCell<Object>>> {
        self.home_object.clone()
    }
    
    pub fn set_home_object(&mut self, home_object: Rc<RefCell<Object>>) {
        self.home_object = Some(home_object);
    }
    
    pub fn get_new_target(&self) -> Option<Value> {
        self.new_target.clone()
    }
    
    pub fn set_new_target(&mut self, new_target: Value) {
        self.new_target = Some(new_target);
    }
    
    pub fn is_global_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Global)
    }
    
    pub fn is_object_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Object)
    }
    
    pub fn is_function_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Function)
    }
    
    pub fn is_lexical_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Lexical)
    }
    
    pub fn is_module_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Module)
    }
    
    pub fn get_binding_value(&self, name: &str) -> Result<Value, String> {
        match self.environment_type {
            EnvironmentType::Global | EnvironmentType::Function | EnvironmentType::Module => {
                if let Some(scope) = &self.scope {
                    scope.borrow().get_binding_value(name)
                } else {
                    Err(format!("Identifier '{}' is not defined", name))
                }
            }
            EnvironmentType::Object => {
                if let Some(object) = &self.object {
                    if let Some(value) = object.borrow().get_property(name) {
                        Ok(value)
                    } else {
                        Err(format!("Property '{}' is not defined", name))
                    }
                } else {
                    Err("Object environment has no object".to_string())
                }
            }
            EnvironmentType::Lexical => {
                if let Some(scope) = &self.scope {
                    scope.borrow().get_binding_value(name)
                } else {
                    Err(format!("Identifier '{}' is not defined", name))
                }
            }
        }
    }
    
    pub fn set_binding_value(&mut self, name: &str, value: Value) -> Result<(), String> {
        match self.environment_type {
            EnvironmentType::Global | EnvironmentType::Function | EnvironmentType::Module => {
                if let Some(scope) = &self.scope {
                    scope.borrow_mut().set_binding_value(name, value)
                } else {
                    Err(format!("Identifier '{}' is not defined", name))
                }
            }
            EnvironmentType::Object => {
                if let Some(object) = &self.object {
                    object.borrow_mut().set_property(name.to_string(), value);
                    Ok(())
                } else {
                    Err("Object environment has no object".to_string())
                }
            }
            EnvironmentType::Lexical => {
                if let Some(scope) = &self.scope {
                    scope.borrow_mut().set_binding_value(name, value)
                } else {
                    Err(format!("Identifier '{}' is not defined", name))
                }
            }
        }
    }
    
    pub fn has_binding(&self, name: &str) -> bool {
        match self.environment_type {
            EnvironmentType::Global | EnvironmentType::Function | EnvironmentType::Module => {
                if let Some(scope) = &self.scope {
                    scope.borrow().has_binding(name)
                } else {
                    false
                }
            }
            EnvironmentType::Object => {
                if let Some(object) = &self.object {
                    object.borrow().has_property(name)
                } else {
                    false
                }
            }
            EnvironmentType::Lexical => {
                if let Some(scope) = &self.scope {
                    scope.borrow().has_binding(name)
                } else {
                    false
                }
            }
        }
    }
    
    pub fn create_mutable_binding(&mut self, name: String, deletable: bool) -> Result<(), String> {
        match self.environment_type {
            EnvironmentType::Global | EnvironmentType::Function | EnvironmentType::Module | EnvironmentType::Lexical => {
                if let Some(scope) = &self.scope {
                    scope.borrow_mut().create_mutable_binding(name, deletable)
                } else {
                    Err("Environment has no scope".to_string())
                }
            }
            EnvironmentType::Object => {
                Err("Cannot create binding in object environment".to_string())
            }
        }
    }
    
    pub fn create_immutable_binding(&mut self, name: String, strict: bool) -> Result<(), String> {
        match self.environment_type {
            EnvironmentType::Global | EnvironmentType::Function | EnvironmentType::Module | EnvironmentType::Lexical => {
                if let Some(scope) = &self.scope {
                    scope.borrow_mut().create_immutable_binding(name, strict)
                } else {
                    Err("Environment has no scope".to_string())
                }
            }
            EnvironmentType::Object => {
                Err("Cannot create binding in object environment".to_string())
            }
        }
    }
    
    pub fn initialize_binding(&mut self, name: &str, value: Value) -> Result<(), String> {
        match self.environment_type {
            EnvironmentType::Global | EnvironmentType::Function | EnvironmentType::Module | EnvironmentType::Lexical => {
                if let Some(scope) = &self.scope {
                    scope.borrow_mut().initialize_binding(name, value)
                } else {
                    Err("Environment has no scope".to_string())
                }
            }
            EnvironmentType::Object => {
                if let Some(object) = &self.object {
                    object.borrow_mut().set_property(name.to_string(), value);
                    Ok(())
                } else {
                    Err("Object environment has no object".to_string())
                }
            }
        }
    }
    
    pub fn delete_binding(&mut self, name: &str) -> bool {
        match self.environment_type {
            EnvironmentType::Global | EnvironmentType::Function | EnvironmentType::Module | EnvironmentType::Lexical => {
                if let Some(scope) = &self.scope {
                    scope.borrow_mut().delete_binding(name)
                } else {
                    false
                }
            }
            EnvironmentType::Object => {
                if let Some(object) = &self.object {
                    object.borrow_mut().delete_property(name)
                } else {
                    false
                }
            }
        }
    }
    
    pub fn has_super_binding(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Function) && self.home_object.is_some()
    }
    
    pub fn get_super_base(&self) -> Option<Value> {
        if let Some(home_object) = &self.home_object {
            if let Some(prototype) = &home_object.borrow().prototype {
                Some(Value::Object(prototype.borrow().clone()))
            } else {
                Some(Value::Null)
            }
        } else {
            None
        }
    }
    
    pub fn get_this_binding(&self) -> Result<Value, String> {
        match self.environment_type {
            EnvironmentType::Global => {
                if let Some(object) = &self.object {
                    Ok(Value::Object(object.borrow().clone()))
                } else {
                    Err("Global environment has no object".to_string())
                }
            }
            EnvironmentType::Function => {
                if let Some(this_binding) = &self.this_binding {
                    Ok(this_binding.clone())
                } else {
                    Err("Function environment has no this binding".to_string())
                }
            }
            _ => Err("Environment does not provide a this binding".to_string()),
        }
    }
    
    pub fn resolve_binding(&self, name: &str) -> Result<Value, String> {
        if self.has_binding(name) {
            self.get_binding_value(name)
        } else if let Some(outer) = &self.outer {
            outer.borrow().resolve_binding(name)
        } else {
            Err(format!("Identifier '{}' is not defined", name))
        }
    }
    
    pub fn get_global_object(&self) -> Rc<RefCell<Object>> {
        if self.is_global_environment() {
            if let Some(object) = &self.object {
                object.clone()
            } else {
                panic!("Global environment has no object")
            }
        } else if let Some(outer) = &self.outer {
            outer.borrow().get_global_object()
        } else {
            panic!("No global environment found")
        }
    }
} 