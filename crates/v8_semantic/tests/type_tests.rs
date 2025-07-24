use v8_semantic::{analyze, SemanticError, Type};
use v8_parser::Parser;

#[test]
fn test_type_compatibility() {
    let mut parser = Parser::new("let x: number = 42;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_number_operations() {
    let mut parser = Parser::new("let result = 5 + 3 * 2;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_boolean_operations() {
    let mut parser = Parser::new("let result = true && false;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_string_operations() {
    let mut parser = Parser::new("let result = 'hello' + ' world';");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_type_mismatch() {
    let mut parser = Parser::new("let result = 5 + 'hello';");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    // This should be valid in JavaScript (type coercion)
    assert!(result.is_ok());
}

#[test]
fn test_function_call() {
    let mut parser = Parser::new("function add(a, b) { return a + b; } let result = add(1, 2);");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_undefined_function_call() {
    let mut parser = Parser::new("let result = undefinedFunction();");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    
    if let Err(SemanticError::UndeclaredFunction { name, .. }) = result {
        assert_eq!(name, "undefinedFunction");
    } else {
        panic!("Expected UndeclaredFunction error");
    }
} 