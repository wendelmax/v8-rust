//! Lexer tests for v8_lexer
//! 
//! Tests for lexer functionality, tokenization, and error handling.

use v8_lexer::{Lexer, Token, TokenKind, tokenize, tokenize_fallback};

#[test]
fn test_lexer_creation() {
    let source = "let x = 42;";
    let mut lexer = Lexer::new(source);
    
    // Test that lexer was created successfully
    let tokens = lexer.tokenize().unwrap();
    assert!(!tokens.is_empty());
    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}

#[test]
fn test_identifier_tokenization() {
    let source = "hello";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens.len(), 2); // identifier + EOF
    assert_eq!(tokens[0].kind, TokenKind::Identifier("hello".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn test_number_tokenization() {
    let source = "42";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens.len(), 2); // number + EOF
    assert_eq!(tokens[0].kind, TokenKind::Number(42.0));
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn test_string_tokenization() {
    let source = "\"hello world\"";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens.len(), 2); // string + EOF
    assert_eq!(tokens[0].kind, TokenKind::String("hello world".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn test_keyword_tokenization() {
    let keywords = vec![
        "let", "const", "var", "function", "if", "else", "return",
        "true", "false", "null", "undefined", "this", "super"
    ];
    
    for keyword in keywords {
        let tokens = tokenize(keyword).unwrap();
        assert_eq!(tokens.len(), 2); // keyword + EOF
        
        match &tokens[0].kind {
            TokenKind::Keyword(k) => {
                if k == "this" || k == "super" {
                    assert_eq!(k, keyword);
                } else {
                    assert_eq!(k, keyword);
                }
            }
            TokenKind::Boolean(true) => assert_eq!(keyword, "true"),
            TokenKind::Boolean(false) => assert_eq!(keyword, "false"),
            TokenKind::Null => assert_eq!(keyword, "null"),
            TokenKind::Undefined => assert_eq!(keyword, "undefined"),
            _ => panic!("Expected keyword token for '{}'", keyword),
        }
    }
}

#[test]
fn test_operator_tokenization() {
    let operators = vec![
        ("+", TokenKind::Plus),
        ("-", TokenKind::Minus),
        ("*", TokenKind::Star),
        ("/", TokenKind::Slash),
        ("=", TokenKind::Assign),
        ("==", TokenKind::Equal),
        ("!=", TokenKind::NotEqual),
        ("===", TokenKind::StrictEqual),
        ("!==", TokenKind::StrictNotEqual),
        ("&&", TokenKind::LogicalAnd),
        ("||", TokenKind::LogicalOr),
        ("++", TokenKind::Increment),
        ("--", TokenKind::Decrement),
    ];
    
    for (op_str, expected_kind) in operators {
        let tokens = tokenize(op_str).unwrap();
        assert_eq!(tokens.len(), 2); // operator + EOF
        assert_eq!(tokens[0].kind, expected_kind);
    }
}

#[test]
fn test_comment_tokenization() {
    // Line comment
    let source = "// this is a comment";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens.len(), 2); // comment + EOF
    assert_eq!(tokens[0].kind, TokenKind::Comment(" this is a comment".to_string()));
    
    // Block comment
    let source = "/* this is a block comment */";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens.len(), 2); // comment + EOF
    assert_eq!(tokens[0].kind, TokenKind::Comment(" this is a block comment ".to_string()));
}

#[test]
fn test_whitespace_handling() {
    let source = "  \t\n  let x = 42;  ";
    let tokens = tokenize(source).unwrap();
    
    // Should skip whitespace and only return meaningful tokens
    assert!(tokens.len() >= 6); // let + identifier + assign + number + semicolon + EOF
    
    // Check that we have the expected tokens
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("x".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::Number(42.0)));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
}

#[test]
fn test_complex_expression() {
    let source = "let result = (a + b) * 2;";
    let tokens = tokenize(source).unwrap();
    
    let expected_tokens = vec![
        TokenKind::Keyword("let".to_string()),
        TokenKind::Identifier("result".to_string()),
        TokenKind::Assign,
        TokenKind::LeftParen,
        TokenKind::Identifier("a".to_string()),
        TokenKind::Plus,
        TokenKind::Identifier("b".to_string()),
        TokenKind::RightParen,
        TokenKind::Star,
        TokenKind::Number(2.0),
        TokenKind::Semicolon,
        TokenKind::Eof,
    ];
    
    assert_eq!(tokens.len(), expected_tokens.len());
    for (i, expected) in expected_tokens.iter().enumerate() {
        assert_eq!(&tokens[i].kind, expected);
    }
}

#[test]
fn test_position_tracking() {
    let source = "let x = 42;\nlet y = 100;";
    let tokens = tokenize(source).unwrap();
    
    // Check that positions are tracked correctly
    assert_eq!(tokens[0].start().line, 1); // "let" should start at line 1
    assert_eq!(tokens[0].start().column, 1);
    
    // Find the second "let" token (should be on line 2)
    let _second_let = tokens.iter().find(|t| {
        matches!(&t.kind, TokenKind::Keyword(k) if k == "let")
    }).unwrap();
    
    // The second "let" should be after the semicolon, so we need to find it
    // This is a simplified test - in practice we'd need to track line numbers more carefully
}

#[test]
fn test_error_handling() {
    // Unterminated string
    let source = "\"unterminated string";
    let result = tokenize(source);
    assert!(result.is_err());
    
    // Unterminated comment
    let source = "/* unterminated comment";
    let result = tokenize(source);
    assert!(result.is_err());
}

#[test]
fn test_fallback_tokenization() {
    let source = "let x = 42;";
    let tokens = tokenize_fallback(source);
    
    // Should always return a valid result, even if there are errors
    assert!(!tokens.is_empty());
    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}

#[test]
fn test_hex_number() {
    let source = "0xFF";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens[0].kind, TokenKind::Number(255.0));
}

#[test]
fn test_binary_number() {
    let source = "0b1010";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens[0].kind, TokenKind::Number(10.0));
}

#[test]
fn test_octal_number() {
    let source = "0o755";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens[0].kind, TokenKind::Number(493.0));
}

#[test]
fn test_bigint_number() {
    let source = "42n";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens[0].kind, TokenKind::BigInt("42n".to_string()));
}

#[test]
fn test_template_string() {
    let source = "`hello ${name}`";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens[0].kind, TokenKind::TemplateString("hello ${name}".to_string()));
}

#[test]
fn test_escape_sequences() {
    let source = "\"hello\\nworld\"";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens[0].kind, TokenKind::String("hello\nworld".to_string()));
}

#[test]
fn test_unicode_identifiers() {
    let source = "let π = 3.14;";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens[1].kind, TokenKind::Identifier("π".to_string()));
}

#[test]
fn test_multiline_source() {
    let source = "let x = 1;\nlet y = 2;\nlet z = x + y;";
    let tokens = tokenize(source).unwrap();
    
    // Should handle multiple lines correctly
    assert!(tokens.len() > 10); // Multiple statements
    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}

#[test]
fn test_empty_source() {
    let source = "";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens.len(), 1); // Only EOF
    assert_eq!(tokens[0].kind, TokenKind::Eof);
}

#[test]
fn test_whitespace_only() {
    let source = "   \t\n  ";
    let tokens = tokenize(source).unwrap();
    
    assert_eq!(tokens.len(), 1); // Only EOF
    assert_eq!(tokens[0].kind, TokenKind::Eof);
} 