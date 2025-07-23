//! Tests for AST visitor pattern

use v8_ast::*;
use super::common::*;

/// Test visitor that counts different types of nodes
#[derive(Debug)]
struct NodeTypeCounter {
    identifiers: usize,
    numbers: usize,
    strings: usize,
    booleans: usize,
    binary_expressions: usize,
    function_declarations: usize,
    variable_declarations: usize,
    if_statements: usize,
    call_expressions: usize,
    member_expressions: usize,
    programs: usize,
    block_statements: usize,
}

impl NodeTypeCounter {
    fn new() -> Self {
        Self {
            identifiers: 0,
            numbers: 0,
            strings: 0,
            booleans: 0,
            binary_expressions: 0,
            function_declarations: 0,
            variable_declarations: 0,
            if_statements: 0,
            call_expressions: 0,
            member_expressions: 0,
            programs: 0,
            block_statements: 0,
        }
    }
}

impl Visitor for NodeTypeCounter {
    type Output = ();

    fn visit_identifier(&mut self, _id: &str) {
        self.identifiers += 1;
    }

    fn visit_number(&mut self, _num: f64) {
        self.numbers += 1;
    }

    fn visit_string(&mut self, _s: &str) {
        self.strings += 1;
    }

    fn visit_boolean(&mut self, _b: bool) {
        self.booleans += 1;
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) {
        self.binary_expressions += 1;
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
    }

    fn visit_function_declaration(&mut self, decl: &FunctionDeclaration) {
        self.function_declarations += 1;
        if let Some(id) = &decl.id {
            self.visit_node(id);
        }
        for param in &decl.params {
            self.visit_node(param);
        }
        self.visit_node(&decl.body);
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) {
        self.variable_declarations += 1;
        for var_decl in &decl.declarations {
            self.visit_node(&var_decl.id);
            if let Some(init) = &var_decl.init {
                self.visit_node(init);
            }
        }
    }

    fn visit_if_statement(&mut self, stmt: &IfStatement) {
        self.if_statements += 1;
        self.visit_node(&stmt.test);
        self.visit_node(&stmt.consequent);
        if let Some(alternate) = &stmt.alternate {
            self.visit_node(alternate);
        }
    }

    fn visit_call_expression(&mut self, expr: &CallExpression) {
        self.call_expressions += 1;
        self.visit_node(&expr.callee);
        for arg in &expr.arguments {
            self.visit_node(arg);
        }
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression) {
        self.member_expressions += 1;
        self.visit_node(&expr.object);
        self.visit_node(&expr.property);
    }

    fn visit_program(&mut self, program: &Program) {
        self.programs += 1;
        for node in &program.body {
            self.visit_node(node);
        }
    }

    fn visit_block_statement(&mut self, stmt: &BlockStatement) {
        self.block_statements += 1;
        for node in &stmt.body {
            self.visit_node(node);
        }
    }
}

/// Test visitor that collects all identifiers
#[derive(Debug)]
struct IdentifierCollector {
    identifiers: Vec<String>,
}

impl IdentifierCollector {
    fn new() -> Self {
        Self {
            identifiers: Vec::new(),
        }
    }
}

impl Visitor for IdentifierCollector {
    type Output = ();

    fn visit_identifier(&mut self, id: &str) {
        self.identifiers.push(id.to_string());
    }
}

/// Test visitor that calculates expression complexity
#[derive(Debug)]
struct ComplexityCalculator {
    complexity: usize,
}

impl ComplexityCalculator {
    fn new() -> Self {
        Self { complexity: 0 }
    }
}

impl Visitor for ComplexityCalculator {
    type Output = ();

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) {
        self.complexity += 1;
        self.visit_node(&expr.left);
        self.visit_node(&expr.right);
    }

    fn visit_unary_expression(&mut self, expr: &UnaryExpression) {
        self.complexity += 1;
        self.visit_node(&expr.argument);
    }

    fn visit_call_expression(&mut self, expr: &CallExpression) {
        self.complexity += 2;
        self.visit_node(&expr.callee);
        for arg in &expr.arguments {
            self.visit_node(arg);
        }
    }

    fn visit_member_expression(&mut self, expr: &MemberExpression) {
        self.complexity += 1;
        self.visit_node(&expr.object);
        self.visit_node(&expr.property);
    }
}

#[test]
fn test_node_counter_visitor() {
    let mut counter = NodeTypeCounter::new();
    
    // Create a simple AST
    let ast = create_program(vec![
        create_variable_declaration("let", "x", Some(create_number(42.0))),
        create_function_declaration("fn", vec![create_identifier("param")], 
            create_block_statement(vec![
                create_return_statement(Some(create_identifier("param")))
            ])
        ),
        create_if_statement(
            create_identifier("condition"),
            create_block_statement(vec![
                create_expression_statement(create_identifier("stmt"))
            ]),
            None
        ),
    ]);
    
    counter.visit_node(&ast);
    
    assert_eq!(counter.programs, 1);
    assert_eq!(counter.variable_declarations, 1);
    assert_eq!(counter.function_declarations, 1);
    assert_eq!(counter.if_statements, 1);
    assert_eq!(counter.block_statements, 2);
    assert_eq!(counter.identifiers, 5); // x, fn, param, param, condition, stmt
    assert_eq!(counter.numbers, 1); // 42.0
}

#[test]
fn test_identifier_collector_visitor() {
    let mut collector = IdentifierCollector::new();
    
    // Create AST with multiple identifiers
    let ast = create_program(vec![
        create_variable_declaration("let", "x", Some(create_identifier("y"))),
        create_binary_expression(
            create_identifier("a"),
            "+",
            create_identifier("b")
        ),
        create_call_expression(
            create_member_expression(
                create_identifier("obj"),
                create_identifier("method"),
                false
            ),
            vec![create_identifier("arg1"), create_identifier("arg2")]
        ),
    ]);
    
    collector.visit_node(&ast);
    
    assert_eq!(collector.identifiers.len(), 7);
    assert!(collector.identifiers.contains(&"x".to_string()));
    assert!(collector.identifiers.contains(&"y".to_string()));
    assert!(collector.identifiers.contains(&"a".to_string()));
    assert!(collector.identifiers.contains(&"b".to_string()));
    assert!(collector.identifiers.contains(&"obj".to_string()));
    assert!(collector.identifiers.contains(&"method".to_string()));
    assert!(collector.identifiers.contains(&"arg1".to_string()));
    assert!(collector.identifiers.contains(&"arg2".to_string()));
}

#[test]
fn test_complexity_calculator_visitor() {
    let mut calculator = ComplexityCalculator::new();
    
    // Create a complex expression
    let ast = create_binary_expression(
        create_call_expression(
            create_member_expression(
                create_identifier("Math"),
                create_identifier("pow"),
                false
            ),
            vec![create_number(2.0), create_number(3.0)]
        ),
        "+",
        create_unary_expression(
            "!",
            create_identifier("flag"),
            true
        )
    );
    
    calculator.visit_node(&ast);
    
    // Binary expression (1) + Call expression (2) + Member expression (1) + Unary expression (1) = 5
    assert_eq!(calculator.complexity, 5);
}

#[test]
fn test_visitor_with_nested_structures() {
    let mut counter = NodeTypeCounter::new();
    
    // Create a deeply nested AST
    let ast = create_program(vec![
        create_function_declaration("complexFunction", vec![], 
            create_block_statement(vec![
                create_if_statement(
                    create_binary_expression(
                        create_identifier("a"),
                        ">",
                        create_number(0.0)
                    ),
                    create_block_statement(vec![
                        create_expression_statement(
                            create_assignment_expression(
                                create_identifier("result"),
                                "=",
                                create_call_expression(
                                    create_identifier("calculate"),
                                    vec![
                                        create_binary_expression(
                                            create_identifier("x"),
                                            "*",
                                            create_identifier("y")
                                        )
                                    ]
                                )
                            )
                        )
                    ]),
                    Some(create_block_statement(vec![
                        create_expression_statement(
                            create_assignment_expression(
                                create_identifier("result"),
                                "=",
                                create_number(0.0)
                            )
                        )
                    ]))
                )
            ])
        )
    ]);
    
    counter.visit_node(&ast);
    
    assert_eq!(counter.programs, 1);
    assert_eq!(counter.function_declarations, 1);
    assert_eq!(counter.if_statements, 1);
    assert_eq!(counter.block_statements, 3);
    assert_eq!(counter.binary_expressions, 2);
    assert_eq!(counter.call_expressions, 1);
    assert_eq!(counter.identifiers, 7); // complexFunction, a, result, calculate, x, y, result
    assert_eq!(counter.numbers, 2); // 0.0, 0.0
}

#[test]
fn test_visitor_with_all_node_types() {
    let mut counter = NodeTypeCounter::new();
    
    // Create AST with all major node types
    let ast = create_program(vec![
        // Variable declaration
        create_variable_declaration("const", "PI", Some(create_number(3.14159))),
        
        // Function declaration
        create_function_declaration("calculate", vec![create_identifier("radius")], 
            create_block_statement(vec![
                create_return_statement(Some(
                    create_binary_expression(
                        create_binary_expression(
                            create_identifier("PI"),
                            "*",
                            create_identifier("radius")
                        ),
                        "*",
                        create_identifier("radius")
                    )
                ))
            ])
        ),
        
        // If statement
        create_if_statement(
            create_boolean(true),
            create_block_statement(vec![
                create_expression_statement(
                    create_call_expression(
                        create_identifier("console"),
                        vec![create_string("Hello World")]
                    )
                )
            ]),
            None
        ),
        
        // Expression statement with complex expression
        create_expression_statement(
            create_assignment_expression(
                create_identifier("area"),
                "=",
                create_call_expression(
                    create_identifier("calculate"),
                    vec![create_number(5.0)]
                )
            )
        ),
    ]);
    
    counter.visit_node(&ast);
    
    // Verify all node types were visited
    assert_eq!(counter.programs, 1);
    assert_eq!(counter.variable_declarations, 1);
    assert_eq!(counter.function_declarations, 1);
    assert_eq!(counter.if_statements, 1);
    assert_eq!(counter.block_statements, 2);
    assert_eq!(counter.binary_expressions, 2);
    assert_eq!(counter.call_expressions, 2);
    assert_eq!(counter.identifiers, 6); // PI, calculate, radius, PI, radius, radius, console, area, calculate
    assert_eq!(counter.numbers, 3); // 3.14159, 5.0
    assert_eq!(counter.strings, 1); // "Hello World"
    assert_eq!(counter.booleans, 1); // true
}

#[test]
fn test_visitor_error_handling() {
    let mut counter = NodeTypeCounter::new();
    
    // Test with empty program
    let empty_program = create_program(vec![]);
    counter.visit_node(&empty_program);
    
    assert_eq!(counter.programs, 1);
    assert_eq!(counter.identifiers, 0);
    assert_eq!(counter.numbers, 0);
}

#[test]
fn test_visitor_with_literals() {
    let mut counter = NodeTypeCounter::new();
    
    let ast = create_program(vec![
        create_expression_statement(create_number(42.0)),
        create_expression_statement(create_string("hello")),
        create_expression_statement(create_boolean(true)),
        create_expression_statement(create_boolean(false)),
        create_expression_statement(create_null()),
        create_expression_statement(create_undefined()),
        create_expression_statement(create_this()),
    ]);
    
    counter.visit_node(&ast);
    
    assert_eq!(counter.numbers, 1);
    assert_eq!(counter.strings, 1);
    assert_eq!(counter.booleans, 2);
}

#[test]
fn test_visitor_with_arrays_and_objects() {
    let mut counter = NodeTypeCounter::new();
    
    let ast = create_program(vec![
        create_expression_statement(
            create_array_literal(vec![
                Some(create_number(1.0)),
                Some(create_string("two")),
                Some(create_boolean(true)),
                None, // hole
            ])
        ),
        create_expression_statement(
            create_object_literal(vec![
                create_property(
                    create_identifier("key1"),
                    create_number(42.0),
                    "init"
                ),
                create_property(
                    create_string("key2"),
                    create_string("value"),
                    "init"
                ),
            ])
        ),
    ]);
    
    counter.visit_node(&ast);
    
    assert_eq!(counter.numbers, 2); // 1.0, 42.0
    assert_eq!(counter.strings, 3); // "two", "key2", "value"
    assert_eq!(counter.booleans, 1); // true
    assert_eq!(counter.identifiers, 1); // "key1"
}

#[test]
fn test_visitor_with_control_flow() {
    let mut counter = NodeTypeCounter::new();
    
    let ast = create_program(vec![
        create_while_statement(
            create_identifier("condition"),
            create_block_statement(vec![
                create_break_statement(None),
            ])
        ),
        create_for_statement(
            Some(create_variable_declaration("let", "i", Some(create_number(0.0)))),
            Some(create_binary_expression(create_identifier("i"), "<", create_number(10.0))),
            Some(create_update_expression("++", create_identifier("i"), false)),
            create_block_statement(vec![
                create_continue_statement(None),
            ])
        ),
        create_switch_statement(
            create_identifier("value"),
            vec![
                create_switch_case(
                    Some(create_number(1.0)),
                    vec![create_identifier("case1")]
                ),
                create_switch_case(
                    None,
                    vec![create_identifier("default")]
                ),
            ]
        ),
    ]);
    
    counter.visit_node(&ast);
    
    assert_eq!(counter.identifiers, 6); // condition, i, i, 10.0, i, value, case1, default
    assert_eq!(counter.numbers, 3); // 0.0, 10.0, 1.0
    assert_eq!(counter.variable_declarations, 1);
    assert_eq!(counter.binary_expressions, 1);
} 