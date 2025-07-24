use v8_parser::Parser;
use v8_ast::Node;

#[test]
fn test_function_declaration() {
    let mut parser = Parser::new("function foo() { return 1; }");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::FunctionDeclaration(func) = &program.body[0] {
            if let Some(id) = &func.id {
                if let Node::Identifier(id_str) = &**id {
                    assert_eq!(id_str, "foo");
                } else {
                    panic!("Expected Identifier");
                }
            } else {
                panic!("Expected Identifier");
            }
            assert_eq!(func.params.len(), 0);
            assert!(!func.generator);
            assert!(!func.r#async);
        } else {
            panic!("Expected FunctionDeclaration");
        }
    }
}

#[test]
fn test_function_declaration_with_params() {
    let mut parser = Parser::new("function add(x, y) { return x + y; }");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::FunctionDeclaration(func) = &program.body[0] {
            if let Some(id) = &func.id {
                if let Node::Identifier(id_str) = &**id {
                    assert_eq!(id_str, "add");
                }
            }
            assert_eq!(func.params.len(), 2);
            if let Node::Identifier(param1) = &func.params[0] {
                assert_eq!(param1, "x");
            } else {
                panic!("Expected Identifier param");
            }
        } else {
            panic!("Expected FunctionDeclaration");
        }
    }
}

#[test]
fn test_class_declaration() {
    let mut parser = Parser::new("class MyClass { constructor() {} }");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::ClassDeclaration(class) = &program.body[0] {
            if let Some(id) = &class.id {
                if let Node::Identifier(id_str) = &**id {
                    assert_eq!(id_str, "MyClass");
                } else {
                    panic!("Expected Identifier");
                }
            } else {
                panic!("Expected Identifier");
            }
            assert!(class.super_class.is_none());
        } else {
            panic!("Expected ClassDeclaration");
        }
    }
}

#[test]
fn test_class_declaration_with_extends() {
    let mut parser = Parser::new("class Child extends Parent { }");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::ClassDeclaration(class) = &program.body[0] {
            if let Some(id) = &class.id {
                if let Node::Identifier(id_str) = &**id {
                    assert_eq!(id_str, "Child");
                }
            }
            if let Some(super_class) = &class.super_class {
                if let Node::Identifier(super_class_str) = &**super_class {
                    assert_eq!(super_class_str, "Parent");
                } else {
                    panic!("Expected super class");
                }
            } else {
                panic!("Expected super class");
            }
        } else {
            panic!("Expected ClassDeclaration");
        }
    }
}

#[test]
fn test_const_declaration() {
    let mut parser = Parser::new("const PI = 3.14;");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            assert_eq!(decl.kind, "const");
            assert_eq!(decl.declarations.len(), 1);
            if let Node::Identifier(id) = &*decl.declarations[0].id {
                assert_eq!(id, "PI");
            } else {
                panic!("Expected Identifier");
            }
        } else {
            panic!("Expected VariableDeclaration");
        }
    }
}

#[test]
fn test_var_declaration() {
    let mut parser = Parser::new("var x, y, z;");
    let result = parser.parse();
    assert!(result.is_ok());
    
    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            assert_eq!(decl.kind, "var");
            assert_eq!(decl.declarations.len(), 3);
        } else {
            panic!("Expected VariableDeclaration");
        }
    }
} 