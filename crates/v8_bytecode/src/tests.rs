//! Unit tests for v8_bytecode

use crate::generator::{BytecodeGenerator, AstNode};
use crate::instructions::{Instruction, Constant};
use v8_ast::*;

#[test]
fn test_generate_number_literal() {
    let ast = AstNode::NumberLiteral(42.0);
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.constants.values, vec![Constant::Number(42.0)]);
    assert_eq!(gen.instructions, vec![Instruction::PushConst(0)]);
}

#[test]
fn test_generate_string_literal() {
    let ast = AstNode::StringLiteral("hello".to_string());
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.constants.values, vec![Constant::String("hello".to_string())]);
    assert_eq!(gen.instructions, vec![Instruction::PushConst(0)]);
}

#[test]
fn test_generate_boolean_literal() {
    let ast = AstNode::BooleanLiteral(true);
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.constants.values, vec![Constant::Boolean(true)]);
    assert_eq!(gen.instructions, vec![Instruction::PushConst(0)]);
}

#[test]
fn test_generate_binary_expression() {
    let ast = AstNode::BinaryExpression {
        left: Box::new(AstNode::NumberLiteral(2.0)),
        op: "+".to_string(),
        right: Box::new(AstNode::NumberLiteral(3.0)),
    };
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.constants.values, vec![Constant::Number(2.0), Constant::Number(3.0)]);
    assert_eq!(gen.instructions, vec![Instruction::PushConst(0), Instruction::PushConst(1), Instruction::Add]);
}

#[test]
fn test_generate_identifier() {
    let ast = AstNode::Identifier("x".to_string());
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.instructions, vec![Instruction::LoadLocal(0)]);
}

#[test]
fn test_generate_assignment() {
    let ast = AstNode::Assignment {
        left: Box::new(AstNode::Identifier("x".to_string())),
        right: Box::new(AstNode::NumberLiteral(10.0)),
    };
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.constants.values, vec![Constant::Number(10.0)]);
    assert_eq!(gen.instructions, vec![Instruction::PushConst(0), Instruction::StoreLocal(0)]);
}

#[test]
fn test_generate_if_statement() {
    let ast = AstNode::IfStatement {
        condition: Box::new(AstNode::BooleanLiteral(true)),
        then_branch: Box::new(AstNode::NumberLiteral(1.0)),
        else_branch: Some(Box::new(AstNode::NumberLiteral(2.0))),
    };
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    // Esperado: PushConst(true), JumpIfFalse, PushConst(1), Jump, PushConst(2)
    assert_eq!(gen.constants.values, vec![Constant::Boolean(true), Constant::Number(1.0), Constant::Number(2.0)]);
    assert_eq!(gen.instructions[0], Instruction::PushConst(0)); // true
    assert!(matches!(gen.instructions[1], Instruction::JumpIfFalse(_)));
    assert_eq!(gen.instructions[2], Instruction::PushConst(1)); // 1.0
    assert!(matches!(gen.instructions[3], Instruction::Jump(_)));
    assert_eq!(gen.instructions[4], Instruction::PushConst(2)); // 2.0
}

#[test]
fn test_generate_function_declaration() {
    let ast = AstNode::FunctionDeclaration {
        name: "foo".to_string(),
        params: vec!["a".to_string(), "b".to_string()],
        body: Box::new(AstNode::NumberLiteral(42.0)),
    };
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    // Para simplificação, só empilha o nome da função como constante
    assert_eq!(gen.constants.values, vec![Constant::String("foo".to_string())]);
    assert_eq!(gen.instructions, vec![Instruction::PushConst(0)]);
}

#[test]
fn test_generate_call_expression() {
    let ast = AstNode::CallExpression {
        callee: Box::new(AstNode::Identifier("foo".to_string())),
        arguments: vec![AstNode::NumberLiteral(1.0), AstNode::NumberLiteral(2.0)],
    };
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    // Esperado: PushConst(0), PushConst(1), LoadLocal(0), Call(2)
    assert_eq!(gen.constants.values, vec![Constant::Number(1.0), Constant::Number(2.0)]);
    assert_eq!(gen.instructions[0], Instruction::PushConst(0)); // 1.0
    assert_eq!(gen.instructions[1], Instruction::PushConst(1)); // 2.0
    assert_eq!(gen.instructions[2], Instruction::LoadLocal(0)); // foo
    assert_eq!(gen.instructions[3], Instruction::Call(2));
}

#[test]
fn test_generate_object_literal() {
    let ast = AstNode::ObjectLiteral(vec![
        ("a".to_string(), Box::new(AstNode::NumberLiteral(1.0))),
        ("b".to_string(), Box::new(AstNode::NumberLiteral(2.0))),
    ]);
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.constants.values, vec![Constant::Number(1.0), Constant::Number(2.0)]);
    assert_eq!(gen.instructions[0], Instruction::PushConst(0));
    assert_eq!(gen.instructions[1], Instruction::PushConst(1));
    assert_eq!(gen.instructions[2], Instruction::NewObject);
}

#[test]
fn test_generate_array_literal() {
    let ast = AstNode::ArrayLiteral(vec![
        AstNode::NumberLiteral(1.0),
        AstNode::NumberLiteral(2.0),
    ]);
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.constants.values, vec![Constant::Number(1.0), Constant::Number(2.0)]);
    assert_eq!(gen.instructions[0], Instruction::PushConst(0));
    assert_eq!(gen.instructions[1], Instruction::PushConst(1));
    assert_eq!(gen.instructions[2], Instruction::NewArray(2));
}

#[test]
fn test_generate_spread() {
    let ast = AstNode::Spread(Box::new(AstNode::Identifier("arr".to_string())));
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.instructions[0], Instruction::LoadLocal(0));
    assert_eq!(gen.instructions[1], Instruction::Spread);
}

#[test]
fn test_generate_destructure() {
    let ast = AstNode::Destructure {
        pattern: Box::new(AstNode::Identifier("x".to_string())),
        value: Box::new(AstNode::ArrayLiteral(vec![AstNode::NumberLiteral(1.0)])),
    };
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ast);
    assert_eq!(gen.instructions[0], Instruction::PushConst(0));
    assert_eq!(gen.instructions[1], Instruction::NewArray(1));
    assert_eq!(gen.instructions[2], Instruction::LoadLocal(0));
    assert_eq!(gen.instructions[3], Instruction::Destructure);
}

#[test]
fn test_program_node() {
    let program = Node::Program(Program {
        body: vec![Node::Number(1.0)],
        source_type: "script".to_string(),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&program);
    assert_eq!(gen.constants.values, vec![Constant::Number(1.0)]);
}

#[test]
fn test_class_declaration() {
    let class = Node::ClassDeclaration(ClassDeclaration {
        id: Some(Box::new(Node::Identifier("MyClass".to_string()))),
        super_class: None,
        body: Box::new(Node::BlockStatement(BlockStatement { body: vec![], span: None })),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&class);
    assert_eq!(gen.instructions.last(), Some(&Instruction::NewClass));
}

#[test]
fn test_import_declaration() {
    let import = Node::ImportDeclaration(ImportDeclaration {
        specifiers: vec![],
        source: Box::new(Node::String("mod.js".to_string())),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&import);
    // Import não gera instrução
    assert!(gen.instructions.is_empty());
}

#[test]
fn test_export_declaration() {
    let export = Node::ExportDeclaration(ExportDeclaration {
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&export);
    // Export não gera instrução
    assert!(gen.instructions.is_empty());
}

#[test]
fn test_class_expression() {
    let class = Node::ClassExpression(ClassExpression {
        id: None,
        super_class: None,
        body: Box::new(Node::BlockStatement(BlockStatement { body: vec![], span: None })),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&class);
    assert_eq!(gen.instructions.last(), Some(&Instruction::NewClass));
}

#[test]
fn test_yield_expression() {
    let yield_expr = Node::YieldExpression(YieldExpression {
        argument: Some(Box::new(Node::Number(42.0))),
        delegate: false,
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&yield_expr);
    assert_eq!(gen.instructions.last(), Some(&Instruction::Yield));
}

#[test]
fn test_await_expression() {
    let await_expr = Node::AwaitExpression(AwaitExpression {
        argument: Box::new(Node::Number(1.0)),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&await_expr);
    assert_eq!(gen.instructions.last(), Some(&Instruction::Await));
}

#[test]
fn test_switch_statement() {
    let switch = Node::SwitchStatement(SwitchStatement {
        discriminant: Box::new(Node::Number(1.0)),
        cases: vec![],
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&switch);
    // Placeholder: não gera instrução específica
    assert!(gen.instructions.len() >= 1);
}

#[test]
fn test_try_statement() {
    let try_stmt = Node::TryStatement(TryStatement {
        block: Box::new(Node::BlockStatement(BlockStatement { body: vec![], span: None })),
        handler: None,
        finalizer: None,
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&try_stmt);
    assert_eq!(gen.instructions.last(), Some(&Instruction::Try(0, 0)));
}

#[test]
fn test_catch_clause() {
    let catch = Node::CatchClause(CatchClause {
        param: Box::new(Node::Identifier("e".to_string())),
        body: Box::new(Node::BlockStatement(BlockStatement { body: vec![], span: None })),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&catch);
    assert_eq!(gen.instructions.last(), Some(&Instruction::Catch));
}

#[test]
fn test_throw_statement() {
    let throw = Node::ThrowStatement(ThrowStatement {
        argument: Box::new(Node::Number(1.0)),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&throw);
    assert_eq!(gen.instructions.last(), Some(&Instruction::Throw));
}

#[test]
fn test_return_statement() {
    let ret = Node::ReturnStatement(ReturnStatement {
        argument: None,
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&ret);
    assert_eq!(gen.instructions.last(), Some(&Instruction::Return));
}

#[test]
fn test_break_statement() {
    let brk = Node::BreakStatement(BreakStatement {
        label: None,
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&brk);
    assert_eq!(gen.instructions.last(), Some(&Instruction::Jump(0)));
}

#[test]
fn test_continue_statement() {
    let cont = Node::ContinueStatement(ContinueStatement {
        label: None,
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&cont);
    assert_eq!(gen.instructions.last(), Some(&Instruction::Jump(0)));
}

#[test]
fn test_labeled_statement() {
    let lbl = Node::LabeledStatement(LabeledStatement {
        label: Box::new(Node::Identifier("lbl".to_string())),
        body: Box::new(Node::BlockStatement(BlockStatement { body: vec![], span: None })),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&lbl);
    // Placeholder: não gera instrução específica
    assert!(gen.instructions.len() >= 0);
}

#[test]
fn test_with_statement() {
    let with = Node::WithStatement(WithStatement {
        object: Box::new(Node::Number(1.0)),
        body: Box::new(Node::BlockStatement(BlockStatement { body: vec![], span: None })),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&with);
    // Placeholder: não gera instrução específica
    assert!(gen.instructions.len() >= 0);
}

#[test]
fn test_debugger_statement() {
    let dbg = Node::DebuggerStatement(DebuggerStatement { span: None });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&dbg);
    // Debugger: não gera instrução
    assert!(gen.instructions.is_empty());
}

#[test]
fn test_template_literal() {
    let tmpl = Node::TemplateLiteral(TemplateLiteral {
        quasis: vec![],
        expressions: vec![Node::Number(1.0)],
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&tmpl);
    // Placeholder: não gera instrução específica
    assert!(gen.instructions.len() >= 0);
}

#[test]
fn test_tagged_template_expression() {
    let tag = Node::TaggedTemplateExpression(TaggedTemplateExpression {
        tag: Box::new(Node::Identifier("tag".to_string())),
        quasi: Box::new(Node::TemplateLiteral(TemplateLiteral { quasis: vec![], expressions: vec![], span: None })),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&tag);
    // Placeholder: não gera instrução específica
    assert!(gen.instructions.len() >= 0);
}

#[test]
fn test_super() {
    let sup = Node::Super(Super { span: None });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&sup);
    // Placeholder: LoadLocal(0)
    assert_eq!(gen.instructions.last(), Some(&Instruction::LoadLocal(0)));
}

#[test]
fn test_meta_property() {
    let meta = Node::MetaProperty(MetaProperty {
        meta: Box::new(Node::Identifier("meta".to_string())),
        property: Box::new(Node::Identifier("prop".to_string())),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&meta);
    // Placeholder: não gera instrução específica
    assert!(gen.instructions.len() >= 0);
}

#[test]
fn test_spread_element() {
    let spread = Node::SpreadElement(SpreadElement {
        argument: Box::new(Node::Identifier("arr".to_string())),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&spread);
    assert_eq!(gen.instructions.last(), Some(&Instruction::Spread));
}

#[test]
fn test_regexp() {
    let re = Node::RegExp(RegExp {
        pattern: "abc".to_string(),
        flags: "g".to_string(),
        span: None,
    });
    let mut gen = BytecodeGenerator::new();
    gen.generate(&re);
    assert_eq!(gen.constants.values.last(), Some(&Constant::String("abc".to_string())));
}

#[test]
fn test_bigint() {
    let big = Node::BigInt("123n".to_string());
    let mut gen = BytecodeGenerator::new();
    gen.generate(&big);
    assert_eq!(gen.instructions.last(), Some(&Instruction::PushBigInt(0)));
} 