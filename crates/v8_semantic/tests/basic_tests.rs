use v8_semantic::{analyze, SemanticError};
use v8_parser::Parser;

#[test]
fn test_valid_variable_declaration() {
    let mut parser = Parser::new("let x = 42;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_valid_function_declaration() {
    let mut parser = Parser::new("function add(a, b) { return a + b; }");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    
    if let Err(e) = &result {
        println!("Semantic error: {}", e);
    }
    
    assert!(result.is_ok());
}

#[test]
fn test_valid_binary_expression() {
    let mut parser = Parser::new("let result = 5 + 3;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_valid_if_statement() {
    let mut parser = Parser::new("if (true) { let x = 1; }");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_valid_while_statement() {
    let mut parser = Parser::new("while (false) { let x = 1; }");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
} 