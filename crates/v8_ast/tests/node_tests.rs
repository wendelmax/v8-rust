//! Tests for AST node creation and manipulation

use v8_ast::*;
use super::common::*;

#[test]
fn test_position_creation() {
    let pos = Position::new(1, 5);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 5);
}

#[test]
fn test_span_creation() {
    let start = Position::new(1, 1);
    let end = Position::new(1, 10);
    let span = Span::new(start, end);
    
    assert_eq!(span.start.line, 1);
    assert_eq!(span.start.column, 1);
    assert_eq!(span.end.line, 1);
    assert_eq!(span.end.column, 10);
}

#[test]
fn test_span_from_positions() {
    let span = Span::from_positions(1, 1, 2, 5);
    assert_eq!(span.start.line, 1);
    assert_eq!(span.start.column, 1);
    assert_eq!(span.end.line, 2);
    assert_eq!(span.end.column, 5);
}

#[test]
fn test_literal_nodes() {
    // Identifier
    let id = create_identifier("x");
    assert!(matches!(id, Node::Identifier(name) if name == "x"));
    
    // Number
    let num = create_number(42.0);
    assert!(matches!(num, Node::Number(n) if n == 42.0));
    
    // String
    let str_lit = create_string("hello");
    assert!(matches!(str_lit, Node::String(s) if s == "hello"));
    
    // Boolean
    let bool_lit = create_boolean(true);
    assert!(matches!(bool_lit, Node::Boolean(b) if b == true));
    
    // Null
    let null_lit = create_null();
    assert!(matches!(null_lit, Node::Null));
    
    // Undefined
    let undefined_lit = create_undefined();
    assert!(matches!(undefined_lit, Node::Undefined));
    
    // This
    let this_lit = create_this();
    assert!(matches!(this_lit, Node::This));
    
    // BigInt
    let bigint_lit = create_bigint("42n");
    assert!(matches!(bigint_lit, Node::BigInt(s) if s == "42n"));
    
    // RegExp
    let regexp_lit = create_regexp("\\d+", "g");
    assert!(matches!(regexp_lit, Node::RegExp(r) if r.pattern == "\\d+" && r.flags == "g"));
}

#[test]
fn test_expression_nodes() {
    // Binary expression
    let left = create_identifier("a");
    let right = create_identifier("b");
    let bin_expr = create_binary_expression(left, "+", right);
    
    assert!(matches!(bin_expr, Node::BinaryExpression(expr) if expr.operator == "+"));
    
    // Unary expression
    let arg = create_identifier("x");
    let unary_expr = create_unary_expression("!", arg, true);
    
    assert!(matches!(unary_expr, Node::UnaryExpression(expr) if expr.operator == "!" && expr.prefix == true));
    
    // Call expression
    let callee = create_identifier("fn");
    let args = vec![create_number(1.0), create_number(2.0)];
    let call_expr = create_call_expression(callee, args);
    
    assert!(matches!(call_expr, Node::CallExpression(expr) if expr.arguments.len() == 2));
    
    // New expression
    let new_expr = create_new_expression(callee, args);
    
    assert!(matches!(new_expr, Node::NewExpression(expr) if expr.arguments.len() == 2));
    
    // Member expression
    let obj = create_identifier("obj");
    let prop = create_identifier("prop");
    let member_expr = create_member_expression(obj, prop, false);
    
    assert!(matches!(member_expr, Node::MemberExpression(expr) if expr.computed == false));
    
    // Assignment expression
    let assign_expr = create_assignment_expression(left, "=", right);
    
    assert!(matches!(assign_expr, Node::AssignmentExpression(expr) if expr.operator == "="));
    
    // Conditional expression
    let test = create_identifier("condition");
    let consequent = create_identifier("true_value");
    let alternate = create_identifier("false_value");
    let cond_expr = create_conditional_expression(test, consequent, alternate);
    
    assert!(matches!(cond_expr, Node::ConditionalExpression(_)));
    
    // Logical expression
    let logical_expr = create_logical_expression(left, "&&", right);
    
    assert!(matches!(logical_expr, Node::LogicalExpression(expr) if expr.operator == "&&"));
    
    // Update expression
    let update_expr = create_update_expression("++", create_identifier("x"), true);
    
    assert!(matches!(update_expr, Node::UpdateExpression(expr) if expr.operator == "++" && expr.prefix == true));
}

#[test]
fn test_statement_nodes() {
    // Variable declaration
    let var_decl = create_variable_declaration("let", "x", Some(create_number(42.0)));
    
    assert!(matches!(var_decl, Node::VariableDeclaration(decl) if decl.kind == "let"));
    
    // Function declaration
    let params = vec![create_identifier("param")];
    let body = create_block_statement(vec![]);
    let func_decl = create_function_declaration("fn", params, body);
    
    assert!(matches!(func_decl, Node::FunctionDeclaration(decl) if decl.params.len() == 1));
    
    // Block statement
    let block = create_block_statement(vec![create_identifier("stmt")]);
    
    assert!(matches!(block, Node::BlockStatement(stmt) if stmt.body.len() == 1));
    
    // If statement
    let test = create_identifier("condition");
    let consequent = create_block_statement(vec![]);
    let if_stmt = create_if_statement(test, consequent, None);
    
    assert!(matches!(if_stmt, Node::IfStatement(_)));
    
    // While statement
    let while_stmt = create_while_statement(create_identifier("test"), create_block_statement(vec![]));
    
    assert!(matches!(while_stmt, Node::WhileStatement(_)));
    
    // Do-while statement
    let do_while_stmt = create_do_while_statement(create_block_statement(vec![]), create_identifier("test"));
    
    assert!(matches!(do_while_stmt, Node::DoWhileStatement(_)));
    
    // For statement
    let for_stmt = create_for_statement(
        Some(create_identifier("init")),
        Some(create_identifier("test")),
        Some(create_identifier("update")),
        create_block_statement(vec![])
    );
    
    assert!(matches!(for_stmt, Node::ForStatement(_)));
    
    // Return statement
    let return_stmt = create_return_statement(Some(create_identifier("value")));
    
    assert!(matches!(return_stmt, Node::ReturnStatement(_)));
    
    // Break statement
    let break_stmt = create_break_statement(Some(create_identifier("label")));
    
    assert!(matches!(break_stmt, Node::BreakStatement(_)));
    
    // Continue statement
    let continue_stmt = create_continue_statement(Some(create_identifier("label")));
    
    assert!(matches!(continue_stmt, Node::ContinueStatement(_)));
    
    // Labeled statement
    let labeled_stmt = create_labeled_statement(create_identifier("label"), create_block_statement(vec![]));
    
    assert!(matches!(labeled_stmt, Node::LabeledStatement(_)));
    
    // Debugger statement
    let debugger_stmt = create_debugger_statement();
    
    assert!(matches!(debugger_stmt, Node::DebuggerStatement(_)));
    
    // Expression statement
    let expr_stmt = create_expression_statement(create_identifier("expr"));
    
    assert!(matches!(expr_stmt, Node::ExpressionStatement(_)));
    
    // With statement
    let with_stmt = create_with_statement(create_identifier("obj"), create_block_statement(vec![]));
    
    assert!(matches!(with_stmt, Node::WithStatement(_)));
}

#[test]
fn test_literal_structures() {
    // Array literal
    let elements = vec![
        Some(create_number(1.0)),
        Some(create_number(2.0)),
        None, // hole
    ];
    let array_lit = create_array_literal(elements);
    
    assert!(matches!(array_lit, Node::ArrayLiteral(lit) if lit.elements.len() == 3));
    
    // Object literal
    let properties = vec![
        create_property(create_identifier("key"), create_string("value"), "init"),
    ];
    let obj_lit = create_object_literal(properties);
    
    assert!(matches!(obj_lit, Node::ObjectLiteral(lit) if lit.properties.len() == 1));
    
    // Property
    let prop = create_property(create_identifier("key"), create_string("value"), "init");
    
    assert!(matches!(prop, Node::Property(p) if p.kind == "init"));
    
    // Template literal
    let quasis = vec![
        create_template_element("Hello ", false),
        create_template_element("!", true),
    ];
    let expressions = vec![create_identifier("name")];
    let template_lit = create_template_literal(quasis, expressions);
    
    assert!(matches!(template_lit, Node::TemplateLiteral(lit) if lit.quasis.len() == 2 && lit.expressions.len() == 1));
    
    // Tagged template expression
    let tag = create_identifier("tag");
    let quasi = create_template_literal(vec![], vec![]);
    let tagged_template = create_tagged_template_expression(tag, quasi);
    
    assert!(matches!(tagged_template, Node::TaggedTemplateExpression(_)));
}

#[test]
fn test_function_and_class_nodes() {
    // Arrow function expression
    let params = vec![create_identifier("x")];
    let body = create_identifier("x");
    let arrow_func = create_arrow_function_expression(params, body, true);
    
    assert!(matches!(arrow_func, Node::ArrowFunctionExpression(expr) if expr.expression == true));
    
    // Function expression
    let func_expr = create_function_expression(Some(create_identifier("name")), vec![], create_block_statement(vec![]));
    
    assert!(matches!(func_expr, Node::FunctionExpression(_)));
    
    // Class declaration
    let class_decl = create_class_declaration(
        Some(create_identifier("MyClass")),
        Some(create_identifier("Parent")),
        create_block_statement(vec![])
    );
    
    assert!(matches!(class_decl, Node::ClassDeclaration(_)));
    
    // Class expression
    let class_expr = create_class_expression(
        Some(create_identifier("MyClass")),
        Some(create_identifier("Parent")),
        create_block_statement(vec![])
    );
    
    assert!(matches!(class_expr, Node::ClassExpression(_)));
}

#[test]
fn test_control_flow_nodes() {
    // Switch statement
    let discriminant = create_identifier("value");
    let cases = vec![
        create_switch_case(Some(create_number(1.0)), vec![create_identifier("case1")]),
        create_switch_case(None, vec![create_identifier("default")]),
    ];
    let switch_stmt = create_switch_statement(discriminant, cases);
    
    assert!(matches!(switch_stmt, Node::SwitchStatement(stmt) if stmt.cases.len() == 2));
    
    // Try statement
    let try_stmt = create_try_statement(
        create_block_statement(vec![]),
        Some(create_catch_clause(create_identifier("error"), create_block_statement(vec![]))),
        Some(create_block_statement(vec![]))
    );
    
    assert!(matches!(try_stmt, Node::TryStatement(_)));
    
    // Throw statement
    let throw_stmt = create_throw_statement(create_identifier("error"));
    
    assert!(matches!(throw_stmt, Node::ThrowStatement(_)));
}

#[test]
fn test_es6_features() {
    // Spread element
    let spread = create_spread_element(create_identifier("array"));
    
    assert!(matches!(spread, Node::SpreadElement(_)));
    
    // Rest element
    let rest = create_rest_element(create_identifier("args"));
    
    assert!(matches!(rest, Node::RestElement(_)));
    
    // Super
    let super_expr = create_super();
    
    assert!(matches!(super_expr, Node::Super(_)));
    
    // Meta property
    let meta_prop = create_meta_property(create_identifier("new"), create_identifier("target"));
    
    assert!(matches!(meta_prop, Node::MetaProperty(_)));
    
    // Yield expression
    let yield_expr = create_yield_expression(Some(create_identifier("value")), false);
    
    assert!(matches!(yield_expr, Node::YieldExpression(expr) if expr.delegate == false));
    
    // Await expression
    let await_expr = create_await_expression(create_identifier("promise"));
    
    assert!(matches!(await_expr, Node::AwaitExpression(_)));
}

#[test]
fn test_module_nodes() {
    // Import declaration
    let specifiers = vec![
        create_import_specifier(create_identifier("local"), create_identifier("imported")),
    ];
    let source = create_string("module");
    let import_decl = create_import_declaration(specifiers, source);
    
    assert!(matches!(import_decl, Node::ImportDeclaration(decl) if decl.specifiers.len() == 1));
    
    // Import specifier - these are handled within ImportDeclaration
    let import_spec = create_import_specifier(create_identifier("local"), create_identifier("imported"));
    
    // Import default specifier - these are handled within ImportDeclaration
    let import_default = create_import_default_specifier(create_identifier("default"));
    
    // Import namespace specifier - these are handled within ImportDeclaration
    let import_namespace = create_import_namespace_specifier(create_identifier("namespace"));
    
    // Export declaration
    let export_decl = create_export_declaration(
        Some(create_identifier("exported")),
        vec![],
        None,
        false
    );
    
    assert!(matches!(export_decl, Node::ExportDeclaration(decl) if decl.default == false));
    
    // Export specifier - these are handled within ExportDeclaration
    let export_spec = create_export_specifier(create_identifier("local"), create_identifier("exported"));
}

#[test]
fn test_program_node() {
    let body = vec![
        create_variable_declaration("let", "x", Some(create_number(42.0))),
        create_function_declaration("fn", vec![], create_block_statement(vec![])),
    ];
    let program = create_program(body);
    
    assert!(matches!(program, Node::Program(prog) if prog.body.len() == 2 && prog.source_type == "script"));
}

#[test]
fn test_node_equality() {
    let node1 = create_identifier("x");
    let node2 = create_identifier("x");
    let node3 = create_identifier("y");
    
    assert_eq!(node1, node2);
    assert_ne!(node1, node3);
    
    let num1 = create_number(42.0);
    let num2 = create_number(42.0);
    let num3 = create_number(43.0);
    
    assert_eq!(num1, num2);
    assert_ne!(num1, num3);
}

#[test]
fn test_node_cloning() {
    let original = create_binary_expression(
        create_identifier("a"),
        "+",
        create_identifier("b")
    );
    
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_complex_nested_structure() {
    // Create a complex nested AST structure
    let program = create_program(vec![
        // Variable declaration
        create_variable_declaration("const", "result", Some(
            create_binary_expression(
                create_call_expression(
                    create_member_expression(
                        create_identifier("Math"),
                        create_identifier("pow"),
                        false
                    ),
                    vec![create_number(2.0), create_number(3.0)]
                ),
                "+",
                create_conditional_expression(
                    create_identifier("condition"),
                    create_string("true"),
                    create_string("false")
                )
            )
        )),
        // Function declaration
        create_function_declaration("calculate", vec![create_identifier("x")], 
            create_block_statement(vec![
                create_return_statement(Some(
                    create_binary_expression(
                        create_identifier("x"),
                        "*",
                        create_number(2.0)
                    )
                ))
            ])
        ),
        // If statement with nested expressions
        create_if_statement(
            create_logical_expression(
                create_identifier("a"),
                "&&",
                create_identifier("b")
            ),
            create_block_statement(vec![
                create_expression_statement(
                    create_assignment_expression(
                        create_identifier("result"),
                        "=",
                        create_number(1.0)
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
    ]);
    
    assert!(matches!(program, Node::Program(prog) if prog.body.len() == 3));
    
    // Test that we can access nested structures
    if let Node::Program(prog) = &program {
        if let Node::VariableDeclaration(var_decl) = &prog.body[0] {
            assert_eq!(var_decl.kind, "const");
            assert_eq!(var_decl.declarations.len(), 1);
        }
    }
} 