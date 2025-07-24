use v8_semantic::{analyze, SemanticError};
use v8_parser::Parser;

#[test]
fn test_array_literal() {
    let mut parser = Parser::new("let arr = [1, 2, 3];");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_object_literal() {
    let mut parser = Parser::new("let obj = { x: 1, y: 2 };");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_member_expression() {
    let mut parser = Parser::new("let obj = { x: 1 }; let value = obj.x;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    
    if let Err(e) = &result {
        println!("Semantic error: {}", e);
    }
    
    assert!(result.is_ok());
}

#[test]
fn test_logical_expression() {
    let mut parser = Parser::new("let result = true && false;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_conditional_expression() {
    let mut parser = Parser::new("let result = true ? 1 : 2;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_arrow_function() {
    let mut parser = Parser::new("let add = (a, b) => a + b;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_arrow_function_call() {
    let mut parser = Parser::new("function add(x) { return x + 1; } let result = add(1);");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    
    if let Err(e) = &result {
        println!("Semantic error: {}", e);
    }
    
    assert!(result.is_ok());
}

#[test]
fn test_nested_expressions() {
    let mut parser = Parser::new("let result = (a, b) => a + b ? true : false;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_complex_object() {
    let mut parser = Parser::new("let obj = { x: 1, y: [2, 3], z: { a: 4 } };");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_array_with_mixed_types() {
    let mut parser = Parser::new("let arr = [1, 'hello', true];");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
} 