//! Bytecode generator: Transforms AST into bytecode instructions

use crate::instructions::*;
use v8_ast::Node;

/// Main struct for bytecode generation
pub struct BytecodeGenerator {
    pub constants: ConstantPool,
    pub instructions: Vec<Instruction>,
}

impl BytecodeGenerator {
    /// Creates a new BytecodeGenerator
    pub fn new() -> Self {
        BytecodeGenerator {
            constants: ConstantPool::default(),
            instructions: Vec::new(),
        }
    }

    /// Generates bytecode from the given AST node
    pub fn generate(&mut self, node: &Node) {
        self.visit_node(node);
    }

    fn visit_node(&mut self, node: &Node) {
        match node {
            // Program structure
            Node::Program(program) => {
                for stmt in &program.body {
                    self.visit_node(stmt);
                }
            }
            // Declarations
            Node::VariableDeclaration(decl) => {
                for var in &decl.declarations {
                    self.visit_node(&var.id);
                    if let Some(init) = &var.init {
                        self.visit_node(init);
                        self.instructions.push(Instruction::StoreLocal(0)); // Exemplo
                    }
                }
            }
            Node::FunctionDeclaration(decl) => {
                if let Some(id) = &decl.id {
                    self.visit_node(id);
                }
                for param in &decl.params {
                    self.visit_node(param);
                }
                self.visit_node(&decl.body);
                // Instrução de função
            }
            Node::ClassDeclaration(decl) => {
                // Para simplificação, apenas empilha o nome da classe (ou None)
                if let Some(id) = &decl.id {
                    self.visit_node(id);
                }
                if let Some(super_class) = &decl.super_class {
                    self.visit_node(super_class);
                }
                self.visit_node(&decl.body);
                self.instructions.push(Instruction::NewClass);
            }
            Node::ImportDeclaration(_)
            | Node::ExportDeclaration(_) => {
                // Import/export não geram bytecode diretamente (runtime/host)
                // Placeholder: nenhuma instrução
            }
            Node::ClassExpression(expr) => {
                if let Some(id) = &expr.id {
                    self.visit_node(id);
                }
                if let Some(super_class) = &expr.super_class {
                    self.visit_node(super_class);
                }
                self.visit_node(&expr.body);
                self.instructions.push(Instruction::NewClass);
            }
            Node::YieldExpression(expr) => {
                if let Some(arg) = &expr.argument {
                    self.visit_node(arg);
                }
                self.instructions.push(Instruction::Yield);
            }
            Node::AwaitExpression(expr) => {
                self.visit_node(&expr.argument);
                self.instructions.push(Instruction::Await);
            }
            Node::SwitchStatement(stmt) => {
                self.visit_node(&stmt.discriminant);
                for case in &stmt.cases {
                    if let Some(test) = &case.test {
                        self.visit_node(test);
                    }
                    for cons in &case.consequent {
                        self.visit_node(cons);
                    }
                }
                // Placeholder: controle de fluxo real pode ser expandido
            }
            Node::TryStatement(stmt) => {
                self.visit_node(&stmt.block);
                if let Some(handler) = &stmt.handler {
                    self.visit_node(handler);
                }
                if let Some(finalizer) = &stmt.finalizer {
                    self.visit_node(finalizer);
                }
                self.instructions.push(Instruction::Try(0, 0)); // Placeholder
            }
            Node::CatchClause(clause) => {
                self.visit_node(&clause.param);
                self.visit_node(&clause.body);
                self.instructions.push(Instruction::Catch);
            }
            Node::ThrowStatement(stmt) => {
                self.visit_node(&stmt.argument);
                self.instructions.push(Instruction::Throw);
            }
            Node::ReturnStatement(stmt) => {
                if let Some(arg) = &stmt.argument {
                    self.visit_node(arg);
                }
                self.instructions.push(Instruction::Return);
            }
            Node::BreakStatement(_) => {
                self.instructions.push(Instruction::Jump(0)); // Placeholder
            }
            Node::ContinueStatement(_) => {
                self.instructions.push(Instruction::Jump(0)); // Placeholder
            }
            Node::LabeledStatement(stmt) => {
                self.visit_node(&stmt.label);
                self.visit_node(&stmt.body);
                // Placeholder: controle de fluxo real pode ser expandido
            }
            Node::WithStatement(stmt) => {
                self.visit_node(&stmt.object);
                self.visit_node(&stmt.body);
                // Placeholder: sem instrução específica
            }
            Node::DebuggerStatement(_) => {
                // Debugger: sem instrução específica
            }
            Node::TemplateLiteral(lit) => {
                for expr in &lit.expressions {
                    self.visit_node(expr);
                }
                // Placeholder: empilha strings/quasis
            }
            Node::TaggedTemplateExpression(expr) => {
                self.visit_node(&expr.tag);
                self.visit_node(&expr.quasi);
                // Placeholder
            }
            Node::Super(_) => {
                self.instructions.push(Instruction::LoadLocal(0)); // Placeholder para super
            }
            Node::MetaProperty(_) => {
                // Placeholder: sem instrução específica
            }
            Node::SpreadElement(elem) => {
                self.visit_node(&elem.argument);
                self.instructions.push(Instruction::Spread);
            }
            Node::RegExp(re) => {
                let idx = self.constants.add(Constant::String(re.pattern.clone()));
                self.instructions.push(Instruction::PushConst(idx));
            }
            Node::BigInt(val) => {
                let idx = self.constants.add(Constant::BigInt(val.clone()));
                self.instructions.push(Instruction::PushBigInt(idx));
            }
            // Expressions
            Node::BinaryExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
                match expr.operator.as_str() {
                    "+" => self.instructions.push(Instruction::Add),
                    "-" => self.instructions.push(Instruction::Sub),
                    "*" => self.instructions.push(Instruction::Mul),
                    "/" => self.instructions.push(Instruction::Div),
                    _ => unimplemented!("Operator {} not implemented", expr.operator),
                }
            }
            Node::UnaryExpression(expr) => {
                self.visit_node(&expr.argument);
                // Instrução unária
            }
            Node::CallExpression(expr) => {
                for arg in &expr.arguments {
                    self.visit_node(arg);
                }
                self.visit_node(&expr.callee);
                self.instructions.push(Instruction::Call(expr.arguments.len()));
            }
            Node::NewExpression(expr) => {
                for arg in &expr.arguments {
                    self.visit_node(arg);
                }
                self.visit_node(&expr.callee);
                self.instructions.push(Instruction::New);
            }
            Node::MemberExpression(expr) => {
                self.visit_node(&expr.object);
                self.visit_node(&expr.property);
                self.instructions.push(Instruction::GetProperty);
            }
            Node::AssignmentExpression(expr) => {
                self.visit_node(&expr.right);
                self.visit_node(&expr.left);
                self.instructions.push(Instruction::StoreLocal(0)); // Exemplo
            }
            Node::ConditionalExpression(expr) => {
                self.visit_node(&expr.test);
                // JumpIfFalse, consequent, alternate
                self.visit_node(&expr.consequent);
                self.visit_node(&expr.alternate);
            }
            Node::LogicalExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
                // Instrução lógica
            }
            Node::UpdateExpression(expr) => {
                self.visit_node(&expr.argument);
                // Instrução de update
            }
            Node::ArrowFunctionExpression(expr) => {
                for param in &expr.params {
                    self.visit_node(param);
                }
                self.visit_node(&expr.body);
                // Instrução de função (arrow)
            }
            Node::FunctionExpression(expr) => {
                if let Some(id) = &expr.id {
                    self.visit_node(id);
                }
                for param in &expr.params {
                    self.visit_node(param);
                }
                self.visit_node(&expr.body);
                // Instrução de função (function expression)
            }
            Node::ClassExpression(_)
            | Node::YieldExpression(_)
            | Node::AwaitExpression(_) => {
                // TODO: Implementar
                unimplemented!("Class/Yield/Await not implemented");
            }
            // Statements
            Node::BlockStatement(stmt) => {
                for node in &stmt.body {
                    self.visit_node(node);
                }
            }
            Node::IfStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.consequent);
                if let Some(alt) = &stmt.alternate {
                    self.visit_node(alt);
                }
            }
            Node::ForStatement(stmt) => {
                if let Some(init) = &stmt.init {
                    self.visit_node(init);
                }
                if let Some(test) = &stmt.test {
                    self.visit_node(test);
                }
                if let Some(update) = &stmt.update {
                    self.visit_node(update);
                }
                self.visit_node(&stmt.body);
            }
            Node::WhileStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.body);
            }
            Node::DoWhileStatement(stmt) => {
                self.visit_node(&stmt.body);
                self.visit_node(&stmt.test);
            }
            Node::ExpressionStatement(stmt) => {
                self.visit_node(&stmt.expression);
            }
            // Literals
            Node::ArrayLiteral(lit) => {
                for elem in &lit.elements {
                    if let Some(e) = elem {
                        self.visit_node(e);
                    }
                }
                self.instructions.push(Instruction::NewArray(lit.elements.len()));
            }
            Node::ObjectLiteral(lit) => {
                for prop in &lit.properties {
                    self.visit_node(prop);
                }
                self.instructions.push(Instruction::NewObject);
            }
            Node::TemplateLiteral(_)
            | Node::TaggedTemplateExpression(_) => {
                // TODO: Implementar
                unimplemented!("Template literal not implemented");
            }
            // Other
            Node::Property(prop) => {
                self.visit_node(&prop.key);
                self.visit_node(&prop.value);
                // Instrução de propriedade
            }
            Node::RestElement(elem) => {
                self.visit_node(&elem.argument);
                // Instrução de rest
            }
            Node::Super(_)
            | Node::MetaProperty(_)
            | Node::SpreadElement(_) => {
                // TODO: Implementar
                unimplemented!("Super/Meta/Spread not implemented");
            }
            Node::Identifier(_name) => {
                self.instructions.push(Instruction::LoadLocal(0));
            }
            Node::Number(n) => {
                let idx = self.constants.add(Constant::Number(*n));
                self.instructions.push(Instruction::PushConst(idx));
            }
            Node::String(s) => {
                let idx = self.constants.add(Constant::String(s.clone()));
                self.instructions.push(Instruction::PushConst(idx));
            }
            Node::Boolean(b) => {
                let idx = self.constants.add(Constant::Boolean(*b));
                self.instructions.push(Instruction::PushConst(idx));
            }
            Node::Null => {
                self.instructions.push(Instruction::PushNull);
            }
            Node::Undefined => {
                self.instructions.push(Instruction::PushUndefined);
            }
            Node::This => {
                self.instructions.push(Instruction::LoadLocal(0)); // Exemplo
            }
            Node::RegExp(_)
            | Node::BigInt(_) => {
                // TODO: Implementar
                unimplemented!("RegExp/BigInt not implemented");
            }
        }
    }
}

impl ConstantPool {
    pub fn add(&mut self, value: Constant) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }
} 