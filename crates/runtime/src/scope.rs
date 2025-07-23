// Scope system for ECMAScript

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::value::Value;

#[derive(Debug, Clone)]
pub enum BindingType {
    Lexical,
    Variable,
    Function,
    Parameter,
    Catch,
    Module,
}

#[derive(Debug, Clone)]
pub struct Binding {
    pub value: Value,
    pub binding_type: BindingType,
    pub mutable: bool,
    pub deletable: bool,
    pub strict: bool,
}

impl Binding {
    pub fn new(value: Value, binding_type: BindingType) -> Self {
        Self {
            value,
            binding_type,
            mutable: true,
            deletable: true,
            strict: false,
        }
    }
    
    pub fn lexical(value: Value) -> Self {
        Self {
            value,
            binding_type: BindingType::Lexical,
            mutable: false,
            deletable: false,
            strict: true,
        }
    }
    
    pub fn variable(value: Value) -> Self {
        Self {
            value,
            binding_type: BindingType::Variable,
            mutable: true,
            deletable: true,
            strict: false,
        }
    }
    
    pub fn function(value: Value) -> Self {
        Self {
            value,
            binding_type: BindingType::Function,
            mutable: true,
            deletable: true,
            strict: false,
        }
    }
    
    pub fn parameter(value: Value) -> Self {
        Self {
            value,
            binding_type: BindingType::Parameter,
            mutable: true,
            deletable: false,
            strict: false,
        }
    }
    
    pub fn catch(value: Value) -> Self {
        Self {
            value,
            binding_type: BindingType::Catch,
            mutable: true,
            deletable: false,
            strict: false,
        }
    }
    
    pub fn module(value: Value) -> Self {
        Self {
            value,
            binding_type: BindingType::Module,
            mutable: false,
            deletable: false,
            strict: true,
        }
    }
    
    pub fn get_value(&self) -> &Value {
        &self.value
    }
    
    pub fn set_value(&mut self, value: Value) -> Result<(), String> {
        if self.mutable {
            self.value = value;
            Ok(())
        } else {
            Err("Cannot assign to immutable binding".to_string())
        }
    }
    
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
    
    pub fn is_deletable(&self) -> bool {
        self.deletable
    }
    
    pub fn is_strict(&self) -> bool {
        self.strict
    }
    
    pub fn get_binding_type(&self) -> &BindingType {
        &self.binding_type
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub bindings: HashMap<String, Binding>,
    pub outer: Option<Rc<RefCell<Scope>>>,
    pub scope_type: ScopeType,
    pub strict: bool,
}

#[derive(Debug, Clone)]
pub enum ScopeType {
    Global,
    Function,
    Block,
    Catch,
    With,
    Module,
    Eval,
}

impl Scope {
    pub fn new(scope_type: ScopeType) -> Self {
        Self {
            bindings: HashMap::new(),
            outer: None,
            scope_type,
            strict: false,
        }
    }
    
    pub fn with_outer(scope_type: ScopeType, outer: Rc<RefCell<Scope>>) -> Self {
        Self {
            bindings: HashMap::new(),
            outer: Some(outer),
            scope_type,
            strict: false,
        }
    }
    
    pub fn global() -> Self {
        Self::new(ScopeType::Global)
    }
    
    pub fn function(outer: Rc<RefCell<Scope>>) -> Self {
        Self::with_outer(ScopeType::Function, outer)
    }
    
    pub fn block(outer: Rc<RefCell<Scope>>) -> Self {
        Self::with_outer(ScopeType::Block, outer)
    }
    
    pub fn catch(outer: Rc<RefCell<Scope>>) -> Self {
        Self::with_outer(ScopeType::Catch, outer)
    }
    
    pub fn with(outer: Rc<RefCell<Scope>>) -> Self {
        Self::with_outer(ScopeType::With, outer)
    }
    
    pub fn module(outer: Rc<RefCell<Scope>>) -> Self {
        Self::with_outer(ScopeType::Module, outer)
    }
    
    pub fn eval(outer: Rc<RefCell<Scope>>) -> Self {
        Self::with_outer(ScopeType::Eval, outer)
    }
    
    pub fn bind_identifier(&mut self, name: String, binding: Binding) -> Result<(), String> {
        if self.bindings.contains_key(&name) {
            return Err(format!("Identifier '{}' has already been declared", name));
        }
        
        self.bindings.insert(name, binding);
        Ok(())
    }
    
    pub fn get_binding(&self, name: &str) -> Option<&Binding> {
        self.bindings.get(name)
    }
    
    pub fn get_binding_mut(&mut self, name: &str) -> Option<&mut Binding> {
        self.bindings.get_mut(name)
    }
    
    pub fn set_binding(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(binding) = self.bindings.get_mut(name) {
            binding.set_value(value)
        } else {
            Err(format!("Identifier '{}' is not defined", name))
        }
    }
    
    pub fn has_binding(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }
    
    pub fn delete_binding(&mut self, name: &str) -> bool {
        if let Some(binding) = self.bindings.get(name) {
            if binding.is_deletable() {
                self.bindings.remove(name);
                true
            } else {
                false
            }
        } else {
            true
        }
    }
    
    pub fn resolve_binding(&self, name: &str) -> Option<&Binding> {
        if let Some(binding) = self.bindings.get(name) {
            Some(binding)
        } else if let Some(outer) = &self.outer {
            outer.borrow().resolve_binding(name)
        } else {
            None
        }
    }
    
    pub fn resolve_binding_mut(&mut self, name: &str) -> Option<&mut Binding> {
        if let Some(binding) = self.bindings.get_mut(name) {
            Some(binding)
        } else if let Some(outer) = &self.outer {
            outer.borrow_mut().resolve_binding_mut(name)
        } else {
            None
        }
    }
    
    pub fn get_binding_value(&self, name: &str) -> Result<Value, String> {
        if let Some(binding) = self.resolve_binding(name) {
            Ok(binding.get_value().clone())
        } else {
            Err(format!("Identifier '{}' is not defined", name))
        }
    }
    
    pub fn set_binding_value(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(binding) = self.resolve_binding_mut(name) {
            binding.set_value(value)
        } else {
            Err(format!("Identifier '{}' is not defined", name))
        }
    }
    
    pub fn create_mutable_binding(&mut self, name: String, deletable: bool) -> Result<(), String> {
        let binding = Binding {
            value: Value::Undefined,
            binding_type: BindingType::Variable,
            mutable: true,
            deletable,
            strict: self.strict,
        };
        
        self.bind_identifier(name, binding)
    }
    
    pub fn create_immutable_binding(&mut self, name: String, strict: bool) -> Result<(), String> {
        let binding = Binding {
            value: Value::Undefined,
            binding_type: BindingType::Lexical,
            mutable: false,
            deletable: false,
            strict,
        };
        
        self.bind_identifier(name, binding)
    }
    
    pub fn initialize_binding(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(binding) = self.bindings.get_mut(name) {
            binding.set_value(value)
        } else {
            Err(format!("Identifier '{}' is not defined", name))
        }
    }
    
    pub fn get_outer_environment(&self) -> Option<Rc<RefCell<Scope>>> {
        self.outer.clone()
    }
    
    pub fn set_outer_environment(&mut self, outer: Rc<RefCell<Scope>>) {
        self.outer = Some(outer);
    }
    
    pub fn get_scope_type(&self) -> &ScopeType {
        &self.scope_type
    }
    
    pub fn is_strict(&self) -> bool {
        self.strict
    }
    
    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }
    
    pub fn get_binding_names(&self) -> Vec<String> {
        self.bindings.keys().cloned().collect()
    }
    
    pub fn get_all_binding_names(&self) -> Vec<String> {
        let mut names = self.get_binding_names();
        
        if let Some(outer) = &self.outer {
            let mut outer_names = outer.borrow().get_all_binding_names();
            names.append(&mut outer_names);
        }
        
        names
    }
    
    pub fn is_global_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Global)
    }
    
    pub fn is_function_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Function)
    }
    
    pub fn is_block_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Block)
    }
    
    pub fn is_catch_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Catch)
    }
    
    pub fn is_with_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::With)
    }
    
    pub fn is_module_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Module)
    }
    
    pub fn is_eval_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Eval)
    }
} 