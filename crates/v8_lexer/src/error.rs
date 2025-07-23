//! Error types for the V8-Rust lexer

use thiserror::Error;

/// Errors that can occur during lexing
#[derive(Debug, Error, Clone, PartialEq)]
pub enum LexerError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    
    #[error("Invalid number: {0}")]
    InvalidNumber(String),
    
    #[error("Unterminated string")]
    UnterminatedString,
    
    #[error("Unterminated template string")]
    UnterminatedTemplateString,
    
    #[error("Unterminated comment")]
    UnterminatedComment,
    
    #[error("Invalid escape sequence: {0}")]
    InvalidEscapeSequence(String),
    
    #[error("Invalid Unicode escape: {0}")]
    InvalidUnicodeEscape(String),
    
    #[error("Invalid hex escape: {0}")]
    InvalidHexEscape(String),
    
    #[error("Invalid octal escape: {0}")]
    InvalidOctalEscape(String),
    
    #[error("Invalid binary literal: {0}")]
    InvalidBinaryLiteral(String),
    
    #[error("Invalid octal literal: {0}")]
    InvalidOctalLiteral(String),
    
    #[error("Invalid hex literal: {0}")]
    InvalidHexLiteral(String),
    
    #[error("Invalid BigInt literal: {0}")]
    InvalidBigIntLiteral(String),
    
    #[error("Invalid regex literal: {0}")]
    InvalidRegexLiteral(String),
    
    #[error("Invalid regex flags: {0}")]
    InvalidRegexFlags(String),
    
    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),
    
    #[error("Invalid keyword: {0}")]
    InvalidKeyword(String),
    
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),
    
    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),
    
    #[error("Invalid comment: {0}")]
    InvalidComment(String),
    
    #[error("Invalid whitespace: {0}")]
    InvalidWhitespace(String),
    
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    
    #[error("End of input reached unexpectedly")]
    UnexpectedEndOfInput,
    
    #[error("Internal lexer error: {0}")]
    InternalError(String),
} 