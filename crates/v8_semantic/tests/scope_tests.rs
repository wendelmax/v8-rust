use v8_semantic::{analyze, SemanticError};
use v8_parser::Parser;

#[test]
fn test_block_scope() {
    let mut parser = Parser::new("let x = 1; { let y = 2; } let z = y;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    
    if let Err(SemanticError::UndeclaredVariable { name, .. }) = result {
        assert_eq!(name, "y");
    } else {
        panic!("Expected UndeclaredVariable error for y");
    }
}

#[test]
fn test_function_scope() {
    let mut parser = Parser::new("function test() { let x = 1; } let y = x;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    
    if let Err(SemanticError::UndeclaredVariable { name, .. }) = result {
        assert_eq!(name, "x");
    } else {
        panic!("Expected UndeclaredVariable error for x");
    }
}

#[test]
fn test_nested_scopes() {
    let mut parser = Parser::new("let x = 1; { let x = 2; }");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok()); // Should be valid - different scopes
}

#[test]
fn test_this_in_function() {
    let mut parser = Parser::new("function test() { return this; }");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    
    if let Err(e) = &result {
        println!("Semantic error: {}", e);
    }
    
    assert!(result.is_ok()); // Should be valid in function scope
}

#[test]
fn test_this_in_global() {
    let mut parser = Parser::new("let x = this;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    
    if let Err(SemanticError::InvalidThisUsage { .. }) = result {
        // Expected error
    } else {
        panic!("Expected InvalidThisUsage error");
    }
} 