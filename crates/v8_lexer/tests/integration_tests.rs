//! Integration tests for v8_lexer
//! 
//! Tests that verify the lexer works correctly with real JavaScript code.

use v8_lexer::{tokenize, TokenKind};

#[test]
fn test_simple_javascript_program() {
    let source = r#"
        let x = 42;
        let y = "hello";
        let z = x + y;
    "#;
    
    let tokens = tokenize(source).unwrap();
    
    // Should contain the expected tokens
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("x".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("y".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("z".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Number(42.0)));
    assert!(token_kinds.contains(&&TokenKind::String("hello".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Plus));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
}

#[test]
fn test_function_declaration() {
    let source = r#"
        function add(a, b) {
            return a + b;
        }
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("function".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("add".to_string())));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Identifier("a".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Identifier("b".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::Keyword("return".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Plus));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
}

#[test]
fn test_if_statement() {
    let source = r#"
        if (x > 0) {
            console.log("positive");
        } else {
            console.log("negative");
        }
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("if".to_string())));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Identifier("x".to_string())));
    assert!(token_kinds.contains(&&TokenKind::GreaterThan));
    assert!(token_kinds.contains(&&TokenKind::Number(0.0)));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::Keyword("else".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
}

#[test]
fn test_for_loop() {
    let source = r#"
        for (let i = 0; i < 10; i++) {
            console.log(i);
        }
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("for".to_string())));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("i".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::Number(0.0)));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
    assert!(token_kinds.contains(&&TokenKind::LessThan));
    assert!(token_kinds.contains(&&TokenKind::Number(10.0)));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
    assert!(token_kinds.contains(&&TokenKind::Increment));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
}

#[test]
fn test_array_literal() {
    let source = r#"
        let arr = [1, 2, 3, "hello", true];
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("arr".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::LeftBracket));
    assert!(token_kinds.contains(&&TokenKind::Number(1.0)));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Number(2.0)));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Number(3.0)));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::String("hello".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Boolean(true)));
    assert!(token_kinds.contains(&&TokenKind::RightBracket));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
}

#[test]
fn test_object_literal() {
    let source = r#"
        let obj = {
            name: "John",
            age: 30,
            isActive: true
        };
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("obj".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::Identifier("name".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Colon));
    assert!(token_kinds.contains(&&TokenKind::String("John".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Identifier("age".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Colon));
    assert!(token_kinds.contains(&&TokenKind::Number(30.0)));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Identifier("isActive".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Colon));
    assert!(token_kinds.contains(&&TokenKind::Boolean(true)));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
}

#[test]
fn test_template_literals() {
    let source = r#"
        let name = "World";
        let greeting = `Hello, ${name}!`;
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("name".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::String("World".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
    assert!(token_kinds.contains(&&TokenKind::Identifier("greeting".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::TemplateString("Hello, ${name}!".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
}

#[test]
fn test_arrow_function() {
    let source = r#"
        const add = (a, b) => a + b;
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("const".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("add".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Identifier("a".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Identifier("b".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::Arrow));
    assert!(token_kinds.contains(&&TokenKind::Plus));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
}

#[test]
fn test_class_declaration() {
    let source = r#"
        class Person {
            constructor(name) {
                this.name = name;
            }
            
            sayHello() {
                return `Hello, ${this.name}!`;
            }
        }
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("class".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("Person".to_string())));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::Keyword("constructor".to_string())));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Identifier("name".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::Keyword("this".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Dot));
    assert!(token_kinds.contains(&&TokenKind::Identifier("name".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::Identifier("name".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
}

#[test]
fn test_import_export_statements() {
    let source = r#"
        import { useState } from 'react';
        export default function App() {}
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("import".to_string())));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::Identifier("useState".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
    assert!(token_kinds.contains(&&TokenKind::Keyword("from".to_string())));
    assert!(token_kinds.contains(&&TokenKind::String("react".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
    assert!(token_kinds.contains(&&TokenKind::Keyword("export".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Keyword("default".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Keyword("function".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("App".to_string())));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
}

#[test]
fn test_comments_and_whitespace() {
    let source = r#"
        // This is a line comment
        let x = 42; /* This is a block comment */
        
        /*
         * This is a multi-line
         * block comment
         */
        let y = 100;
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    // Should contain comments
    assert!(token_kinds.iter().any(|k| matches!(k, TokenKind::Comment(_))));
    
    // Should contain the expected tokens
    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("x".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Number(42.0)));
    assert!(token_kinds.contains(&&TokenKind::Identifier("y".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Number(100.0)));
}

#[test]
fn test_complex_expression() {
    let source = r#"
        let result = (a + b) * (c - d) / (e % f);
    "#;
    
    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
    
    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("result".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Assign));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Identifier("a".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Plus));
    assert!(token_kinds.contains(&&TokenKind::Identifier("b".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::Star));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Identifier("c".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Minus));
    assert!(token_kinds.contains(&&TokenKind::Identifier("d".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::Slash));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Identifier("e".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Percent));
    assert!(token_kinds.contains(&&TokenKind::Identifier("f".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
} 