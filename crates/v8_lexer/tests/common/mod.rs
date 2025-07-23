//! Common test utilities and fixtures for v8_lexer tests
//! 
//! This module provides shared test utilities, fixtures, and helper functions
//! that can be used across all test modules.

use v8_lexer::{Token, TokenKind, Position, Span};

/// Test fixture for common token creation
pub struct TokenFixture;

impl TokenFixture {
    /// Create a simple identifier token
    pub fn identifier(name: &str) -> Token {
        Token::new(
            TokenKind::Identifier(name.to_string()),
            Span::new(Position::new(1, 1), Position::new(1, name.len() + 1))
        )
    }
    
    /// Create a simple number token
    pub fn number(value: f64) -> Token {
        Token::new(
            TokenKind::Number(value),
            Span::new(Position::new(1, 1), Position::new(1, 3))
        )
    }
    
    /// Create a simple string token
    pub fn string(value: &str) -> Token {
        Token::new(
            TokenKind::String(value.to_string()),
            Span::new(Position::new(1, 1), Position::new(1, value.len() + 2))
        )
    }
    
    /// Create a simple keyword token
    pub fn keyword(value: &str) -> Token {
        Token::new(
            TokenKind::Keyword(value.to_string()),
            Span::new(Position::new(1, 1), Position::new(1, value.len() + 1))
        )
    }
    
    /// Create a simple operator token
    pub fn operator(kind: TokenKind) -> Token {
        Token::new(
            kind,
            Span::new(Position::new(1, 1), Position::new(1, 2))
        )
    }
}

/// Test fixture for common source code patterns
pub struct SourceFixture;

impl SourceFixture {
    /// Simple variable declaration
    pub fn variable_declaration() -> &'static str {
        "let x = 42;"
    }
    
    /// Simple function declaration
    pub fn function_declaration() -> &'static str {
        "function add(a, b) { return a + b; }"
    }
    
    /// Simple if statement
    pub fn if_statement() -> &'static str {
        "if (x > 0) { console.log('positive'); }"
    }
    
    /// Simple for loop
    pub fn for_loop() -> &'static str {
        "for (let i = 0; i < 10; i++) { console.log(i); }"
    }
    
    /// Complex expression
    pub fn complex_expression() -> &'static str {
        "(a + b) * (c - d) / (e % f)"
    }
    
    /// Object literal
    pub fn object_literal() -> &'static str {
        "{ name: 'John', age: 30, isActive: true }"
    }
    
    /// Array literal
    pub fn array_literal() -> &'static str {
        "[1, 2, 3, 'hello', true]"
    }
    
    /// Template literal
    pub fn template_literal() -> &'static str {
        "`Hello, ${name}!`"
    }
    
    /// Arrow function
    pub fn arrow_function() -> &'static str {
        "(a, b) => a + b"
    }
    
    /// Class declaration
    pub fn class_declaration() -> &'static str {
        "class Person { constructor(name) { this.name = name; } }"
    }
}

/// Test fixture for error conditions
pub struct ErrorFixture;

impl ErrorFixture {
    /// Unterminated string
    pub fn unterminated_string() -> &'static str {
        "\"hello world"
    }
    
    /// Unterminated comment
    pub fn unterminated_comment() -> &'static str {
        "/* hello world"
    }
    
    /// Invalid number
    pub fn invalid_number() -> &'static str {
        "0xGG"
    }
    
    /// Invalid identifier
    pub fn invalid_identifier() -> &'static str {
        "let 123abc = 42;"
    }
    
    /// Invalid operator
    pub fn invalid_operator() -> &'static str {
        "let x = 42 @ 10;"
    }
}

/// Helper functions for test assertions
pub mod assertions {
    use super::*;
    use v8_lexer::tokenize;
    
    /// Assert that a source tokenizes successfully
    pub fn assert_tokenizes_successfully(source: &str) {
        let result = tokenize(source);
        assert!(result.is_ok(), "Failed to tokenize: {:?}", result.err());
    }
    
    /// Assert that a source fails to tokenize with a specific error
    pub fn assert_tokenizes_with_error<T>(source: &str, expected_error: T)
    where
        T: PartialEq + std::fmt::Debug,
    {
        let result = tokenize(source);
        assert!(result.is_err());
        // Note: This would need to be implemented based on the actual error type
    }
    
    /// Assert that tokens contain a specific token kind
    pub fn assert_contains_token(tokens: &[Token], expected_kind: &TokenKind) {
        let found = tokens.iter().any(|t| &t.kind == expected_kind);
        assert!(found, "Expected token {:?} not found in tokens", expected_kind);
    }
    
    /// Assert that tokens contain a specific identifier
    pub fn assert_contains_identifier(tokens: &[Token], expected_name: &str) {
        let expected_kind = TokenKind::Identifier(expected_name.to_string());
        assert_contains_token(tokens, &expected_kind);
    }
    
    /// Assert that tokens contain a specific keyword
    pub fn assert_contains_keyword(tokens: &[Token], expected_keyword: &str) {
        let expected_kind = TokenKind::Keyword(expected_keyword.to_string());
        assert_contains_token(tokens, &expected_kind);
    }
    
    /// Assert that tokens contain a specific number
    pub fn assert_contains_number(tokens: &[Token], expected_value: f64) {
        let expected_kind = TokenKind::Number(expected_value);
        assert_contains_token(tokens, &expected_kind);
    }
    
    /// Assert that tokens contain a specific string
    pub fn assert_contains_string(tokens: &[Token], expected_value: &str) {
        let expected_kind = TokenKind::String(expected_value.to_string());
        assert_contains_token(tokens, &expected_kind);
    }
}

/// Performance testing utilities
pub mod performance {
    use std::time::Instant;
    
    /// Measure execution time of a function
    pub fn measure_time<F, R>(func: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = func();
        let duration = start.elapsed();
        (result, duration)
    }
    
    /// Assert that execution time is within acceptable bounds
    pub fn assert_performance_acceptable<F>(func: F, max_duration_ms: u128)
    where
        F: FnOnce(),
    {
        let (_, duration) = measure_time(func);
        assert!(
            duration.as_millis() < max_duration_ms,
            "Performance test failed: took {:?}, expected less than {}ms",
            duration,
            max_duration_ms
        );
    }
}

/// Memory testing utilities
pub mod memory {
    use std::mem;
    
    /// Estimate memory usage of a data structure
    pub fn estimate_memory_usage<T>(items: &[T]) -> usize {
        items.len() * mem::size_of::<T>()
    }
    
    /// Assert that memory usage is within acceptable bounds
    pub fn assert_memory_usage_acceptable<T>(items: &[T], max_bytes: usize) {
        let usage = estimate_memory_usage(items);
        assert!(
            usage < max_bytes,
            "Memory usage too high: {} bytes, expected less than {} bytes",
            usage,
            max_bytes
        );
    }
} 