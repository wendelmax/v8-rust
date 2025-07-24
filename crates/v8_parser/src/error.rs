//! Error handling for the parser

use thiserror::Error;
use v8_ast::{Position, Span};
use v8_lexer::Token;

/// Result type for parsing operations
pub type ParseResult<T> = Result<T, ParseError>;

/// Errors that can occur during parsing
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error("Unexpected token '{token}' at {position}")]
    UnexpectedToken {
        token: String,
        position: Position,
        expected: Option<String>,
    },

    #[error("Unexpected end of input at {position}")]
    UnexpectedEndOfInput {
        position: Position,
        expected: Option<String>,
    },

    #[error("Invalid syntax: {message} at {position}")]
    InvalidSyntax {
        message: String,
        position: Position,
    },

    #[error("Invalid expression: {message} at {position}")]
    InvalidExpression {
        message: String,
        position: Position,
    },

    #[error("Invalid statement: {message} at {position}")]
    InvalidStatement {
        message: String,
        position: Position,
    },

    #[error("Invalid declaration: {message} at {position}")]
    InvalidDeclaration {
        message: String,
        position: Position,
    },

    #[error("Invalid function: {message} at {position}")]
    InvalidFunction {
        message: String,
        position: Position,
    },

    #[error("Invalid class: {message} at {position}")]
    InvalidClass {
        message: String,
        position: Position,
    },

    #[error("Invalid module: {message} at {position}")]
    InvalidModule {
        message: String,
        position: Position,
    },

    #[error("Lexer error: {message} at {position}")]
    LexerError {
        message: String,
        position: Position,
    },

    #[error("Internal parser error: {message}")]
    InternalError {
        message: String,
    },
}

impl ParseError {
    /// Create an unexpected token error
    pub fn unexpected_token(token: &Token, expected: Option<&str>) -> Self {
        let position = Position {
            line: token.start().line,
            column: token.start().column,
        };
        ParseError::UnexpectedToken {
            token: format!("{:?}", token.kind),
            position,
            expected: expected.map(|s| s.to_string()),
        }
    }

    /// Create an unexpected end of input error
    pub fn unexpected_end_of_input(expected: Option<&str>) -> Self {
        ParseError::UnexpectedEndOfInput {
            position: Position::new(1, 1),
            expected: expected.map(|s| s.to_string()),
        }
    }

    /// Create an invalid syntax error
    pub fn invalid_syntax(message: &str, position: Position) -> Self {
        ParseError::InvalidSyntax {
            message: message.to_string(),
            position,
        }
    }

    /// Create an invalid expression error
    pub fn invalid_expression(message: &str, position: Position) -> Self {
        ParseError::InvalidExpression {
            message: message.to_string(),
            position,
        }
    }

    /// Create an invalid statement error
    pub fn invalid_statement(message: &str, position: Position) -> Self {
        ParseError::InvalidStatement {
            message: message.to_string(),
            position,
        }
    }

    /// Create an invalid declaration error
    pub fn invalid_declaration(message: &str, position: Position) -> Self {
        ParseError::InvalidDeclaration {
            message: message.to_string(),
            position,
        }
    }

    /// Create an invalid function error
    pub fn invalid_function(message: &str, position: Position) -> Self {
        ParseError::InvalidFunction {
            message: message.to_string(),
            position,
        }
    }

    /// Create an invalid class error
    pub fn invalid_class(message: &str, position: Position) -> Self {
        ParseError::InvalidClass {
            message: message.to_string(),
            position,
        }
    }

    /// Create an invalid module error
    pub fn invalid_module(message: &str, position: Position) -> Self {
        ParseError::InvalidModule {
            message: message.to_string(),
            position,
        }
    }

    /// Create a lexer error
    pub fn lexer_error(message: &str, position: Position) -> Self {
        ParseError::LexerError {
            message: message.to_string(),
            position,
        }
    }

    /// Create an internal error
    pub fn internal_error(message: &str) -> Self {
        ParseError::InternalError {
            message: message.to_string(),
        }
    }

    /// Get the position where the error occurred
    pub fn position(&self) -> Option<Position> {
        match self {
            ParseError::UnexpectedToken { position, .. } => Some(*position),
            ParseError::UnexpectedEndOfInput { position, .. } => Some(*position),
            ParseError::InvalidSyntax { position, .. } => Some(*position),
            ParseError::InvalidExpression { position, .. } => Some(*position),
            ParseError::InvalidStatement { position, .. } => Some(*position),
            ParseError::InvalidDeclaration { position, .. } => Some(*position),
            ParseError::InvalidFunction { position, .. } => Some(*position),
            ParseError::InvalidClass { position, .. } => Some(*position),
            ParseError::InvalidModule { position, .. } => Some(*position),
            ParseError::LexerError { position, .. } => Some(*position),
            ParseError::InternalError { .. } => None,
        }
    }

    /// Get the span where the error occurred
    pub fn span(&self) -> Option<Span> {
        self.position().map(|pos| Span::new(pos, pos))
    }
} 