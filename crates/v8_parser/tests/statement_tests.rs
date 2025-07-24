use v8_parser::Parser;
use v8_ast::Node;

#[test]
fn test_variable_declaration() {
    let mut parser = Parser::new("let x = 5;");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            assert_eq!(decl.kind, "let");
            assert_eq!(decl.declarations.len(), 1);
            if let Node::Identifier(id) = &*decl.declarations[0].id {
                assert_eq!(id, "x");
            } else {
                panic!("Expected Identifier");
            }
        } else {
            panic!("Expected VariableDeclaration");
        }
    }
}

#[test]
fn test_if_statement() {
    let mut parser = Parser::new("if (true) { x = 1; }");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::IfStatement(stmt) = &program.body[0] {
            if let Node::Boolean(b) = &*stmt.test {
                assert_eq!(*b, true);
            } else {
                panic!("Expected Boolean test");
            }
            if let Node::BlockStatement(block) = &*stmt.consequent {
                assert_eq!(block.body.len(), 1);
            } else {
                panic!("Expected BlockStatement consequent");
            }
        } else {
            panic!("Expected IfStatement");
        }
    }
}

#[test]
fn test_while_statement() {
    let mut parser = Parser::new("while (x > 0) { x--; }");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::WhileStatement(stmt) = &program.body[0] {
            if let Node::BinaryExpression(expr) = &*stmt.test {
                assert_eq!(expr.operator, ">");
            } else {
                panic!("Expected BinaryExpression test");
            }
        } else {
            panic!("Expected WhileStatement");
        }
    }
}

#[test]
fn test_return_statement() {
    let mut parser = Parser::new("return 42;");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::ReturnStatement(stmt) = &program.body[0] {
            if let Some(arg) = &stmt.argument {
                if let Node::Number(num) = &**arg {
                    assert_eq!(*num, 42.0);
                } else {
                    panic!("Expected Number argument");
                }
            } else {
                panic!("Expected Number argument");
            }
        } else {
            panic!("Expected ReturnStatement");
        }
    }
}

#[test]
fn test_block_statement() {
    let mut parser = Parser::new("{ let x = 1; let y = 2; }");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::BlockStatement(block) = &program.body[0] {
            assert_eq!(block.body.len(), 2);
            if let Node::VariableDeclaration(decl) = &block.body[0] {
                assert_eq!(decl.kind, "let");
            } else {
                panic!("Expected VariableDeclaration");
            }
        } else {
            panic!("Expected BlockStatement");
        }
    }
} 