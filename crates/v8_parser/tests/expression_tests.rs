use v8_parser::Parser;
use v8_ast::Node;

#[test]
fn test_unary_expression() {
    let mut parser = Parser::new("-5");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::UnaryExpression(expr) = &*stmt.expression {
                assert_eq!(expr.operator, "-");
                assert!(expr.prefix);
                if let Node::Number(num) = &*expr.argument {
                    assert_eq!(*num, 5.0);
                } else {
                    panic!("Expected Number argument");
                }
            } else {
                panic!("Expected UnaryExpression");
            }
        }
    }
}

#[test]
fn test_logical_expression() {
    let mut parser = Parser::new("true && false");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::LogicalExpression(expr) = &*stmt.expression {
                assert_eq!(expr.operator, "&&");
                if let Node::Boolean(left) = &*expr.left {
                    assert_eq!(*left, true);
                } else {
                    panic!("Expected Boolean on left");
                }
                if let Node::Boolean(right) = &*expr.right {
                    assert_eq!(*right, false);
                } else {
                    panic!("Expected Boolean on right");
                }
            } else {
                panic!("Expected LogicalExpression");
            }
        }
    }
}

#[test]
fn test_assignment_expression() {
    let mut parser = Parser::new("x = 10");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::AssignmentExpression(expr) = &*stmt.expression {
                assert_eq!(expr.operator, "=");
                if let Node::Identifier(id) = &*expr.left {
                    assert_eq!(id, "x");
                } else {
                    panic!("Expected Identifier on left");
                }
                if let Node::Number(num) = &*expr.right {
                    assert_eq!(*num, 10.0);
                } else {
                    panic!("Expected Number on right");
                }
            } else {
                panic!("Expected AssignmentExpression");
            }
        }
    }
}

#[test]
fn test_call_expression() {
    let mut parser = Parser::new("foo()");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::CallExpression(expr) = &*stmt.expression {
                if let Node::Identifier(id) = &*expr.callee {
                    assert_eq!(id, "foo");
                } else {
                    panic!("Expected Identifier callee");
                }
                assert_eq!(expr.arguments.len(), 0);
            } else {
                panic!("Expected CallExpression");
            }
        }
    }
}

#[test]
fn test_array_literal() {
    let mut parser = Parser::new("[1, 2, 3]");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::ArrayLiteral(arr) = &*stmt.expression {
                assert_eq!(arr.elements.len(), 3);
                if let Some(Node::Number(num)) = &arr.elements[0] {
                    assert_eq!(*num, 1.0);
                } else {
                    panic!("Expected Number at index 0");
                }
            } else {
                panic!("Expected ArrayLiteral");
            }
        }
    }
} 

#[test]
fn test_arrow_function() {
    let mut parser = Parser::new("const add = (a, b) => a + b;");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            if let Node::ArrowFunctionExpression(arrow) = &**decl.declarations[0].init.as_ref().unwrap() {
                assert_eq!(arrow.params.len(), 2);
                assert!(arrow.expression);
                assert!(!arrow.r#async);
            } else {
                panic!("Expected ArrowFunctionExpression");
            }
        } else {
            panic!("Expected VariableDeclaration");
        }
    }
}

#[test]
fn test_arrow_function_single_param() {
    let mut parser = Parser::new("const double = x => x * 2;");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            if let Node::ArrowFunctionExpression(arrow) = &**decl.declarations[0].init.as_ref().unwrap() {
                assert_eq!(arrow.params.len(), 1);
                assert!(arrow.expression);
            } else {
                panic!("Expected ArrowFunctionExpression");
            }
        }
    }
}

#[test]
fn test_arrow_function_block_body() {
    let mut parser = Parser::new("const greet = (name) => { return `Hello ${name}`; };");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            if let Node::ArrowFunctionExpression(arrow) = &**decl.declarations[0].init.as_ref().unwrap() {
                assert_eq!(arrow.params.len(), 1);
                assert!(!arrow.expression);
            } else {
                panic!("Expected ArrowFunctionExpression");
            }
        }
    }
}

#[test]
fn test_nullish_coalescing() {
    let mut parser = Parser::new("const value = a ?? b;");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            if let Node::LogicalExpression(logical) = &**decl.declarations[0].init.as_ref().unwrap() {
                assert_eq!(logical.operator, "??");
            } else {
                panic!("Expected LogicalExpression");
            }
        }
    }
}

#[test]
fn test_destructuring_assignment() {
    let mut parser = Parser::new("const { x, y } = obj;");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            if let Node::ObjectLiteral(obj) = &*decl.declarations[0].id {
                assert_eq!(obj.properties.len(), 2);
            } else {
                panic!("Expected ObjectLiteral");
            }
        }
    }
}

#[test]
fn test_array_destructuring() {
    let mut parser = Parser::new("const [first, second] = arr;");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            if let Node::ArrayLiteral(arr) = &*decl.declarations[0].id {
                assert_eq!(arr.elements.len(), 2);
            } else {
                panic!("Expected ArrayLiteral");
            }
        }
    }
}

#[test]
fn test_spread_operator() {
    let mut parser = Parser::new("const newArr = [...arr, 4, 5];");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            if let Node::ArrayLiteral(arr) = &**decl.declarations[0].init.as_ref().unwrap() {
                assert_eq!(arr.elements.len(), 3);
                if let Some(Node::SpreadElement(_)) = &arr.elements[0] {
                    // Spread element found
                } else {
                    panic!("Expected SpreadElement");
                }
            } else {
                panic!("Expected ArrayLiteral");
            }
        }
    }
} 