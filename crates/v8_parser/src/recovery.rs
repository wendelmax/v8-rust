//! Error recovery strategies for the parser

use crate::error::ParseError;
use v8_lexer::{Token, TokenKind};

/// Error recovery strategies
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// Skip tokens until a specific token is found
    SkipUntil(Vec<String>),
    
    /// Skip tokens until a statement boundary
    SkipUntilStatement,
    
    /// Skip tokens until a block boundary
    SkipUntilBlock,
    
    /// Skip tokens until a function boundary
    SkipUntilFunction,
    
    /// Skip tokens until a class boundary
    SkipUntilClass,
    
    /// Skip tokens until a module boundary
    SkipUntilModule,
    
    /// Insert a missing token
    InsertToken(String),
    
    /// Replace the current token
    ReplaceToken(String),
    
    /// Delete the current token
    DeleteToken,
    
    /// No recovery possible
    NoRecovery,
}

/// Error recovery context
#[derive(Debug, Clone)]
pub struct RecoveryContext {
    /// Current token that caused the error
    pub current_token: Option<Token>,
    
    /// Previous token
    pub previous_token: Option<Token>,
    
    /// Tokens that can be used for recovery
    pub recovery_tokens: Vec<String>,
    
    /// Current parsing context
    pub context: ParsingContext,
}

/// Parsing context for error recovery
#[derive(Debug, Clone, PartialEq)]
pub enum ParsingContext {
    /// Parsing at the top level
    TopLevel,
    
    /// Parsing inside a statement
    Statement,
    
    /// Parsing inside a block
    Block,
    
    /// Parsing inside a function
    Function,
    
    /// Parsing inside a class
    Class,
    
    /// Parsing inside a module
    Module,
    
    /// Parsing inside an expression
    Expression,
    
    /// Parsing inside a declaration
    Declaration,
}

impl RecoveryContext {
    /// Create a new recovery context
    pub fn new(
        current_token: Option<Token>,
        previous_token: Option<Token>,
        context: ParsingContext,
    ) -> Self {
        Self {
            current_token,
            previous_token,
            recovery_tokens: Vec::new(),
            context,
        }
    }

    /// Add recovery tokens
    pub fn with_recovery_tokens(mut self, tokens: Vec<String>) -> Self {
        self.recovery_tokens = tokens;
        self
    }

    /// Determine the best recovery strategy
    pub fn determine_strategy(&self) -> RecoveryStrategy {
        match &self.context {
            ParsingContext::TopLevel => {
                if let Some(token) = &self.current_token {
                    match token.kind {
                        TokenKind::Semicolon | TokenKind::RightBrace => {
                            RecoveryStrategy::SkipUntil(vec![";".to_string(), "}".to_string()])
                        }
                        _ => RecoveryStrategy::SkipUntilStatement,
                    }
                } else {
                    RecoveryStrategy::NoRecovery
                }
            }
            
            ParsingContext::Statement => {
                RecoveryStrategy::SkipUntil(vec![
                    ";".to_string(),
                    "}".to_string(),
                    ")".to_string(),
                ])
            }
            
            ParsingContext::Block => {
                RecoveryStrategy::SkipUntil(vec!["}".to_string()])
            }
            
            ParsingContext::Function => {
                RecoveryStrategy::SkipUntil(vec![
                    "}".to_string(),
                    ";".to_string(),
                ])
            }
            
            ParsingContext::Class => {
                RecoveryStrategy::SkipUntil(vec!["}".to_string()])
            }
            
            ParsingContext::Module => {
                RecoveryStrategy::SkipUntil(vec![
                    "}".to_string(),
                    "import".to_string(),
                    "export".to_string(),
                ])
            }
            
            ParsingContext::Expression => {
                RecoveryStrategy::SkipUntil(vec![
                    ";".to_string(),
                    ",".to_string(),
                    ")".to_string(),
                    "]".to_string(),
                    "}".to_string(),
                ])
            }
            
            ParsingContext::Declaration => {
                RecoveryStrategy::SkipUntil(vec![
                    ";".to_string(),
                    "}".to_string(),
                ])
            }
        }
    }

    /// Check if a token is a recovery token
    pub fn is_recovery_token(&self, token: &Token) -> bool {
        let token_str = format!("{:?}", token.kind);
        self.recovery_tokens.iter().any(|t| token_str.contains(t))
    }

    /// Get the current position
    pub fn current_position(&self) -> Option<v8_ast::Position> {
        self.current_token
            .as_ref()
            .map(|t| v8_ast::Position {
                line: t.start().line,
                column: t.start().column,
            })
    }
}

/// Error recovery manager
#[derive(Debug)]
pub struct ErrorRecovery {
    /// Maximum number of errors to recover from
    max_errors: usize,
    
    /// Current error count
    error_count: usize,
    
    /// Collected errors
    errors: Vec<ParseError>,
}

impl ErrorRecovery {
    /// Create a new error recovery manager
    pub fn new(max_errors: usize) -> Self {
        Self {
            max_errors,
            error_count: 0,
            errors: Vec::new(),
        }
    }

    /// Check if recovery is still possible
    pub fn can_recover(&self) -> bool {
        self.error_count < self.max_errors
    }

    /// Add an error
    pub fn add_error(&mut self, error: ParseError) {
        self.errors.push(error);
        self.error_count += 1;
    }

    /// Get all collected errors
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Clear all errors
    pub fn clear_errors(&mut self) {
        self.errors.clear();
        self.error_count = 0;
    }

    /// Get the error count
    pub fn error_count(&self) -> usize {
        self.error_count
    }

    /// Check if any errors occurred
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Default for ErrorRecovery {
    fn default() -> Self {
        Self::new(100) // Default to 100 errors max
    }
} 