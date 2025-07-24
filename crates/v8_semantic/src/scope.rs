use crate::types::Type;
use std::collections::HashMap;

/// Represents a scope in the program
#[derive(Debug, Clone)]
pub struct Scope {
    /// Variables declared in this scope
    variables: HashMap<String, VariableInfo>,
    
    /// Functions declared in this scope
    functions: HashMap<String, FunctionInfo>,
    
    /// Parent scope (if any)
    parent: Option<Box<Scope>>,
    
    /// Scope type
    scope_type: ScopeType,
}

/// Information about a variable
#[derive(Debug, Clone)]
pub struct VariableInfo {
    /// Variable name
    pub name: String,
    
    /// Variable type
    pub type_info: Type,
    
    /// Whether the variable is mutable (let/var vs const)
    pub mutable: bool,
    
    /// Whether the variable is initialized
    pub initialized: bool,
    
    /// Line number where declared
    pub line: usize,
}

/// Information about a function
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    /// Function name
    pub name: String,
    
    /// Parameter types
    pub param_types: Vec<Type>,
    
    /// Return type
    pub return_type: Type,
    
    /// Whether it's a method
    pub is_method: bool,
    
    /// Line number where declared
    pub line: usize,
}

/// Type of scope
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    /// Global scope
    Global,
    
    /// Function scope
    Function,
    
    /// Block scope (if, while, for, etc.)
    Block,
    
    /// Class scope
    Class,
    
    /// Module scope
    Module,
}

impl Scope {
    /// Create a new global scope
    pub fn new_global() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
            scope_type: ScopeType::Global,
        }
    }
    
    /// Create a new child scope
    pub fn new_child(parent: Scope, scope_type: ScopeType) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
            scope_type,
        }
    }
    
    /// Declare a variable in this scope
    pub fn declare_variable(&mut self, name: &str, type_info: Type, mutable: bool, line: usize) -> bool {
        if self.variables.contains_key(name) {
            false // Variable already declared in this scope
        } else {
            self.variables.insert(name.to_string(), VariableInfo {
                name: name.to_string(),
                type_info,
                mutable,
                initialized: false,
                line,
            });
            true
        }
    }
    
    /// Initialize a variable (mark as assigned)
    pub fn initialize_variable(&mut self, name: &str) -> bool {
        if let Some(var) = self.variables.get_mut(name) {
            var.initialized = true;
            true
        } else {
            false
        }
    }
    
    /// Get variable information
    pub fn get_variable(&self, name: &str) -> Option<&VariableInfo> {
        self.variables.get(name).or_else(|| {
            self.parent.as_ref().and_then(|p| p.get_variable(name))
        })
    }
    
    /// Check if a variable is declared in this scope or any parent scope
    pub fn is_variable_declared(&self, name: &str) -> bool {
        self.variables.contains_key(name) || 
        self.parent.as_ref().map_or(false, |p| p.is_variable_declared(name))
    }
    
    /// Check if a variable is declared only in this scope (not in parent)
    pub fn is_variable_declared_in_current_scope(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
    
    /// Declare a function in this scope
    pub fn declare_function(&mut self, name: &str, param_types: Vec<Type>, return_type: Type, is_method: bool, line: usize) -> bool {
        if self.functions.contains_key(name) {
            false // Function already declared in this scope
        } else {
            self.functions.insert(name.to_string(), FunctionInfo {
                name: name.to_string(),
                param_types,
                return_type,
                is_method,
                line,
            });
            true
        }
    }
    
    /// Get function information
    pub fn get_function(&self, name: &str) -> Option<&FunctionInfo> {
        self.functions.get(name).or_else(|| {
            self.parent.as_ref().and_then(|p| p.get_function(name))
        })
    }
    
    /// Check if a function is declared
    pub fn is_function_declared(&self, name: &str) -> bool {
        self.functions.contains_key(name) || 
        self.parent.as_ref().map_or(false, |p| p.is_function_declared(name))
    }
    
    /// Get the scope type
    pub fn scope_type(&self) -> &ScopeType {
        &self.scope_type
    }
    
    /// Get all variables in this scope (not including parent scopes)
    pub fn get_local_variables(&self) -> &HashMap<String, VariableInfo> {
        &self.variables
    }
    
    /// Get all functions in this scope (not including parent scopes)
    pub fn get_local_functions(&self) -> &HashMap<String, FunctionInfo> {
        &self.functions
    }
    
    /// Check if this is a function scope
    pub fn is_function_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Function)
    }
    
    /// Check if this is a block scope
    pub fn is_block_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Block)
    }
    
    /// Check if this is the global scope
    pub fn is_global_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Global)
    }
} 