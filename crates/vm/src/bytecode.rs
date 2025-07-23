// Bytecode system for ECMAScript

use std::collections::HashMap;
use crate::parser::ast::{Node, Program};
use crate::runtime::Value;
use super::instructions::Instruction;

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub strings: Vec<String>,
    pub functions: Vec<FunctionInfo>,
    pub source_map: HashMap<usize, usize>, // instruction index -> source position
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub start_pc: usize,
    pub end_pc: usize,
    pub param_count: usize,
    pub local_count: usize,
    pub max_stack: usize,
    pub source: String,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            strings: Vec::new(),
            functions: Vec::new(),
            source_map: HashMap::new(),
        }
    }
    
    pub fn from_ast(ast: &Program) -> Result<Self, String> {
        let mut bytecode = Bytecode::new();
        let mut generator = BytecodeGenerator::new(&mut bytecode);
        
        for node in &ast.body {
            generator.generate_statement(node)?;
        }
        
        Ok(bytecode)
    }
    
    pub fn add_instruction(&mut self, instruction: Instruction, source_pos: Option<usize>) {
        if let Some(pos) = source_pos {
            self.source_map.insert(self.instructions.len(), pos);
        }
        self.instructions.push(instruction);
    }
    
    pub fn add_constant(&mut self, value: Value) -> usize {
        for (i, constant) in self.constants.iter().enumerate() {
            if constant == &value {
                return i;
            }
        }
        let index = self.constants.len();
        self.constants.push(value);
        index
    }
    
    pub fn add_string(&mut self, string: String) -> usize {
        for (i, s) in self.strings.iter().enumerate() {
            if s == &string {
                return i;
            }
        }
        let index = self.strings.len();
        self.strings.push(string);
        index
    }
    
    pub fn add_function(&mut self, info: FunctionInfo) -> usize {
        let index = self.functions.len();
        self.functions.push(info);
        index
    }
    
    pub fn get_instruction(&self, pc: usize) -> Option<&Instruction> {
        self.instructions.get(pc)
    }
    
    pub fn get_constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }
    
    pub fn get_string(&self, index: usize) -> Option<&str> {
        self.strings.get(index).map(|s| s.as_str())
    }
    
    pub fn get_function(&self, index: usize) -> Option<&FunctionInfo> {
        self.functions.get(index)
    }
    
    pub fn get_source_position(&self, pc: usize) -> Option<usize> {
        self.source_map.get(&pc).copied()
    }
    
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
    
    pub fn optimize(&mut self) {
        // TODO: Implement bytecode optimization
        // - Constant folding
        // - Dead code elimination
        // - Instruction combining
        // - Jump optimization
    }
    
    pub fn disassemble(&self) -> String {
        let mut output = String::new();
        
        for (i, instruction) in self.instructions.iter().enumerate() {
            output.push_str(&format!("{:04x}: {}\n", i, instruction.disassemble(self)));
        }
        
        if !self.constants.is_empty() {
            output.push_str("\nConstants:\n");
            for (i, constant) in self.constants.iter().enumerate() {
                output.push_str(&format!("  {}: {:?}\n", i, constant));
            }
        }
        
        if !self.strings.is_empty() {
            output.push_str("\nStrings:\n");
            for (i, string) in self.strings.iter().enumerate() {
                output.push_str(&format!("  {}: {:?}\n", i, string));
            }
        }
        
        if !self.functions.is_empty() {
            output.push_str("\nFunctions:\n");
            for (i, function) in self.functions.iter().enumerate() {
                output.push_str(&format!("  {}: {} ({} params, {} locals)\n", 
                    i, function.name, function.param_count, function.local_count));
            }
        }
        
        output
    }
}

struct BytecodeGenerator<'a> {
    bytecode: &'a mut Bytecode,
    current_function: Option<usize>,
    local_vars: HashMap<String, usize>,
    param_count: usize,
    local_count: usize,
    max_stack: usize,
    current_stack: usize,
}

impl<'a> BytecodeGenerator<'a> {
    fn new(bytecode: &'a mut Bytecode) -> Self {
        Self {
            bytecode,
            current_function: None,
            local_vars: HashMap::new(),
            param_count: 0,
            local_count: 0,
            max_stack: 0,
            current_stack: 0,
        }
    }
    
    fn generate_statement(&mut self, node: &Node) -> Result<(), String> {
        match node {
            Node::VariableDeclaration(decl) => self.generate_variable_declaration(decl),
            Node::FunctionDeclaration(func) => self.generate_function_declaration(func),
            Node::ExpressionStatement(expr) => self.generate_expression(&expr.expression),
            Node::ReturnStatement(ret) => self.generate_return_statement(ret),
            Node::IfStatement(if_stmt) => self.generate_if_statement(if_stmt),
            Node::BlockStatement(block) => self.generate_block_statement(block),
            Node::ForStatement(for_stmt) => self.generate_for_statement(for_stmt),
            Node::WhileStatement(while_stmt) => self.generate_while_statement(while_stmt),
            Node::SwitchStatement(switch_stmt) => self.generate_switch_statement(switch_stmt),
            Node::TryStatement(try_stmt) => self.generate_try_statement(try_stmt),
            Node::ThrowStatement(throw_stmt) => self.generate_throw_statement(throw_stmt),
            Node::BreakStatement(break_stmt) => self.generate_break_statement(break_stmt),
            Node::ContinueStatement(continue_stmt) => self.generate_continue_statement(continue_stmt),
            Node::DebuggerStatement(_) => self.generate_debugger_statement(),
            _ => Err(format!("Unsupported statement: {:?}", node)),
        }
    }
    
    fn generate_expression(&mut self, node: &Node) -> Result<(), String> {
        match node {
            Node::Identifier(name) => self.generate_identifier(name),
            Node::Number(n) => self.generate_number(*n),
            Node::String(s) => self.generate_string(s),
            Node::Boolean(b) => self.generate_boolean(*b),
            Node::Null => self.generate_null(),
            Node::Undefined => self.generate_undefined(),
            Node::This => self.generate_this(),
            Node::BinaryExpression(bin) => self.generate_binary_expression(bin),
            Node::UnaryExpression(unary) => self.generate_unary_expression(unary),
            Node::CallExpression(call) => self.generate_call_expression(call),
            Node::NewExpression(new) => self.generate_new_expression(new),
            Node::MemberExpression(member) => self.generate_member_expression(member),
            Node::AssignmentExpression(assign) => self.generate_assignment_expression(assign),
            Node::ConditionalExpression(cond) => self.generate_conditional_expression(cond),
            Node::LogicalExpression(logical) => self.generate_logical_expression(logical),
            Node::UpdateExpression(update) => self.generate_update_expression(update),
            Node::ArrayLiteral(arr) => self.generate_array_literal(arr),
            Node::ObjectLiteral(obj) => self.generate_object_literal(obj),
            Node::FunctionExpression(func) => self.generate_function_expression(func),
            Node::ArrowFunctionExpression(arrow) => self.generate_arrow_function_expression(arrow),
            Node::TemplateLiteral(template) => self.generate_template_literal(template),
            Node::YieldExpression(yield_expr) => self.generate_yield_expression(yield_expr),
            Node::AwaitExpression(await_expr) => self.generate_await_expression(await_expr),
            _ => Err(format!("Unsupported expression: {:?}", node)),
        }
    }
    
    fn generate_variable_declaration(&mut self, decl: &crate::parser::ast::VariableDeclaration) -> Result<(), String> {
        for var_decl in &decl.declarations {
            if let Some(init) = &var_decl.init {
                self.generate_expression(init)?;
            } else {
                self.bytecode.add_instruction(Instruction::LoadUndefined, None);
            }
            
            match &*var_decl.id {
                Node::Identifier(name) => {
                    let index = self.get_or_create_local(name);
                    self.bytecode.add_instruction(Instruction::StoreLocal(index), None);
                }
                _ => return Err("Variable declaration with non-identifier".to_string()),
            }
        }
        Ok(())
    }
    
    fn generate_identifier(&mut self, name: &str) -> Result<(), String> {
        if let Some(index) = self.local_vars.get(name) {
            self.bytecode.add_instruction(Instruction::LoadLocal(*index), None);
        } else {
            let string_index = self.bytecode.add_string(name.to_string());
            self.bytecode.add_instruction(Instruction::LoadGlobal(string_index), None);
        }
        Ok(())
    }
    
    fn generate_number(&mut self, n: f64) -> Result<(), String> {
        let index = self.bytecode.add_constant(Value::Number(n));
        self.bytecode.add_instruction(Instruction::LoadConstant(index), None);
        Ok(())
    }
    
    fn generate_string(&mut self, s: &str) -> Result<(), String> {
        let index = self.bytecode.add_constant(Value::String(s.to_string()));
        self.bytecode.add_instruction(Instruction::LoadConstant(index), None);
        Ok(())
    }
    
    fn generate_boolean(&mut self, b: bool) -> Result<(), String> {
        let index = self.bytecode.add_constant(Value::Boolean(b));
        self.bytecode.add_instruction(Instruction::LoadConstant(index), None);
        Ok(())
    }
    
    fn generate_null(&mut self) -> Result<(), String> {
        self.bytecode.add_instruction(Instruction::LoadNull, None);
        Ok(())
    }
    
    fn generate_undefined(&mut self) -> Result<(), String> {
        self.bytecode.add_instruction(Instruction::LoadUndefined, None);
        Ok(())
    }
    
    fn generate_this(&mut self) -> Result<(), String> {
        self.bytecode.add_instruction(Instruction::LoadThis, None);
        Ok(())
    }
    
    fn generate_binary_expression(&mut self, bin: &crate::parser::ast::BinaryExpression) -> Result<(), String> {
        self.generate_expression(&bin.left)?;
        self.generate_expression(&bin.right)?;
        
        match bin.operator.as_str() {
            "+" => self.bytecode.add_instruction(Instruction::Add, None),
            "-" => self.bytecode.add_instruction(Instruction::Subtract, None),
            "*" => self.bytecode.add_instruction(Instruction::Multiply, None),
            "/" => self.bytecode.add_instruction(Instruction::Divide, None),
            "%" => self.bytecode.add_instruction(Instruction::Modulo, None),
            "**" => self.bytecode.add_instruction(Instruction::Exponentiate, None),
            "==" => self.bytecode.add_instruction(Instruction::Equal, None),
            "!=" => self.bytecode.add_instruction(Instruction::NotEqual, None),
            "===" => self.bytecode.add_instruction(Instruction::StrictEqual, None),
            "!==" => self.bytecode.add_instruction(Instruction::StrictNotEqual, None),
            "<" => self.bytecode.add_instruction(Instruction::LessThan, None),
            "<=" => self.bytecode.add_instruction(Instruction::LessThanEqual, None),
            ">" => self.bytecode.add_instruction(Instruction::GreaterThan, None),
            ">=" => self.bytecode.add_instruction(Instruction::GreaterThanEqual, None),
            "&&" => self.bytecode.add_instruction(Instruction::LogicalAnd, None),
            "||" => self.bytecode.add_instruction(Instruction::LogicalOr, None),
            "&" => self.bytecode.add_instruction(Instruction::BitwiseAnd, None),
            "|" => self.bytecode.add_instruction(Instruction::BitwiseOr, None),
            "^" => self.bytecode.add_instruction(Instruction::BitwiseXor, None),
            "<<" => self.bytecode.add_instruction(Instruction::LeftShift, None),
            ">>" => self.bytecode.add_instruction(Instruction::RightShift, None),
            ">>>" => self.bytecode.add_instruction(Instruction::UnsignedRightShift, None),
            _ => return Err(format!("Unsupported binary operator: {}", bin.operator)),
        }
        Ok(())
    }
    
    fn generate_unary_expression(&mut self, unary: &crate::parser::ast::UnaryExpression) -> Result<(), String> {
        if unary.prefix {
            match unary.operator.as_str() {
                "!" => {
                    self.generate_expression(&unary.argument)?;
                    self.bytecode.add_instruction(Instruction::LogicalNot, None);
                }
                "~" => {
                    self.generate_expression(&unary.argument)?;
                    self.bytecode.add_instruction(Instruction::BitwiseNot, None);
                }
                "+" => {
                    self.generate_expression(&unary.argument)?;
                    self.bytecode.add_instruction(Instruction::UnaryPlus, None);
                }
                "-" => {
                    self.generate_expression(&unary.argument)?;
                    self.bytecode.add_instruction(Instruction::UnaryMinus, None);
                }
                "typeof" => {
                    self.generate_expression(&unary.argument)?;
                    self.bytecode.add_instruction(Instruction::TypeOf, None);
                }
                "void" => {
                    self.generate_expression(&unary.argument)?;
                    self.bytecode.add_instruction(Instruction::Void, None);
                }
                "delete" => {
                    self.generate_expression(&unary.argument)?;
                    self.bytecode.add_instruction(Instruction::Delete, None);
                }
                _ => return Err(format!("Unsupported unary operator: {}", unary.operator)),
            }
        } else {
            // Postfix operators (++, --)
            self.generate_expression(&unary.argument)?;
            match unary.operator.as_str() {
                "++" => self.bytecode.add_instruction(Instruction::Increment, None),
                "--" => self.bytecode.add_instruction(Instruction::Decrement, None),
                _ => return Err(format!("Unsupported postfix operator: {}", unary.operator)),
            }
        }
        Ok(())
    }
    
    fn generate_call_expression(&mut self, call: &crate::parser::ast::CallExpression) -> Result<(), String> {
        self.generate_expression(&call.callee)?;
        
        for arg in &call.arguments {
            self.generate_expression(arg)?;
        }
        
        self.bytecode.add_instruction(Instruction::Call(call.arguments.len()), None);
        Ok(())
    }
    
    fn generate_new_expression(&mut self, new: &crate::parser::ast::NewExpression) -> Result<(), String> {
        self.generate_expression(&new.callee)?;
        
        for arg in &new.arguments {
            self.generate_expression(arg)?;
        }
        
        self.bytecode.add_instruction(Instruction::New(new.arguments.len()), None);
        Ok(())
    }
    
    fn generate_member_expression(&mut self, member: &crate::parser::ast::MemberExpression) -> Result<(), String> {
        self.generate_expression(&member.object)?;
        self.generate_expression(&member.property)?;
        
        if member.computed {
            self.bytecode.add_instruction(Instruction::LoadProperty, None);
        } else {
            // For non-computed properties, we need to convert the property to a string
            if let Node::Identifier(name) = &*member.property {
                let string_index = self.bytecode.add_string(name.clone());
                self.bytecode.add_instruction(Instruction::LoadConstant(string_index), None);
                self.bytecode.add_instruction(Instruction::LoadProperty, None);
            } else {
                return Err("Non-computed property must be an identifier".to_string());
            }
        }
        Ok(())
    }
    
    fn generate_assignment_expression(&mut self, assign: &crate::parser::ast::AssignmentExpression) -> Result<(), String> {
        match assign.operator.as_str() {
            "=" => {
                self.generate_expression(&assign.right)?;
                self.generate_assignment_target(&assign.left)?;
            }
            "+=" => {
                self.generate_expression(&assign.left)?;
                self.generate_expression(&assign.right)?;
                self.bytecode.add_instruction(Instruction::Add, None);
                self.generate_assignment_target(&assign.left)?;
            }
            "-=" => {
                self.generate_expression(&assign.left)?;
                self.generate_expression(&assign.right)?;
                self.bytecode.add_instruction(Instruction::Subtract, None);
                self.generate_assignment_target(&assign.left)?;
            }
            "*=" => {
                self.generate_expression(&assign.left)?;
                self.generate_expression(&assign.right)?;
                self.bytecode.add_instruction(Instruction::Multiply, None);
                self.generate_assignment_target(&assign.left)?;
            }
            "/=" => {
                self.generate_expression(&assign.left)?;
                self.generate_expression(&assign.right)?;
                self.bytecode.add_instruction(Instruction::Divide, None);
                self.generate_assignment_target(&assign.left)?;
            }
            "%=" => {
                self.generate_expression(&assign.left)?;
                self.generate_expression(&assign.right)?;
                self.bytecode.add_instruction(Instruction::Modulo, None);
                self.generate_assignment_target(&assign.left)?;
            }
            "**=" => {
                self.generate_expression(&assign.left)?;
                self.generate_expression(&assign.right)?;
                self.bytecode.add_instruction(Instruction::Exponentiate, None);
                self.generate_assignment_target(&assign.left)?;
            }
            _ => return Err(format!("Unsupported assignment operator: {}", assign.operator)),
        }
        Ok(())
    }
    
    fn generate_assignment_target(&mut self, target: &Node) -> Result<(), String> {
        match target {
            Node::Identifier(name) => {
                if let Some(index) = self.local_vars.get(name) {
                    self.bytecode.add_instruction(Instruction::StoreLocal(*index), None);
                } else {
                    let string_index = self.bytecode.add_string(name.clone());
                    self.bytecode.add_instruction(Instruction::StoreGlobal(string_index), None);
                }
            }
            Node::MemberExpression(member) => {
                self.bytecode.add_instruction(Instruction::StoreProperty, None);
            }
            _ => return Err("Unsupported assignment target".to_string()),
        }
        Ok(())
    }
    
    fn generate_conditional_expression(&mut self, cond: &crate::parser::ast::ConditionalExpression) -> Result<(), String> {
        self.generate_expression(&cond.test)?;
        
        let jump_false = self.bytecode.instructions.len();
        self.bytecode.add_instruction(Instruction::JumpIfFalse(0), None);
        
        self.generate_expression(&cond.consequent)?;
        
        let jump = self.bytecode.instructions.len();
        self.bytecode.add_instruction(Instruction::Jump(0), None);
        
        // Update the false jump target
        if let Some(Instruction::JumpIfFalse(_)) = self.bytecode.instructions.get_mut(jump_false) {
            *self.bytecode.instructions.get_mut(jump_false).unwrap() = Instruction::JumpIfFalse(self.bytecode.instructions.len());
        }
        
        self.generate_expression(&cond.alternate)?;
        
        // Update the jump target
        if let Some(Instruction::Jump(_)) = self.bytecode.instructions.get_mut(jump) {
            *self.bytecode.instructions.get_mut(jump).unwrap() = Instruction::Jump(self.bytecode.instructions.len());
        }
        
        Ok(())
    }
    
    fn generate_logical_expression(&mut self, logical: &crate::parser::ast::LogicalExpression) -> Result<(), String> {
        self.generate_expression(&logical.left)?;
        
        match logical.operator.as_str() {
            "&&" => {
                let jump_false = self.bytecode.instructions.len();
                self.bytecode.add_instruction(Instruction::JumpIfFalse(0), None);
                
                self.generate_expression(&logical.right)?;
                
                if let Some(Instruction::JumpIfFalse(_)) = self.bytecode.instructions.get_mut(jump_false) {
                    *self.bytecode.instructions.get_mut(jump_false).unwrap() = Instruction::JumpIfFalse(self.bytecode.instructions.len());
                }
            }
            "||" => {
                let jump_true = self.bytecode.instructions.len();
                self.bytecode.add_instruction(Instruction::JumpIfTrue(0), None);
                
                self.generate_expression(&logical.right)?;
                
                if let Some(Instruction::JumpIfTrue(_)) = self.bytecode.instructions.get_mut(jump_true) {
                    *self.bytecode.instructions.get_mut(jump_true).unwrap() = Instruction::JumpIfTrue(self.bytecode.instructions.len());
                }
            }
            "??" => {
                let jump_not_null = self.bytecode.instructions.len();
                self.bytecode.add_instruction(Instruction::JumpIfNotNull(0), None);
                
                self.generate_expression(&logical.right)?;
                
                if let Some(Instruction::JumpIfNotNull(_)) = self.bytecode.instructions.get_mut(jump_not_null) {
                    *self.bytecode.instructions.get_mut(jump_not_null).unwrap() = Instruction::JumpIfNotNull(self.bytecode.instructions.len());
                }
            }
            _ => return Err(format!("Unsupported logical operator: {}", logical.operator)),
        }
        Ok(())
    }
    
    fn generate_update_expression(&mut self, update: &crate::parser::ast::UpdateExpression) -> Result<(), String> {
        if update.prefix {
            match update.operator.as_str() {
                "++" => {
                    self.generate_expression(&update.argument)?;
                    self.bytecode.add_instruction(Instruction::Increment, None);
                }
                "--" => {
                    self.generate_expression(&update.argument)?;
                    self.bytecode.add_instruction(Instruction::Decrement, None);
                }
                _ => return Err(format!("Unsupported prefix operator: {}", update.operator)),
            }
        } else {
            // Postfix operators
            self.generate_expression(&update.argument)?;
            match update.operator.as_str() {
                "++" => self.bytecode.add_instruction(Instruction::PostIncrement, None),
                "--" => self.bytecode.add_instruction(Instruction::PostDecrement, None),
                _ => return Err(format!("Unsupported postfix operator: {}", update.operator)),
            }
        }
        Ok(())
    }
    
    fn generate_array_literal(&mut self, arr: &crate::parser::ast::ArrayLiteral) -> Result<(), String> {
        for element in &arr.elements {
            if let Some(elem) = element {
                self.generate_expression(elem)?;
            } else {
                self.bytecode.add_instruction(Instruction::LoadUndefined, None);
            }
        }
        
        self.bytecode.add_instruction(Instruction::CreateArray(arr.elements.len()), None);
        Ok(())
    }
    
    fn generate_object_literal(&mut self, obj: &crate::parser::ast::ObjectLiteral) -> Result<(), String> {
        for property in &obj.properties {
            self.generate_expression(&property.key)?;
            self.generate_expression(&property.value)?;
        }
        
        self.bytecode.add_instruction(Instruction::CreateObject(obj.properties.len()), None);
        Ok(())
    }
    
    fn generate_function_expression(&mut self, func: &crate::parser::ast::FunctionExpression) -> Result<(), String> {
        // TODO: Implement function expression generation
        self.bytecode.add_instruction(Instruction::LoadUndefined, None);
        Ok(())
    }
    
    fn generate_arrow_function_expression(&mut self, arrow: &crate::parser::ast::ArrowFunctionExpression) -> Result<(), String> {
        // TODO: Implement arrow function generation
        self.bytecode.add_instruction(Instruction::LoadUndefined, None);
        Ok(())
    }
    
    fn generate_template_literal(&mut self, template: &crate::parser::ast::TemplateLiteral) -> Result<(), String> {
        // TODO: Implement template literal generation
        self.bytecode.add_instruction(Instruction::LoadUndefined, None);
        Ok(())
    }
    
    fn generate_yield_expression(&mut self, yield_expr: &crate::parser::ast::YieldExpression) -> Result<(), String> {
        if let Some(argument) = &yield_expr.argument {
            self.generate_expression(argument)?;
        } else {
            self.bytecode.add_instruction(Instruction::LoadUndefined, None);
        }
        
        if yield_expr.delegate {
            self.bytecode.add_instruction(Instruction::YieldDelegate, None);
        } else {
            self.bytecode.add_instruction(Instruction::Yield, None);
        }
        Ok(())
    }
    
    fn generate_await_expression(&mut self, await_expr: &crate::parser::ast::AwaitExpression) -> Result<(), String> {
        self.generate_expression(&await_expr.argument)?;
        self.bytecode.add_instruction(Instruction::Await, None);
        Ok(())
    }
    
    fn generate_function_declaration(&mut self, func: &crate::parser::ast::FunctionDeclaration) -> Result<(), String> {
        // TODO: Implement function declaration generation
        Ok(())
    }
    
    fn generate_return_statement(&mut self, ret: &crate::parser::ast::ReturnStatement) -> Result<(), String> {
        if let Some(argument) = &ret.argument {
            self.generate_expression(argument)?;
        } else {
            self.bytecode.add_instruction(Instruction::LoadUndefined, None);
        }
        
        self.bytecode.add_instruction(Instruction::Return, None);
        Ok(())
    }
    
    fn generate_if_statement(&mut self, if_stmt: &crate::parser::ast::IfStatement) -> Result<(), String> {
        self.generate_expression(&if_stmt.test)?;
        
        let jump_false = self.bytecode.instructions.len();
        self.bytecode.add_instruction(Instruction::JumpIfFalse(0), None);
        
        self.generate_statement(&if_stmt.consequent)?;
        
        if let Some(alternate) = &if_stmt.alternate {
            let jump = self.bytecode.instructions.len();
            self.bytecode.add_instruction(Instruction::Jump(0), None);
            
            // Update the false jump target
            if let Some(Instruction::JumpIfFalse(_)) = self.bytecode.instructions.get_mut(jump_false) {
                *self.bytecode.instructions.get_mut(jump_false).unwrap() = Instruction::JumpIfFalse(self.bytecode.instructions.len());
            }
            
            self.generate_statement(alternate)?;
            
            // Update the jump target
            if let Some(Instruction::Jump(_)) = self.bytecode.instructions.get_mut(jump) {
                *self.bytecode.instructions.get_mut(jump).unwrap() = Instruction::Jump(self.bytecode.instructions.len());
            }
        } else {
            // Update the false jump target
            if let Some(Instruction::JumpIfFalse(_)) = self.bytecode.instructions.get_mut(jump_false) {
                *self.bytecode.instructions.get_mut(jump_false).unwrap() = Instruction::JumpIfFalse(self.bytecode.instructions.len());
            }
        }
        
        Ok(())
    }
    
    fn generate_block_statement(&mut self, block: &crate::parser::ast::BlockStatement) -> Result<(), String> {
        for statement in &block.body {
            self.generate_statement(statement)?;
        }
        Ok(())
    }
    
    fn generate_for_statement(&mut self, for_stmt: &crate::parser::ast::ForStatement) -> Result<(), String> {
        // TODO: Implement for statement generation
        Ok(())
    }
    
    fn generate_while_statement(&mut self, while_stmt: &crate::parser::ast::WhileStatement) -> Result<(), String> {
        // TODO: Implement while statement generation
        Ok(())
    }
    
    fn generate_switch_statement(&mut self, switch_stmt: &crate::parser::ast::SwitchStatement) -> Result<(), String> {
        // TODO: Implement switch statement generation
        Ok(())
    }
    
    fn generate_try_statement(&mut self, try_stmt: &crate::parser::ast::TryStatement) -> Result<(), String> {
        // TODO: Implement try statement generation
        Ok(())
    }
    
    fn generate_throw_statement(&mut self, throw_stmt: &crate::parser::ast::ThrowStatement) -> Result<(), String> {
        self.generate_expression(&throw_stmt.argument)?;
        self.bytecode.add_instruction(Instruction::Throw, None);
        Ok(())
    }
    
    fn generate_break_statement(&mut self, break_stmt: &crate::parser::ast::BreakStatement) -> Result<(), String> {
        // TODO: Implement break statement generation
        Ok(())
    }
    
    fn generate_continue_statement(&mut self, continue_stmt: &crate::parser::ast::ContinueStatement) -> Result<(), String> {
        // TODO: Implement continue statement generation
        Ok(())
    }
    
    fn generate_debugger_statement(&mut self) -> Result<(), String> {
        self.bytecode.add_instruction(Instruction::Debugger, None);
        Ok(())
    }
    
    fn get_or_create_local(&mut self, name: &str) -> usize {
        if let Some(index) = self.local_vars.get(name) {
            *index
        } else {
            let index = self.local_count;
            self.local_vars.insert(name.to_string(), index);
            self.local_count += 1;
            index
        }
    }
} 