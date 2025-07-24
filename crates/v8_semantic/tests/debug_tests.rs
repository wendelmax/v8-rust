use v8_parser::Parser;
use v8_ast::Node;

#[test]
fn test_debug_ast() {
    let mut parser = Parser::new("let obj = { x: 1 }; let value = obj.x;");
    let ast = parser.parse().unwrap();
    
    println!("AST: {:?}", ast);
    
    // Just check if parsing succeeds
    assert!(true);
}

#[test]
fn test_debug_arrow_function() {
    let mut parser = Parser::new("let add = (a, b) => a + b; let result = add(1, 2);");
    let ast = parser.parse().unwrap();
    
    println!("Arrow Function AST: {:?}", ast);
    
    // Just check if parsing succeeds
    assert!(true);
} 

#[test]
fn test_debug_simple_arrow_function() {
    let mut parser = Parser::new("let add = x => x + 1; let result = add(1);");
    let ast = parser.parse().unwrap();
    
    println!("Simple Arrow Function AST: {:?}", ast);
    
    // Just check if parsing succeeds
    assert!(true);
} 