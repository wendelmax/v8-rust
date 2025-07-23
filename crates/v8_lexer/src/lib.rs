//! Lexer for V8-Rust JavaScript engine
//! 
//! This crate provides a robust lexer for JavaScript/ECMAScript with:
//! - Precise position tracking (line/column)
//! - Comprehensive token support
//! - Error handling
//! - Unicode support

pub mod token;
pub mod lexer;
pub mod error;

pub use token::{Token, TokenKind, Position, Span};
pub use lexer::Lexer;
pub use error::LexerError;

/// Tokenize source code into a vector of tokens
pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

/// Tokenize source code into a vector of tokens (fallback version)
pub fn tokenize_fallback(source: &str) -> Vec<Token> {
    match tokenize(source) {
        Ok(tokens) => tokens,
        Err(_) => vec![Token::with_positions(TokenKind::Eof, 1, 1, 1, 1)],
    }
} 