use v8_semantic::{analyze, SemanticError};
use v8_parser::Parser;

#[test]
fn test_undeclared_variable() {
    let mut parser = Parser::new("x = 42;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    
    if let Err(SemanticError::UndeclaredVariable { name, .. }) = result {
        assert_eq!(name, "x");
    } else {
        panic!("Expected UndeclaredVariable error");
    }
}

#[test]
fn test_const_reassignment() {
    let mut parser = Parser::new("const x = 42; x = 100;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    
    if let Err(SemanticError::ConstReassignment { name, .. }) = result {
        assert_eq!(name, "x");
    } else {
        panic!("Expected ConstReassignment error");
    }
}

#[test]
fn test_uninitialized_variable() {
    let mut parser = Parser::new("let x; let y = x;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    
    if let Err(SemanticError::UninitializedVariable { name, .. }) = result {
        assert_eq!(name, "x");
    } else {
        panic!("Expected UninitializedVariable error");
    }
}

#[test]
fn test_duplicate_declaration() {
    let mut parser = Parser::new("let x = 1; let x = 2;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    
    if let Err(SemanticError::DuplicateDeclaration { name, .. }) = result {
        assert_eq!(name, "x");
    } else {
        panic!("Expected DuplicateDeclaration error");
    }
} 