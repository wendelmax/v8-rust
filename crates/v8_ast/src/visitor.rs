//! Visitor pattern for AST traversal

use crate::Node;

/// Visitor trait for AST traversal
pub trait Visitor {
    type Output;
    
    /// Visit a node
    fn visit_node(&mut self, node: &Node) -> Self::Output {
        match node {
            Node::Program(program) => self.visit_program(program),
            Node::VariableDeclaration(decl) => self.visit_variable_declaration(decl),
            Node::FunctionDeclaration(decl) => self.visit_function_declaration(decl),
            Node::ClassDeclaration(decl) => self.visit_class_declaration(decl),
            Node::BinaryExpression(expr) => self.visit_binary_expression(expr),
            Node::UnaryExpression(expr) => self.visit_unary_expression(expr),
            Node::CallExpression(expr) => self.visit_call_expression(expr),
            Node::NewExpression(expr) => self.visit_new_expression(expr),
            Node::MemberExpression(expr) => self.visit_member_expression(expr),
            Node::AssignmentExpression(expr) => self.visit_assignment_expression(expr),
            Node::ConditionalExpression(expr) => self.visit_conditional_expression(expr),
            Node::LogicalExpression(expr) => self.visit_logical_expression(expr),
            Node::UpdateExpression(expr) => self.visit_update_expression(expr),
            Node::BlockStatement(stmt) => self.visit_block_statement(stmt),
            Node::IfStatement(stmt) => self.visit_if_statement(stmt),
            Node::ForStatement(stmt) => self.visit_for_statement(stmt),
            Node::WhileStatement(stmt) => self.visit_while_statement(stmt),
            Node::DoWhileStatement(stmt) => self.visit_do_while_statement(stmt),
            Node::SwitchStatement(stmt) => self.visit_switch_statement(stmt),
            Node::TryStatement(stmt) => self.visit_try_statement(stmt),
            Node::ThrowStatement(stmt) => self.visit_throw_statement(stmt),
            Node::ReturnStatement(stmt) => self.visit_return_statement(stmt),
            Node::BreakStatement(stmt) => self.visit_break_statement(stmt),
            Node::ContinueStatement(stmt) => self.visit_continue_statement(stmt),
            Node::ExpressionStatement(stmt) => self.visit_expression_statement(stmt),
            Node::ArrayLiteral(lit) => self.visit_array_literal(lit),
            Node::ObjectLiteral(lit) => self.visit_object_literal(lit),
            Node::Property(prop) => self.visit_property(prop),
            Node::Identifier(id) => self.visit_identifier(id),
            Node::Number(num) => self.visit_number(*num),
            Node::String(s) => self.visit_string(s),
            Node::Boolean(b) => self.visit_boolean(*b),
            Node::Null => self.visit_null(),
            Node::Undefined => self.visit_undefined(),
            Node::This => self.visit_this(),
            // Add more cases as needed
            _ => self.visit_unknown(node),
        }
    }
    
    // Default implementations
    fn visit_program(&mut self, _program: &crate::Program) -> Self::Output { unimplemented!() }
    fn visit_variable_declaration(&mut self, _decl: &crate::VariableDeclaration) -> Self::Output { unimplemented!() }
    fn visit_function_declaration(&mut self, _decl: &crate::FunctionDeclaration) -> Self::Output { unimplemented!() }
    fn visit_class_declaration(&mut self, _decl: &crate::ClassDeclaration) -> Self::Output { unimplemented!() }
    fn visit_binary_expression(&mut self, _expr: &crate::BinaryExpression) -> Self::Output { unimplemented!() }
    fn visit_unary_expression(&mut self, _expr: &crate::UnaryExpression) -> Self::Output { unimplemented!() }
    fn visit_call_expression(&mut self, _expr: &crate::CallExpression) -> Self::Output { unimplemented!() }
    fn visit_new_expression(&mut self, _expr: &crate::NewExpression) -> Self::Output { unimplemented!() }
    fn visit_member_expression(&mut self, _expr: &crate::MemberExpression) -> Self::Output { unimplemented!() }
    fn visit_assignment_expression(&mut self, _expr: &crate::AssignmentExpression) -> Self::Output { unimplemented!() }
    fn visit_conditional_expression(&mut self, _expr: &crate::ConditionalExpression) -> Self::Output { unimplemented!() }
    fn visit_logical_expression(&mut self, _expr: &crate::LogicalExpression) -> Self::Output { unimplemented!() }
    fn visit_update_expression(&mut self, _expr: &crate::UpdateExpression) -> Self::Output { unimplemented!() }
    fn visit_block_statement(&mut self, _stmt: &crate::BlockStatement) -> Self::Output { unimplemented!() }
    fn visit_if_statement(&mut self, _stmt: &crate::IfStatement) -> Self::Output { unimplemented!() }
    fn visit_for_statement(&mut self, _stmt: &crate::ForStatement) -> Self::Output { unimplemented!() }
    fn visit_while_statement(&mut self, _stmt: &crate::WhileStatement) -> Self::Output { unimplemented!() }
    fn visit_do_while_statement(&mut self, _stmt: &crate::DoWhileStatement) -> Self::Output { unimplemented!() }
    fn visit_switch_statement(&mut self, _stmt: &crate::SwitchStatement) -> Self::Output { unimplemented!() }
    fn visit_try_statement(&mut self, _stmt: &crate::TryStatement) -> Self::Output { unimplemented!() }
    fn visit_throw_statement(&mut self, _stmt: &crate::ThrowStatement) -> Self::Output { unimplemented!() }
    fn visit_return_statement(&mut self, _stmt: &crate::ReturnStatement) -> Self::Output { unimplemented!() }
    fn visit_break_statement(&mut self, _stmt: &crate::BreakStatement) -> Self::Output { unimplemented!() }
    fn visit_continue_statement(&mut self, _stmt: &crate::ContinueStatement) -> Self::Output { unimplemented!() }
    fn visit_expression_statement(&mut self, _stmt: &crate::ExpressionStatement) -> Self::Output { unimplemented!() }
    fn visit_array_literal(&mut self, _lit: &crate::ArrayLiteral) -> Self::Output { unimplemented!() }
    fn visit_object_literal(&mut self, _lit: &crate::ObjectLiteral) -> Self::Output { unimplemented!() }
    fn visit_property(&mut self, _prop: &crate::Property) -> Self::Output { unimplemented!() }
    fn visit_identifier(&mut self, _id: &str) -> Self::Output { unimplemented!() }
    fn visit_number(&mut self, _num: f64) -> Self::Output { unimplemented!() }
    fn visit_string(&mut self, _s: &str) -> Self::Output { unimplemented!() }
    fn visit_boolean(&mut self, _b: bool) -> Self::Output { unimplemented!() }
    fn visit_null(&mut self) -> Self::Output { unimplemented!() }
    fn visit_undefined(&mut self) -> Self::Output { unimplemented!() }
    fn visit_this(&mut self) -> Self::Output { unimplemented!() }
    fn visit_unknown(&mut self, _node: &Node) -> Self::Output { unimplemented!() }
}

/// Simple visitor that counts nodes
pub struct NodeCounter {
    pub count: usize,
}

impl NodeCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Visitor for NodeCounter {
    type Output = ();
    
    fn visit_node(&mut self, node: &Node) {
        self.count += 1;
        // Recursively visit child nodes
        match node {
            Node::Program(program) => {
                for child in &program.body {
                    self.visit_node(child);
                }
            }
            Node::BinaryExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
            }
            Node::UnaryExpression(expr) => {
                self.visit_node(&expr.argument);
            }
            Node::CallExpression(expr) => {
                self.visit_node(&expr.callee);
                for arg in &expr.arguments {
                    self.visit_node(arg);
                }
            }
            Node::BlockStatement(stmt) => {
                for child in &stmt.body {
                    self.visit_node(child);
                }
            }
            Node::IfStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.consequent);
                if let Some(alt) = &stmt.alternate {
                    self.visit_node(alt);
                }
            }
            // Add more cases as needed
            _ => {}
        }
    }
}

/// Visitor that prints the AST structure
pub struct AstPrinter {
    pub indent: usize,
}

impl AstPrinter {
    pub fn new() -> Self {
        Self { indent: 0 }
    }
    
    fn print_indent(&self) {
        for _ in 0..self.indent {
            print!("  ");
        }
    }
}

impl Visitor for AstPrinter {
    type Output = ();
    
    fn visit_node(&mut self, node: &Node) {
        self.print_indent();
        match node {
            Node::Program(_) => println!("Program"),
            Node::VariableDeclaration(decl) => println!("VariableDeclaration({})", decl.kind),
            Node::FunctionDeclaration(decl) => println!("FunctionDeclaration"),
            Node::BinaryExpression(expr) => println!("BinaryExpression({})", expr.operator),
            Node::UnaryExpression(expr) => println!("UnaryExpression({})", expr.operator),
            Node::CallExpression(_) => println!("CallExpression"),
            Node::BlockStatement(_) => println!("BlockStatement"),
            Node::IfStatement(_) => println!("IfStatement"),
            Node::Identifier(id) => println!("Identifier({})", id),
            Node::Number(n) => println!("Number({})", n),
            Node::String(s) => println!("String({})", s),
            Node::Boolean(b) => println!("Boolean({})", b),
            Node::Null => println!("Null"),
            Node::Undefined => println!("Undefined"),
            Node::This => println!("This"),
            _ => println!("Unknown"),
        }
        
        // Recursively visit child nodes with increased indent
        self.indent += 1;
        match node {
            Node::Program(program) => {
                for child in &program.body {
                    self.visit_node(child);
                }
            }
            Node::BinaryExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
            }
            Node::UnaryExpression(expr) => {
                self.visit_node(&expr.argument);
            }
            Node::CallExpression(expr) => {
                self.visit_node(&expr.callee);
                for arg in &expr.arguments {
                    self.visit_node(arg);
                }
            }
            Node::BlockStatement(stmt) => {
                for child in &stmt.body {
                    self.visit_node(child);
                }
            }
            Node::IfStatement(stmt) => {
                self.visit_node(&stmt.test);
                self.visit_node(&stmt.consequent);
                if let Some(alt) = &stmt.alternate {
                    self.visit_node(alt);
                }
            }
            // Add more cases as needed
            _ => {}
        }
        self.indent -= 1;
    }
} 