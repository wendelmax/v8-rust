//! Token tests for v8_lexer
//! 
//! Tests for token creation, validation, and utility methods.

use v8_lexer::{Token, TokenKind, Position, Span};

#[test]
fn test_token_creation() {
    let token = Token::new(
        TokenKind::Identifier("test".to_string()),
        Span::new(Position::new(1, 1), Position::new(1, 5))
    );
    
    assert_eq!(token.start(), Position::new(1, 1));
    assert_eq!(token.end(), Position::new(1, 5));
    assert!(token.is_identifier());
}

#[test]
fn test_token_with_positions() {
    let token = Token::with_positions(
        TokenKind::Number(42.0),
        1, 1, 1, 3
    );
    
    assert_eq!(token.start(), Position::new(1, 1));
    assert_eq!(token.end(), Position::new(1, 3));
    assert!(token.is_literal());
}

#[test]
fn test_identifier_token() {
    let token = Token::new(
        TokenKind::Identifier("variable".to_string()),
        Span::new(Position::new(1, 1), Position::new(1, 9))
    );
    
    assert!(token.is_identifier());
    assert!(!token.is_keyword());
    assert!(!token.is_literal());
    assert!(!token.is_operator());
}

#[test]
fn test_keyword_token() {
    let token = Token::new(
        TokenKind::Keyword("let".to_string()),
        Span::new(Position::new(1, 1), Position::new(1, 4))
    );
    
    assert!(token.is_keyword());
    assert!(!token.is_identifier());
    assert!(!token.is_literal());
    assert!(!token.is_operator());
}

#[test]
fn test_literal_tokens() {
    // Number literal
    let number_token = Token::new(
        TokenKind::Number(3.14),
        Span::new(Position::new(1, 1), Position::new(1, 4))
    );
    assert!(number_token.is_literal());
    
    // String literal
    let string_token = Token::new(
        TokenKind::String("hello".to_string()),
        Span::new(Position::new(1, 1), Position::new(1, 6))
    );
    assert!(string_token.is_literal());
    
    // Boolean literal
    let bool_token = Token::new(
        TokenKind::Boolean(true),
        Span::new(Position::new(1, 1), Position::new(1, 5))
    );
    assert!(bool_token.is_literal());
    
    // Null literal
    let null_token = Token::new(
        TokenKind::Null,
        Span::new(Position::new(1, 1), Position::new(1, 5))
    );
    assert!(null_token.is_literal());
    
    // Undefined literal
    let undefined_token = Token::new(
        TokenKind::Undefined,
        Span::new(Position::new(1, 1), Position::new(1, 10))
    );
    assert!(undefined_token.is_literal());
}

#[test]
fn test_operator_tokens() {
    let operators = vec![
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Slash,
        TokenKind::Equal,
        TokenKind::NotEqual,
        TokenKind::LogicalAnd,
        TokenKind::LogicalOr,
    ];
    
    for op in operators {
        let token = Token::new(
            op.clone(),
            Span::new(Position::new(1, 1), Position::new(1, 2))
        );
        assert!(token.is_operator(), "Token {:?} should be an operator", op);
    }
}

#[test]
fn test_position_creation() {
    let pos = Position::new(5, 10);
    assert_eq!(pos.line, 5);
    assert_eq!(pos.column, 10);
}

#[test]
fn test_span_creation() {
    let start = Position::new(1, 1);
    let end = Position::new(1, 10);
    let span = Span::new(start, end);
    
    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
}

#[test]
fn test_span_from_positions() {
    let span = Span::from_positions(2, 5, 2, 15);
    
    assert_eq!(span.start.line, 2);
    assert_eq!(span.start.column, 5);
    assert_eq!(span.end.line, 2);
    assert_eq!(span.end.column, 15);
}

#[test]
fn test_token_kind_variants() {
    // Test that all token kinds can be created
    let _identifier = TokenKind::Identifier("test".to_string());
    let _number = TokenKind::Number(42.0);
    let _string = TokenKind::String("hello".to_string());
    let _boolean = TokenKind::Boolean(true);
    let _null = TokenKind::Null;
    let _undefined = TokenKind::Undefined;
    let _keyword = TokenKind::Keyword("let".to_string());
    let _symbol = TokenKind::Symbol("+".to_string());
    let _comment = TokenKind::Comment("test comment".to_string());
    let _whitespace = TokenKind::Whitespace;
    let _eof = TokenKind::Eof;
    
    // Test specific tokens
    let _left_paren = TokenKind::LeftParen;
    let _right_paren = TokenKind::RightParen;
    let _plus = TokenKind::Plus;
    let _minus = TokenKind::Minus;
    let _assign = TokenKind::Assign;
    let _equal = TokenKind::Equal;
    let _logical_and = TokenKind::LogicalAnd;
    let _logical_or = TokenKind::LogicalOr;
    let _increment = TokenKind::Increment;
    let _decrement = TokenKind::Decrement;
}

#[test]
fn test_token_serialization() {
    use serde_json;
    
    let token = Token::new(
        TokenKind::Identifier("test".to_string()),
        Span::new(Position::new(1, 1), Position::new(1, 5))
    );
    
    // Test serialization
    let serialized = serde_json::to_string(&token).unwrap();
    assert!(!serialized.is_empty());
    
    // Test deserialization
    let deserialized: Token = serde_json::from_str(&serialized).unwrap();
    assert_eq!(token, deserialized);
}

#[test]
fn test_token_clone() {
    let original = Token::new(
        TokenKind::Number(42.0),
        Span::new(Position::new(1, 1), Position::new(1, 3))
    );
    
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_token_debug() {
    let token = Token::new(
        TokenKind::String("test".to_string()),
        Span::new(Position::new(1, 1), Position::new(1, 5))
    );
    
    let debug_str = format!("{:?}", token);
    assert!(debug_str.contains("String"));
    assert!(debug_str.contains("test"));
} 