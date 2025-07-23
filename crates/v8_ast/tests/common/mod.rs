//! Common utilities for AST tests

use v8_ast::*;

/// Helper function to create a simple identifier node
pub fn create_identifier(name: &str) -> Node {
    Node::Identifier(name.to_string())
}

/// Helper function to create a number literal node
pub fn create_number(value: f64) -> Node {
    Node::Number(value)
}

/// Helper function to create a string literal node
pub fn create_string(value: &str) -> Node {
    Node::String(value.to_string())
}

/// Helper function to create a boolean literal node
pub fn create_boolean(value: bool) -> Node {
    Node::Boolean(value)
}

/// Helper function to create a span
pub fn create_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
    Span::from_positions(start_line, start_col, end_line, end_col)
}

/// Helper function to create a binary expression
pub fn create_binary_expression(left: Node, operator: &str, right: Node) -> Node {
    Node::BinaryExpression(BinaryExpression {
        left: Box::new(left),
        operator: operator.to_string(),
        right: Box::new(right),
        span: None,
    })
}

/// Helper function to create a variable declaration
pub fn create_variable_declaration(kind: &str, name: &str, init: Option<Node>) -> Node {
    Node::VariableDeclaration(VariableDeclaration {
        kind: kind.to_string(),
        declarations: vec![VariableDeclarator {
            id: Box::new(create_identifier(name)),
            init: init.map(Box::new),
            span: None,
        }],
        span: None,
    })
}

/// Helper function to create a function declaration
pub fn create_function_declaration(name: &str, params: Vec<Node>, body: Node) -> Node {
    Node::FunctionDeclaration(FunctionDeclaration {
        id: Some(Box::new(create_identifier(name))),
        params,
        body: Box::new(body),
        generator: false,
        r#async: false,
        span: None,
    })
}

/// Helper function to create a block statement
pub fn create_block_statement(body: Vec<Node>) -> Node {
    Node::BlockStatement(BlockStatement {
        body,
        span: None,
    })
}

/// Helper function to create an if statement
pub fn create_if_statement(test: Node, consequent: Node, alternate: Option<Node>) -> Node {
    Node::IfStatement(IfStatement {
        test: Box::new(test),
        consequent: Box::new(consequent),
        alternate: alternate.map(Box::new),
        span: None,
    })
}

/// Helper function to create a program
pub fn create_program(body: Vec<Node>) -> Node {
    Node::Program(Program {
        body,
        source_type: "script".to_string(),
        span: None,
    })
}

/// Helper function to create an array literal
pub fn create_array_literal(elements: Vec<Option<Node>>) -> Node {
    Node::ArrayLiteral(ArrayLiteral {
        elements,
        span: None,
    })
}

/// Helper function to create an object literal
pub fn create_object_literal(properties: Vec<Node>) -> Node {
    Node::ObjectLiteral(ObjectLiteral {
        properties,
        span: None,
    })
}

/// Helper function to create a property
pub fn create_property(key: Node, value: Node, kind: &str) -> Node {
    Node::Property(Property {
        key: Box::new(key),
        value: Box::new(value),
        kind: kind.to_string(),
        computed: false,
        method: false,
        shorthand: false,
        span: None,
    })
}

/// Helper function to create a call expression
pub fn create_call_expression(callee: Node, arguments: Vec<Node>) -> Node {
    Node::CallExpression(CallExpression {
        callee: Box::new(callee),
        arguments,
        span: None,
    })
}

/// Helper function to create a member expression
pub fn create_member_expression(object: Node, property: Node, computed: bool) -> Node {
    Node::MemberExpression(MemberExpression {
        object: Box::new(object),
        property: Box::new(property),
        computed,
        optional: false,
        span: None,
    })
}

/// Helper function to create an assignment expression
pub fn create_assignment_expression(left: Node, operator: &str, right: Node) -> Node {
    Node::AssignmentExpression(AssignmentExpression {
        left: Box::new(left),
        operator: operator.to_string(),
        right: Box::new(right),
        span: None,
    })
}

/// Helper function to create a return statement
pub fn create_return_statement(argument: Option<Node>) -> Node {
    Node::ReturnStatement(ReturnStatement {
        argument: argument.map(Box::new),
        span: None,
    })
}

/// Helper function to create a while statement
pub fn create_while_statement(test: Node, body: Node) -> Node {
    Node::WhileStatement(WhileStatement {
        test: Box::new(test),
        body: Box::new(body),
        span: None,
    })
}

/// Helper function to create a for statement
pub fn create_for_statement(init: Option<Node>, test: Option<Node>, update: Option<Node>, body: Node) -> Node {
    Node::ForStatement(ForStatement {
        init: init.map(Box::new),
        test: test.map(Box::new),
        update: update.map(Box::new),
        body: Box::new(body),
        span: None,
    })
}

/// Helper function to create a switch statement
pub fn create_switch_statement(discriminant: Node, cases: Vec<SwitchCase>) -> Node {
    Node::SwitchStatement(SwitchStatement {
        discriminant: Box::new(discriminant),
        cases,
        span: None,
    })
}

/// Helper function to create a switch case
pub fn create_switch_case(test: Option<Node>, consequent: Vec<Node>) -> SwitchCase {
    SwitchCase {
        test: test.map(Box::new),
        consequent,
        span: None,
    }
}

/// Helper function to create a try statement
pub fn create_try_statement(block: Node, handler: Option<Node>, finalizer: Option<Node>) -> Node {
    Node::TryStatement(TryStatement {
        block: Box::new(block),
        handler: handler.map(Box::new),
        finalizer: finalizer.map(Box::new),
        span: None,
    })
}

/// Helper function to create a catch clause
pub fn create_catch_clause(param: Node, body: Node) -> Node {
    Node::CatchClause(CatchClause {
        param: Box::new(param),
        body: Box::new(body),
        span: None,
    })
}

/// Helper function to create a throw statement
pub fn create_throw_statement(argument: Node) -> Node {
    Node::ThrowStatement(ThrowStatement {
        argument: Box::new(argument),
        span: None,
    })
}

/// Helper function to create a break statement
pub fn create_break_statement(label: Option<Node>) -> Node {
    Node::BreakStatement(BreakStatement {
        label: label.map(Box::new),
        span: None,
    })
}

/// Helper function to create a continue statement
pub fn create_continue_statement(label: Option<Node>) -> Node {
    Node::ContinueStatement(ContinueStatement {
        label: label.map(Box::new),
        span: None,
    })
}

/// Helper function to create a labeled statement
pub fn create_labeled_statement(label: Node, body: Node) -> Node {
    Node::LabeledStatement(LabeledStatement {
        label: Box::new(label),
        body: Box::new(body),
        span: None,
    })
}

/// Helper function to create a debugger statement
pub fn create_debugger_statement() -> Node {
    Node::DebuggerStatement(DebuggerStatement {
        span: None,
    })
}

/// Helper function to create an expression statement
pub fn create_expression_statement(expression: Node) -> Node {
    Node::ExpressionStatement(ExpressionStatement {
        expression: Box::new(expression),
        span: None,
    })
}

/// Helper function to create a template literal
pub fn create_template_literal(quasis: Vec<TemplateElement>, expressions: Vec<Node>) -> Node {
    Node::TemplateLiteral(TemplateLiteral {
        quasis,
        expressions,
        span: None,
    })
}

/// Helper function to create a template element
pub fn create_template_element(value: &str, tail: bool) -> TemplateElement {
    TemplateElement {
        value: value.to_string(),
        tail,
        span: None,
    }
}

/// Helper function to create a tagged template expression
pub fn create_tagged_template_expression(tag: Node, quasi: Node) -> Node {
    Node::TaggedTemplateExpression(TaggedTemplateExpression {
        tag: Box::new(tag),
        quasi: Box::new(quasi),
        span: None,
    })
}

/// Helper function to create a spread element
pub fn create_spread_element(argument: Node) -> Node {
    Node::SpreadElement(SpreadElement {
        argument: Box::new(argument),
        span: None,
    })
}

/// Helper function to create a rest element
pub fn create_rest_element(argument: Node) -> Node {
    Node::RestElement(RestElement {
        argument: Box::new(argument),
        span: None,
    })
}

/// Helper function to create a super expression
pub fn create_super() -> Node {
    Node::Super(Super {
        span: None,
    })
}

/// Helper function to create a this expression
pub fn create_this() -> Node {
    Node::This
}

/// Helper function to create a null literal
pub fn create_null() -> Node {
    Node::Null
}

/// Helper function to create an undefined literal
pub fn create_undefined() -> Node {
    Node::Undefined
}

/// Helper function to create a BigInt literal
pub fn create_bigint(value: &str) -> Node {
    Node::BigInt(value.to_string())
}

/// Helper function to create a RegExp literal
pub fn create_regexp(pattern: &str, flags: &str) -> Node {
    Node::RegExp(RegExp {
        pattern: pattern.to_string(),
        flags: flags.to_string(),
        span: None,
    })
}

/// Helper function to create an arrow function expression
pub fn create_arrow_function_expression(params: Vec<Node>, body: Node, expression: bool) -> Node {
    Node::ArrowFunctionExpression(ArrowFunctionExpression {
        params,
        body: Box::new(body),
        expression,
        r#async: false,
        span: None,
    })
}

/// Helper function to create a function expression
pub fn create_function_expression(id: Option<Node>, params: Vec<Node>, body: Node) -> Node {
    Node::FunctionExpression(FunctionExpression {
        id: id.map(Box::new),
        params,
        body: Box::new(body),
        generator: false,
        r#async: false,
        span: None,
    })
}

/// Helper function to create a class declaration
pub fn create_class_declaration(id: Option<Node>, super_class: Option<Node>, body: Node) -> Node {
    Node::ClassDeclaration(ClassDeclaration {
        id: id.map(Box::new),
        super_class: super_class.map(Box::new),
        body: Box::new(body),
        span: None,
    })
}

/// Helper function to create a class expression
pub fn create_class_expression(id: Option<Node>, super_class: Option<Node>, body: Node) -> Node {
    Node::ClassExpression(ClassExpression {
        id: id.map(Box::new),
        super_class: super_class.map(Box::new),
        body: Box::new(body),
        span: None,
    })
}

/// Helper function to create a new expression
pub fn create_new_expression(callee: Node, arguments: Vec<Node>) -> Node {
    Node::NewExpression(NewExpression {
        callee: Box::new(callee),
        arguments,
        span: None,
    })
}

/// Helper function to create a conditional expression
pub fn create_conditional_expression(test: Node, consequent: Node, alternate: Node) -> Node {
    Node::ConditionalExpression(ConditionalExpression {
        test: Box::new(test),
        consequent: Box::new(consequent),
        alternate: Box::new(alternate),
        span: None,
    })
}

/// Helper function to create a logical expression
pub fn create_logical_expression(left: Node, operator: &str, right: Node) -> Node {
    Node::LogicalExpression(LogicalExpression {
        left: Box::new(left),
        operator: operator.to_string(),
        right: Box::new(right),
        span: None,
    })
}

/// Helper function to create an update expression
pub fn create_update_expression(operator: &str, argument: Node, prefix: bool) -> Node {
    Node::UpdateExpression(UpdateExpression {
        operator: operator.to_string(),
        argument: Box::new(argument),
        prefix,
        span: None,
    })
}

/// Helper function to create a unary expression
pub fn create_unary_expression(operator: &str, argument: Node, prefix: bool) -> Node {
    Node::UnaryExpression(UnaryExpression {
        operator: operator.to_string(),
        argument: Box::new(argument),
        prefix,
        span: None,
    })
}

/// Helper function to create a yield expression
pub fn create_yield_expression(argument: Option<Node>, delegate: bool) -> Node {
    Node::YieldExpression(YieldExpression {
        argument: argument.map(Box::new),
        delegate,
        span: None,
    })
}

/// Helper function to create an await expression
pub fn create_await_expression(argument: Node) -> Node {
    Node::AwaitExpression(AwaitExpression {
        argument: Box::new(argument),
        span: None,
    })
}

/// Helper function to create a meta property
pub fn create_meta_property(meta: Node, property: Node) -> Node {
    Node::MetaProperty(MetaProperty {
        meta: Box::new(meta),
        property: Box::new(property),
        span: None,
    })
}

/// Helper function to create an import declaration
pub fn create_import_declaration(specifiers: Vec<Node>, source: Node) -> Node {
    Node::ImportDeclaration(ImportDeclaration {
        specifiers,
        source: Box::new(source),
        span: None,
    })
}

/// Helper function to create an import specifier
pub fn create_import_specifier(local: Node, imported: Node) -> Node {
    Node::ImportSpecifier(ImportSpecifier {
        local: Box::new(local),
        imported: Box::new(imported),
        span: None,
    })
}

/// Helper function to create an import default specifier
pub fn create_import_default_specifier(local: Node) -> Node {
    Node::ImportDefaultSpecifier(ImportDefaultSpecifier {
        local: Box::new(local),
        span: None,
    })
}

/// Helper function to create an import namespace specifier
pub fn create_import_namespace_specifier(local: Node) -> Node {
    Node::ImportNamespaceSpecifier(ImportNamespaceSpecifier {
        local: Box::new(local),
        span: None,
    })
}

/// Helper function to create an export declaration
pub fn create_export_declaration(declaration: Option<Node>, specifiers: Vec<Node>, source: Option<Node>, default: bool) -> Node {
    Node::ExportDeclaration(ExportDeclaration {
        declaration: declaration.map(Box::new),
        specifiers,
        source: source.map(Box::new),
        default,
        span: None,
    })
}

/// Helper function to create an export specifier
pub fn create_export_specifier(local: Node, exported: Node) -> Node {
    Node::ExportSpecifier(ExportSpecifier {
        local: Box::new(local),
        exported: Box::new(exported),
        span: None,
    })
}

/// Helper function to create a do-while statement
pub fn create_do_while_statement(body: Node, test: Node) -> Node {
    Node::DoWhileStatement(DoWhileStatement {
        body: Box::new(body),
        test: Box::new(test),
        span: None,
    })
}

/// Helper function to create a with statement
pub fn create_with_statement(object: Node, body: Node) -> Node {
    Node::WithStatement(WithStatement {
        object: Box::new(object),
        body: Box::new(body),
        span: None,
    })
} 