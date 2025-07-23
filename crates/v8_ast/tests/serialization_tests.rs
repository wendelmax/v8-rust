//! Tests for AST serialization and deserialization

use v8_ast::*;
use super::common::*;

#[test]
fn test_serialize_deserialize_literals() {
    // Test identifier
    let original_id = create_identifier("test");
    let serialized = serde_json::to_string(&original_id).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_id, deserialized);
    
    // Test number
    let original_num = create_number(42.5);
    let serialized = serde_json::to_string(&original_num).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_num, deserialized);
    
    // Test string
    let original_str = create_string("hello world");
    let serialized = serde_json::to_string(&original_str).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_str, deserialized);
    
    // Test boolean
    let original_bool = create_boolean(true);
    let serialized = serde_json::to_string(&original_bool).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_bool, deserialized);
    
    // Test null
    let original_null = create_null();
    let serialized = serde_json::to_string(&original_null).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_null, deserialized);
    
    // Test undefined
    let original_undefined = create_undefined();
    let serialized = serde_json::to_string(&original_undefined).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_undefined, deserialized);
    
    // Test this
    let original_this = create_this();
    let serialized = serde_json::to_string(&original_this).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_this, deserialized);
    
    // Test BigInt
    let original_bigint = create_bigint("42n");
    let serialized = serde_json::to_string(&original_bigint).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_bigint, deserialized);
    
    // Test RegExp
    let original_regexp = create_regexp("\\d+", "g");
    let serialized = serde_json::to_string(&original_regexp).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_regexp, deserialized);
}

#[test]
fn test_serialize_deserialize_expressions() {
    // Test binary expression
    let original_bin = create_binary_expression(
        create_identifier("a"),
        "+",
        create_identifier("b")
    );
    let serialized = serde_json::to_string(&original_bin).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_bin, deserialized);
    
    // Test unary expression
    let original_unary = create_unary_expression(
        "!",
        create_identifier("flag"),
        true
    );
    let serialized = serde_json::to_string(&original_unary).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_unary, deserialized);
    
    // Test call expression
    let original_call = create_call_expression(
        create_identifier("fn"),
        vec![create_number(1.0), create_string("arg")]
    );
    let serialized = serde_json::to_string(&original_call).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_call, deserialized);
    
    // Test member expression
    let original_member = create_member_expression(
        create_identifier("obj"),
        create_identifier("prop"),
        false
    );
    let serialized = serde_json::to_string(&original_member).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_member, deserialized);
    
    // Test assignment expression
    let original_assign = create_assignment_expression(
        create_identifier("x"),
        "=",
        create_number(42.0)
    );
    let serialized = serde_json::to_string(&original_assign).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_assign, deserialized);
    
    // Test conditional expression
    let original_cond = create_conditional_expression(
        create_identifier("test"),
        create_string("true"),
        create_string("false")
    );
    let serialized = serde_json::to_string(&original_cond).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_cond, deserialized);
    
    // Test logical expression
    let original_logical = create_logical_expression(
        create_identifier("a"),
        "&&",
        create_identifier("b")
    );
    let serialized = serde_json::to_string(&original_logical).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_logical, deserialized);
    
    // Test update expression
    let original_update = create_update_expression(
        "++",
        create_identifier("x"),
        false
    );
    let serialized = serde_json::to_string(&original_update).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_update, deserialized);
}

#[test]
fn test_serialize_deserialize_statements() {
    // Test variable declaration
    let original_var = create_variable_declaration(
        "const",
        "x",
        Some(create_number(42.0))
    );
    let serialized = serde_json::to_string(&original_var).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_var, deserialized);
    
    // Test function declaration
    let original_func = create_function_declaration(
        "fn",
        vec![create_identifier("param")],
        create_block_statement(vec![
            create_return_statement(Some(create_identifier("param")))
        ])
    );
    let serialized = serde_json::to_string(&original_func).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_func, deserialized);
    
    // Test block statement
    let original_block = create_block_statement(vec![
        create_identifier("stmt1"),
        create_identifier("stmt2"),
    ]);
    let serialized = serde_json::to_string(&original_block).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_block, deserialized);
    
    // Test if statement
    let original_if = create_if_statement(
        create_identifier("condition"),
        create_block_statement(vec![create_identifier("consequent")]),
        Some(create_block_statement(vec![create_identifier("alternate")]))
    );
    let serialized = serde_json::to_string(&original_if).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_if, deserialized);
    
    // Test while statement
    let original_while = create_while_statement(
        create_identifier("condition"),
        create_block_statement(vec![create_identifier("body")])
    );
    let serialized = serde_json::to_string(&original_while).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_while, deserialized);
    
    // Test for statement
    let original_for = create_for_statement(
        Some(create_variable_declaration("let", "i", Some(create_number(0.0)))),
        Some(create_binary_expression(create_identifier("i"), "<", create_number(10.0))),
        Some(create_update_expression("++", create_identifier("i"), false)),
        create_block_statement(vec![create_identifier("body")])
    );
    let serialized = serde_json::to_string(&original_for).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_for, deserialized);
    
    // Test return statement
    let original_return = create_return_statement(Some(create_identifier("value")));
    let serialized = serde_json::to_string(&original_return).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_return, deserialized);
    
    // Test break statement
    let original_break = create_break_statement(Some(create_identifier("label")));
    let serialized = serde_json::to_string(&original_break).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_break, deserialized);
    
    // Test continue statement
    let original_continue = create_continue_statement(Some(create_identifier("label")));
    let serialized = serde_json::to_string(&original_continue).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_continue, deserialized);
    
    // Test labeled statement
    let original_labeled = create_labeled_statement(
        create_identifier("label"),
        create_block_statement(vec![create_identifier("body")])
    );
    let serialized = serde_json::to_string(&original_labeled).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_labeled, deserialized);
    
    // Test debugger statement
    let original_debugger = create_debugger_statement();
    let serialized = serde_json::to_string(&original_debugger).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_debugger, deserialized);
    
    // Test expression statement
    let original_expr_stmt = create_expression_statement(create_identifier("expr"));
    let serialized = serde_json::to_string(&original_expr_stmt).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_expr_stmt, deserialized);
    
    // Test with statement
    let original_with = create_with_statement(
        create_identifier("obj"),
        create_block_statement(vec![create_identifier("body")])
    );
    let serialized = serde_json::to_string(&original_with).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_with, deserialized);
}

#[test]
fn test_serialize_deserialize_literals_with_spans() {
    // Test with spans
    let span = create_span(1, 1, 1, 10);
    
    let mut binary_expr = BinaryExpression {
        left: Box::new(create_identifier("a")),
        operator: "+".to_string(),
        right: Box::new(create_identifier("b")),
        span: Some(span.clone()),
    };
    
    let original = Node::BinaryExpression(binary_expr);
    let serialized = serde_json::to_string(&original).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_serialize_deserialize_complex_structure() {
    // Create a complex AST structure
    let original = create_program(vec![
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
    
    let serialized = serde_json::to_string(&original).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_serialize_deserialize_arrays_and_objects() {
    // Test array literal
    let original_array = create_array_literal(vec![
        Some(create_number(1.0)),
        Some(create_string("two")),
        Some(create_boolean(true)),
        None, // hole
    ]);
    let serialized = serde_json::to_string(&original_array).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_array, deserialized);
    
    // Test object literal
    let original_object = create_object_literal(vec![
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
    ]);
    let serialized = serde_json::to_string(&original_object).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_object, deserialized);
    
    // Test property
    let original_property = create_property(
        create_identifier("key"),
        create_string("value"),
        "init"
    );
    let serialized = serde_json::to_string(&original_property).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_property, deserialized);
}

#[test]
fn test_serialize_deserialize_es6_features() {
    // Test template literal
    let original_template = create_template_literal(
        vec![
            create_template_element("Hello ", false),
            create_template_element("!", true),
        ],
        vec![create_identifier("name")]
    );
    let serialized = serde_json::to_string(&original_template).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_template, deserialized);
    
    // Test tagged template expression
    let original_tagged = create_tagged_template_expression(
        create_identifier("tag"),
        create_template_literal(vec![], vec![])
    );
    let serialized = serde_json::to_string(&original_tagged).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_tagged, deserialized);
    
    // Test spread element
    let original_spread = create_spread_element(create_identifier("array"));
    let serialized = serde_json::to_string(&original_spread).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_spread, deserialized);
    
    // Test rest element
    let original_rest = create_rest_element(create_identifier("args"));
    let serialized = serde_json::to_string(&original_rest).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_rest, deserialized);
    
    // Test arrow function expression
    let original_arrow = create_arrow_function_expression(
        vec![create_identifier("x")],
        create_identifier("x"),
        true
    );
    let serialized = serde_json::to_string(&original_arrow).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_arrow, deserialized);
    
    // Test class declaration
    let original_class = create_class_declaration(
        Some(create_identifier("MyClass")),
        Some(create_identifier("Parent")),
        create_block_statement(vec![])
    );
    let serialized = serde_json::to_string(&original_class).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_class, deserialized);
    
    // Test yield expression
    let original_yield = create_yield_expression(Some(create_identifier("value")), false);
    let serialized = serde_json::to_string(&original_yield).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_yield, deserialized);
    
    // Test await expression
    let original_await = create_await_expression(create_identifier("promise"));
    let serialized = serde_json::to_string(&original_await).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_await, deserialized);
}

#[test]
fn test_serialize_deserialize_modules() {
    // Test import declaration
    let original_import = create_import_declaration(
        vec![
            create_import_specifier(create_identifier("local"), create_identifier("imported")),
        ],
        create_string("module")
    );
    let serialized = serde_json::to_string(&original_import).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_import, deserialized);
    
    // Test export declaration
    let original_export = create_export_declaration(
        Some(create_identifier("exported")),
        vec![],
        None,
        false
    );
    let serialized = serde_json::to_string(&original_export).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_export, deserialized);
}

#[test]
fn test_serialize_deserialize_control_flow() {
    // Test switch statement
    let original_switch = create_switch_statement(
        create_identifier("value"),
        vec![
            create_switch_case(Some(create_number(1.0)), vec![create_identifier("case1")]),
            create_switch_case(None, vec![create_identifier("default")]),
        ]
    );
    let serialized = serde_json::to_string(&original_switch).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_switch, deserialized);
    
    // Test try statement
    let original_try = create_try_statement(
        create_block_statement(vec![]),
        Some(create_catch_clause(create_identifier("error"), create_block_statement(vec![]))),
        Some(create_block_statement(vec![]))
    );
    let serialized = serde_json::to_string(&original_try).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_try, deserialized);
    
    // Test throw statement
    let original_throw = create_throw_statement(create_identifier("error"));
    let serialized = serde_json::to_string(&original_throw).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_throw, deserialized);
    
    // Test do-while statement
    let original_do_while = create_do_while_statement(
        create_block_statement(vec![create_identifier("body")]),
        create_identifier("condition")
    );
    let serialized = serde_json::to_string(&original_do_while).unwrap();
    let deserialized: Node = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original_do_while, deserialized);
}

#[test]
fn test_serialize_deserialize_error_handling() {
    // Test with invalid JSON
    let invalid_json = r#"{"type": "invalid"}"#;
    let result: Result<Node, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
    
    // Test with missing required fields
    let incomplete_json = r#"{"BinaryExpression": {"operator": "+"}}"#;
    let result: Result<Node, _> = serde_json::from_str(incomplete_json);
    assert!(result.is_err());
}

#[test]
fn test_serialize_pretty_print() {
    let ast = create_program(vec![
        create_variable_declaration("const", "x", Some(create_number(42.0))),
        create_function_declaration("fn", vec![], create_block_statement(vec![])),
    ]);
    
    let pretty = serde_json::to_string_pretty(&ast).unwrap();
    assert!(pretty.contains("const"));
    assert!(pretty.contains("42"));
    assert!(pretty.contains("fn"));
} 