use serde::{Deserialize, Serialize};
use v8_ast::Position;

/// Semantic analysis errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SemanticError {
    /// Variable is not declared
    UndeclaredVariable {
        name: String,
        position: Option<Position>,
    },
    
    /// Variable is declared but not initialized
    UninitializedVariable {
        name: String,
        position: Option<Position>,
    },
    
    /// Attempting to reassign a const variable
    ConstReassignment {
        name: String,
        position: Option<Position>,
    },
    
    /// Type mismatch in assignment or operation
    TypeMismatch {
        expected: String,
        found: String,
        position: Option<Position>,
    },
    
    /// Function is not declared
    UndeclaredFunction {
        name: String,
        position: Option<Position>,
    },
    
    /// Wrong number of arguments for function call
    WrongArgumentCount {
        function_name: String,
        expected: usize,
        found: usize,
        position: Option<Position>,
    },
    
    /// Invalid use of 'this' outside of method or constructor
    InvalidThisUsage {
        position: Option<Position>,
    },
    
    /// Duplicate variable declaration in same scope
    DuplicateDeclaration {
        name: String,
        position: Option<Position>,
    },
    
    /// Invalid operation on type
    InvalidOperation {
        operation: String,
        type_name: String,
        position: Option<Position>,
    },
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticError::UndeclaredVariable { name, position } => {
                write!(f, "Undeclared variable '{}'", name)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::UninitializedVariable { name, position } => {
                write!(f, "Variable '{}' is used before being initialized", name)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::ConstReassignment { name, position } => {
                write!(f, "Cannot reassign const variable '{}'", name)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::TypeMismatch { expected, found, position } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::UndeclaredFunction { name, position } => {
                write!(f, "Undeclared function '{}'", name)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::WrongArgumentCount { function_name, expected, found, position } => {
                write!(f, "Function '{}' expects {} arguments, but {} were provided", 
                       function_name, expected, found)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::InvalidThisUsage { position } => {
                write!(f, "Invalid use of 'this' outside of method or constructor")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::DuplicateDeclaration { name, position } => {
                write!(f, "Duplicate declaration of '{}'", name)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::InvalidOperation { operation, type_name, position } => {
                write!(f, "Invalid operation '{}' on type '{}'", operation, type_name)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SemanticError {} 