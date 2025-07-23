//! Integration tests for the entire v8-rust project
//! 
//! These tests verify that all components work together correctly.

use v8_lexer::tokenize;
use v8_ast::{AstNode, Program, Statement, Expression, Declaration};
use v8_parser::parse;

#[test]
fn test_full_pipeline_simple() {
    let source = "let x = 42;";
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    assert_eq!(tokens.last().unwrap().kind, v8_lexer::TokenKind::Eof);
    
    // Step 2: Parsing (when implemented)
    // let ast = parse(&tokens).unwrap();
    // assert!(matches!(ast, AstNode::Program(_)));
    
    println!("Full pipeline test passed for: {}", source);
}

#[test]
fn test_full_pipeline_function() {
    let source = "function add(a, b) { return a + b; }";
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify we have the expected tokens
    let token_kinds: Vec<&v8_lexer::TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Keyword("function".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Identifier("add".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Keyword("return".to_string())));
    
    println!("Function pipeline test passed for: {}", source);
}

#[test]
fn test_full_pipeline_complex() {
    let source = r#"
        function fibonacci(n) {
            if (n <= 1) {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        let result = fibonacci(10);
        console.log(`Fibonacci of 10 is ${result}`);
    "#;
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify we have the expected tokens
    let token_kinds: Vec<&v8_lexer::TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Keyword("function".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Keyword("if".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Keyword("return".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::TemplateString("Fibonacci of 10 is ${result}".to_string())));
    
    println!("Complex pipeline test passed");
}

#[test]
fn test_error_propagation() {
    let source = "\"unterminated string";
    
    // Step 1: Lexical analysis should fail
    let result = tokenize(source);
    assert!(result.is_err());
    
    // Step 2: Parsing should also fail (when implemented)
    // let result = parse(&[]);
    // assert!(result.is_err());
    
    println!("Error propagation test passed");
}

#[test]
fn test_unicode_support() {
    let source = "let π = 3.14159; let 你好 = 'world'; let 🚀 = 'rocket';";
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify Unicode identifiers are handled correctly
    let token_kinds: Vec<&v8_lexer::TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Identifier("π".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Identifier("你好".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Identifier("🚀".to_string())));
    
    println!("Unicode support test passed");
}

#[test]
fn test_template_literals() {
    let source = "let greeting = `Hello, ${name}!`;";
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify template literal is handled correctly
    let token_kinds: Vec<&v8_lexer::TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::TemplateString("Hello, ${name}!".to_string())));
    
    println!("Template literals test passed");
}

#[test]
fn test_numbers() {
    let source = "let int = 42; let float = 3.14; let hex = 0xFF; let binary = 0b1010; let bigint = 42n;";
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify different number formats are handled correctly
    let token_kinds: Vec<&v8_lexer::TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Number(42.0)));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Number(3.14)));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Number(255.0))); // 0xFF
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Number(10.0))); // 0b1010
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::BigInt("42n".to_string())));
    
    println!("Numbers test passed");
}

#[test]
fn test_operators() {
    let source = "let result = a + b * c / d % e;";
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify operators are handled correctly
    let token_kinds: Vec<&v8_lexer::TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Plus));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Star));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Slash));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Percent));
    
    println!("Operators test passed");
}

#[test]
fn test_comments() {
    let source = r#"
        // This is a line comment
        let x = 42; /* This is a block comment */
        /*
         * Multi-line comment
         */
        let y = 100;
    "#;
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify comments are handled correctly
    let token_kinds: Vec<&v8_lexer::TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    assert!(token_kinds.iter().any(|k| matches!(k, v8_lexer::TokenKind::Comment(_))));
    
    println!("Comments test passed");
}

#[test]
fn test_whitespace_handling() {
    let source = "   \t\n   let   x   =   42   ;   \n   ";
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify whitespace is handled correctly (should be skipped)
    let token_kinds: Vec<&v8_lexer::TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Identifier("x".to_string())));
    assert!(token_kinds.contains(&&v8_lexer::TokenKind::Number(42.0)));
    
    println!("Whitespace handling test passed");
}

#[test]
fn test_position_tracking() {
    let source = "let x = 42;\nlet y = 100;";
    
    // Step 1: Lexical analysis
    let tokens = tokenize(source).unwrap();
    assert!(!tokens.is_empty());
    
    // Verify position tracking works correctly
    assert_eq!(tokens[0].start().line, 1);
    assert_eq!(tokens[0].start().column, 1);
    
    println!("Position tracking test passed");
}

#[test]
fn test_large_source() {
    // Generate a large source with many tokens
    let mut source = String::new();
    for i in 0..100 {
        source.push_str(&format!("let var{} = {};", i, i));
    }
    
    // Step 1: Lexical analysis
    let tokens = tokenize(&source).unwrap();
    assert!(!tokens.is_empty());
    assert!(tokens.len() > 300); // At least 3 tokens per line
    
    println!("Large source test passed with {} tokens", tokens.len());
}

#[test]
fn test_performance() {
    use std::time::Instant;
    
    let source = "let x = 42; let y = 100; let z = x + y;";
    let iterations = 1000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _tokens = tokenize(source).unwrap();
    }
    let duration = start.elapsed();
    
    println!("Performance test: {} iterations took {:?}", iterations, duration);
    
    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_memory_usage() {
    let source = "let x = 42;";
    let tokens = tokenize(source).unwrap();
    
    // Check that tokens don't consume excessive memory
    let token_count = tokens.len();
    let estimated_memory = token_count * std::mem::size_of::<v8_lexer::Token>();
    
    println!("Memory usage: {} bytes for {} tokens", estimated_memory, token_count);
    
    // Should be reasonable memory usage (less than 1MB for small source)
    assert!(estimated_memory < 1_000_000);
} 