//! Abstract Syntax Tree for V8-Rust JavaScript engine
//! 
//! This crate provides a complete AST implementation for JavaScript/ECMAScript with:
//! - All ECMAScript node types
//! - Serialization support
//! - Visitor pattern support
//! - Source location tracking

pub mod node;
pub mod visitor;

pub use node::*;
pub use visitor::*;

/// Re-export commonly used types
pub use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ast_creation() {
        // Test basic node creation
        let identifier = Node::Identifier("x".to_string());
        let number = Node::Number(42.0);
        let string = Node::String("hello".to_string());
        let boolean = Node::Boolean(true);
        
        assert!(matches!(identifier, Node::Identifier(name) if name == "x"));
        assert!(matches!(number, Node::Number(n) if n == 42.0));
        assert!(matches!(string, Node::String(s) if s == "hello"));
        assert!(matches!(boolean, Node::Boolean(b) if b == true));
    }

    #[test]
    fn test_binary_expression() {
        let left = Node::Identifier("a".to_string());
        let right = Node::Identifier("b".to_string());
        let binary = Node::BinaryExpression(BinaryExpression {
            left: Box::new(left),
            operator: "+".to_string(),
            right: Box::new(right),
            span: None,
        });
        
        assert!(matches!(binary, Node::BinaryExpression(expr) if expr.operator == "+"));
    }

    #[test]
    fn test_program() {
        let body = vec![
            Node::Identifier("x".to_string()),
            Node::Number(42.0),
        ];
        let program = Node::Program(Program {
            body,
            source_type: "script".to_string(),
            span: None,
        });
        
        assert!(matches!(program, Node::Program(prog) if prog.body.len() == 2));
    }

    #[test]
    fn test_serialization() {
        let ast = Node::Program(Program {
            body: vec![
                Node::Identifier("x".to_string()),
                Node::Number(42.0),
            ],
            source_type: "script".to_string(),
            span: None,
        });
        
        let serialized = serde_json::to_string(&ast).unwrap();
        let deserialized: Node = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ast, deserialized);
    }

    #[test]
    fn test_visitor() {
        let ast = Node::Program(Program {
            body: vec![
                Node::Identifier("x".to_string()),
                Node::Number(42.0),
            ],
            source_type: "script".to_string(),
            span: None,
        });
        
        let mut counter = NodeCounter::new();
        counter.visit_node(&ast);
        assert_eq!(counter.count, 3); // Program + 2 children
    }
} 