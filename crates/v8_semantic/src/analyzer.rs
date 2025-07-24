use crate::{SemanticError, SemanticResult, Type, Scope};
use crate::scope::ScopeType;
use v8_ast::Node;
use std::collections::HashMap;

/// Main semantic analyzer
pub struct SemanticAnalyzer {
    /// Current scope stack
    scope_stack: Vec<Scope>,
    
    /// Type environment
    type_env: HashMap<String, Type>,
    
    /// Collected errors
    errors: Vec<SemanticError>,
    
    /// Whether we're in strict mode
    strict_mode: bool,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            scope_stack: Vec::new(),
            type_env: HashMap::new(),
            errors: Vec::new(),
            strict_mode: false,
        };
        
        // Push global scope
        analyzer.scope_stack.push(Scope::new_global());
        analyzer
    }
    
    /// Analyze an AST node
    pub fn analyze(&mut self, ast: &Node) -> SemanticResult<()> {
        self.visit_node(ast)?;
        
        if !self.errors.is_empty() {
            return Err(self.errors.remove(0));
        }
        
        Ok(())
    }
    
    /// Visit a node and perform semantic analysis
    fn visit_node(&mut self, node: &Node) -> SemanticResult<Type> {
        match node {
            Node::Program(program) => self.visit_program(program),
            Node::VariableDeclaration(decl) => self.visit_variable_declaration(decl),
            Node::FunctionDeclaration(func) => self.visit_function_declaration(func),
            Node::ExpressionStatement(stmt) => self.visit_expression_statement(stmt),
            Node::BinaryExpression(expr) => self.visit_binary_expression(expr),
            Node::UnaryExpression(expr) => self.visit_unary_expression(expr),
            Node::Identifier(id) => self.visit_identifier(id),
            Node::Number(_) => Ok(Type::Number),
            Node::String(_) => Ok(Type::String),
            Node::Boolean(_) => Ok(Type::Boolean),
            Node::Null => Ok(Type::Null),
            Node::Undefined => Ok(Type::Undefined),
            Node::This => self.visit_this(),
            Node::CallExpression(call) => self.visit_call_expression(call),
            Node::AssignmentExpression(assign) => self.visit_assignment_expression(assign),
            Node::IfStatement(if_stmt) => self.visit_if_statement(if_stmt),
            Node::WhileStatement(while_stmt) => self.visit_while_statement(while_stmt),
            Node::ReturnStatement(return_stmt) => self.visit_return_statement(return_stmt),
            Node::BlockStatement(block) => self.visit_block_statement(block),
            Node::ArrayLiteral(array) => self.visit_array_literal(array),
            Node::ObjectLiteral(obj) => self.visit_object_literal(obj),
            Node::Property(prop) => self.visit_property(prop),
            Node::MemberExpression(member) => self.visit_member_expression(member),
            Node::LogicalExpression(logical) => self.visit_logical_expression(logical),
            Node::ConditionalExpression(conditional) => self.visit_conditional_expression(conditional),
            Node::ArrowFunctionExpression(arrow) => self.visit_arrow_function_expression(arrow),
            _ => Ok(Type::Any), // Default for unimplemented nodes
        }
    }
    
    /// Visit program node
    fn visit_program(&mut self, program: &v8_ast::Program) -> SemanticResult<Type> {
        for statement in &program.body {
            self.visit_node(statement)?;
        }
        Ok(Type::Undefined)
    }
    
    /// Visit variable declaration
    fn visit_variable_declaration(&mut self, decl: &v8_ast::VariableDeclaration) -> SemanticResult<Type> {
        let is_const = decl.kind == "const";
        
        for var_decl in &decl.declarations {
            // For now, only handle simple identifiers
            if let Node::Identifier(var_name) = &*var_decl.id {
                // Determine variable type first
                let var_type = if let Some(init) = &var_decl.init {
                    self.visit_node(init)?
                } else {
                    Type::Undefined
                };
                
                // Now get mutable reference to current scope
                let current_scope = self.scope_stack.last_mut().unwrap();
                
                // Check for duplicate declaration in current scope
                if current_scope.is_variable_declared_in_current_scope(var_name) {
                    self.errors.push(SemanticError::DuplicateDeclaration {
                        name: var_name.clone(),
                        position: decl.span.as_ref().map(|s| s.start.clone()),
                    });
                    continue;
                }
                
                // Declare variable in current scope
                current_scope.declare_variable(
                    var_name,
                    var_type.clone(),
                    !is_const, // const is immutable
                    1, // TODO: Get actual line number
                );
                
                // Mark as initialized if it has an initializer
                if var_decl.init.is_some() {
                    current_scope.initialize_variable(var_name);
                }
            }
        }
        
        Ok(Type::Undefined)
    }
    
    /// Visit function declaration
    fn visit_function_declaration(&mut self, func: &v8_ast::FunctionDeclaration) -> SemanticResult<Type> {
        // Get function name
        let func_name = if let Some(id) = &func.id {
            if let Node::Identifier(name) = &**id {
                name.clone()
            } else {
                return Ok(Type::Any);
            }
        } else {
            return Ok(Type::Any); // Anonymous function
        };
        
        // Create function scope
        let current_scope = self.scope_stack.last().unwrap().clone();
        let function_scope = Scope::new_child(current_scope, ScopeType::Function);
        self.scope_stack.push(function_scope);
        
        // Declare parameters in function scope
        for param in &func.params {
            if let Node::Identifier(param_name) = param {
                let current_scope = self.scope_stack.last_mut().unwrap();
                current_scope.declare_variable(
                    param_name,
                    Type::Any, // TODO: Infer parameter types
                    true, // Parameters are mutable
                    1, // TODO: Get actual line number
                );
                current_scope.initialize_variable(param_name);
            }
        }
        
        // Analyze function body
        let return_type = self.visit_node(&func.body)?;
        
        // Pop function scope
        self.scope_stack.pop();
        
        // Declare function in current scope
        let current_scope = self.scope_stack.last_mut().unwrap();
        current_scope.declare_function(
            &func_name,
            vec![], // TODO: Get actual parameter types
            return_type.clone(),
            false, // Not a method
            1, // TODO: Get actual line number
        );
        
        Ok(Type::Function {
            params: vec![],
            return_type: Box::new(return_type),
        })
    }
    
    /// Visit expression statement
    fn visit_expression_statement(&mut self, stmt: &v8_ast::ExpressionStatement) -> SemanticResult<Type> {
        self.visit_node(&stmt.expression)
    }
    
    /// Visit binary expression
    fn visit_binary_expression(&mut self, expr: &v8_ast::BinaryExpression) -> SemanticResult<Type> {
        let left_type = self.visit_node(&expr.left)?;
        let right_type = self.visit_node(&expr.right)?;
        
        // Check type compatibility for the operation
        match expr.operator.as_str() {
            "+" => {
                // String concatenation or number addition
                if left_type.is_compatible_with(&Type::String) || right_type.is_compatible_with(&Type::String) {
                    Ok(Type::String) // String concatenation
                } else if left_type.is_compatible_with(&Type::Number) && right_type.is_compatible_with(&Type::Number) {
                    Ok(Type::Number) // Number addition
                } else {
                    // JavaScript allows mixed types with coercion
                    Ok(Type::String) // Default to string for mixed types
                }
            }
            "-" | "*" | "/" | "%" => {
                if !left_type.is_compatible_with(&Type::Number) || !right_type.is_compatible_with(&Type::Number) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "number".to_string(),
                        found: format!("{:?} and {:?}", left_type, right_type),
                        position: expr.span.as_ref().map(|s| s.start.clone()),
                    });
                }
                Ok(Type::Number)
            }
            "==" | "!=" | "===" | "!==" => {
                Ok(Type::Boolean)
            }
            "<" | ">" | "<=" | ">=" => {
                if !left_type.is_compatible_with(&Type::Number) || !right_type.is_compatible_with(&Type::Number) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "number".to_string(),
                        found: format!("{:?} and {:?}", left_type, right_type),
                        position: expr.span.as_ref().map(|s| s.start.clone()),
                    });
                }
                Ok(Type::Boolean)
            }
            "&&" | "||" => {
                if !left_type.is_compatible_with(&Type::Boolean) || !right_type.is_compatible_with(&Type::Boolean) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "boolean".to_string(),
                        found: format!("{:?} and {:?}", left_type, right_type),
                        position: expr.span.as_ref().map(|s| s.start.clone()),
                    });
                }
                Ok(Type::Boolean)
            }
            _ => Ok(Type::Any),
        }
    }
    
    /// Visit unary expression
    fn visit_unary_expression(&mut self, expr: &v8_ast::UnaryExpression) -> SemanticResult<Type> {
        let operand_type = self.visit_node(&expr.argument)?;
        
        match expr.operator.as_str() {
            "!" => {
                if !operand_type.is_compatible_with(&Type::Boolean) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "boolean".to_string(),
                        found: format!("{:?}", operand_type),
                        position: expr.span.as_ref().map(|s| s.start.clone()),
                    });
                }
                Ok(Type::Boolean)
            }
            "+" | "-" => {
                if !operand_type.is_compatible_with(&Type::Number) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "number".to_string(),
                        found: format!("{:?}", operand_type),
                        position: expr.span.as_ref().map(|s| s.start.clone()),
                    });
                }
                Ok(Type::Number)
            }
            _ => Ok(Type::Any),
        }
    }
    
    /// Visit identifier
    fn visit_identifier(&mut self, id: &str) -> SemanticResult<Type> {
        let current_scope = self.scope_stack.last().unwrap();
        
        // Check if variable is declared
        if let Some(var_info) = current_scope.get_variable(id) {
            // Check if variable is initialized
            if !var_info.initialized {
                self.errors.push(SemanticError::UninitializedVariable {
                    name: id.to_string(),
                    position: None, // TODO: Get actual position
                });
            }
            Ok(var_info.type_info.clone())
        } else {
            self.errors.push(SemanticError::UndeclaredVariable {
                name: id.to_string(),
                position: None, // TODO: Get actual position
            });
            Ok(Type::Any)
        }
    }
    
    /// Visit 'this' expression
    fn visit_this(&mut self) -> SemanticResult<Type> {
        let current_scope = self.scope_stack.last().unwrap();
        
        // 'this' is valid in function scopes, but not in global scope
        if current_scope.is_global_scope() {
            self.errors.push(SemanticError::InvalidThisUsage {
                position: None, // TODO: Get actual position
            });
        }
        
        Ok(Type::Object)
    }
    
    /// Visit call expression
    fn visit_call_expression(&mut self, call: &v8_ast::CallExpression) -> SemanticResult<Type> {
        // First check if callee is an identifier (function name)
        if let Node::Identifier(func_name) = &*call.callee {
            let current_scope = self.scope_stack.last().unwrap();
            
            // Check if function is declared
            if let Some(func_info) = current_scope.get_function(func_name) {
                let return_type = func_info.return_type.clone();
                
                // Analyze arguments
                for arg in &call.arguments {
                    self.visit_node(arg)?;
                }
                
                Ok(return_type)
            } else {
                self.errors.push(SemanticError::UndeclaredFunction {
                    name: func_name.clone(),
                    position: call.span.as_ref().map(|s| s.start.clone()),
                });
                Ok(Type::Any)
            }
        } else {
            // For complex callees, just analyze the callee and arguments
            let callee_type = self.visit_node(&call.callee)?;
            
            // Analyze arguments
            for arg in &call.arguments {
                self.visit_node(arg)?;
            }
            
            // Check if callee is a function type
            if let Type::Function { return_type, .. } = callee_type {
                Ok(*return_type)
            } else {
                self.errors.push(SemanticError::TypeMismatch {
                    expected: "function".to_string(),
                    found: format!("{:?}", callee_type),
                    position: call.span.as_ref().map(|s| s.start.clone()),
                });
                Ok(Type::Any)
            }
        }
    }
    
    /// Visit assignment expression
    fn visit_assignment_expression(&mut self, assign: &v8_ast::AssignmentExpression) -> SemanticResult<Type> {
        let value_type = self.visit_node(&assign.right)?;
        
        // Check if left side is an identifier
        if let Node::Identifier(var_name) = &*assign.left {
            let current_scope = self.scope_stack.last().unwrap();
            
            if let Some(var_info) = current_scope.get_variable(var_name) {
                if !var_info.mutable {
                    self.errors.push(SemanticError::ConstReassignment {
                        name: var_name.clone(),
                        position: assign.span.as_ref().map(|s| s.start.clone()),
                    });
                }
            } else {
                self.errors.push(SemanticError::UndeclaredVariable {
                    name: var_name.clone(),
                    position: assign.span.as_ref().map(|s| s.start.clone()),
                });
            }
        }
        
        Ok(value_type)
    }
    
    /// Visit if statement
    fn visit_if_statement(&mut self, if_stmt: &v8_ast::IfStatement) -> SemanticResult<Type> {
        let condition_type = self.visit_node(&if_stmt.test)?;
        
        if !condition_type.is_compatible_with(&Type::Boolean) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: "boolean".to_string(),
                found: format!("{:?}", condition_type),
                position: if_stmt.span.as_ref().map(|s| s.start.clone()),
            });
        }
        
        // Create block scope for then branch
        let current_scope = self.scope_stack.last().unwrap().clone();
        let block_scope = Scope::new_child(current_scope, ScopeType::Block);
        self.scope_stack.push(block_scope);
        self.visit_node(&if_stmt.consequent)?;
        self.scope_stack.pop();
        
        // Create block scope for else branch if it exists
        if let Some(alternate) = &if_stmt.alternate {
            let current_scope = self.scope_stack.last().unwrap().clone();
            let block_scope = Scope::new_child(current_scope, ScopeType::Block);
            self.scope_stack.push(block_scope);
            self.visit_node(alternate)?;
            self.scope_stack.pop();
        }
        
        Ok(Type::Undefined)
    }
    
    /// Visit while statement
    fn visit_while_statement(&mut self, while_stmt: &v8_ast::WhileStatement) -> SemanticResult<Type> {
        let condition_type = self.visit_node(&while_stmt.test)?;
        
        if !condition_type.is_compatible_with(&Type::Boolean) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: "boolean".to_string(),
                found: format!("{:?}", condition_type),
                position: while_stmt.span.as_ref().map(|s| s.start.clone()),
            });
        }
        
        // Create block scope for body
        let current_scope = self.scope_stack.last().unwrap().clone();
        let block_scope = Scope::new_child(current_scope, ScopeType::Block);
        self.scope_stack.push(block_scope);
        self.visit_node(&while_stmt.body)?;
        self.scope_stack.pop();
        
        Ok(Type::Undefined)
    }
    
    /// Visit return statement
    fn visit_return_statement(&mut self, return_stmt: &v8_ast::ReturnStatement) -> SemanticResult<Type> {
        if let Some(argument) = &return_stmt.argument {
            self.visit_node(argument)
        } else {
            Ok(Type::Undefined)
        }
    }
    
    /// Visit block statement
    fn visit_block_statement(&mut self, block: &v8_ast::BlockStatement) -> SemanticResult<Type> {
        // Create block scope
        let current_scope = self.scope_stack.last().unwrap().clone();
        let block_scope = Scope::new_child(current_scope, ScopeType::Block);
        self.scope_stack.push(block_scope);
        
        let mut last_type = Type::Undefined;
        
        for statement in &block.body {
            last_type = self.visit_node(statement)?;
        }
        
        // Pop block scope
        self.scope_stack.pop();
        
        Ok(last_type)
    }
    
    /// Visit array literal
    fn visit_array_literal(&mut self, array: &v8_ast::ArrayLiteral) -> SemanticResult<Type> {
        let mut element_types = Vec::new();
        
        for element in &array.elements {
            if let Some(elem) = element {
                let elem_type = self.visit_node(elem)?;
                element_types.push(elem_type);
            }
        }
        
        // Determine array type based on elements
        if element_types.is_empty() {
            Ok(Type::Array(Box::new(Type::Any)))
        } else {
            let common_type = element_types.iter().fold(Type::Any, |acc, t| acc.common_type(t));
            Ok(Type::Array(Box::new(common_type)))
        }
    }
    
    /// Visit object literal
    fn visit_object_literal(&mut self, obj: &v8_ast::ObjectLiteral) -> SemanticResult<Type> {
        for property in &obj.properties {
            self.visit_node(property)?;
        }
        Ok(Type::Object)
    }
    
    /// Visit property
    fn visit_property(&mut self, prop: &v8_ast::Property) -> SemanticResult<Type> {
        // For property keys, we don't need to analyze them as variables
        // They are just property names, not variable references
        let _key_type = match &*prop.key {
            Node::Identifier(_) => Ok(Type::String), // Property names are strings
            _ => self.visit_node(&prop.key),
        }?;
        
        let value_type = self.visit_node(&prop.value)?;
        
        Ok(value_type)
    }
    
    /// Visit member expression
    fn visit_member_expression(&mut self, member: &v8_ast::MemberExpression) -> SemanticResult<Type> {
        let _object_type = self.visit_node(&member.object)?;
        
        // For property access, we don't need to analyze the property name as a variable
        // It's just a property name, not a variable reference
        let _property_type = match &*member.property {
            Node::Identifier(_) => Ok(Type::String), // Property names are strings
            _ => self.visit_node(&member.property),
        }?;
        
        // For now, return Any for member access
        // In a more sophisticated implementation, we'd track object shapes
        Ok(Type::Any)
    }
    
    /// Visit logical expression
    fn visit_logical_expression(&mut self, logical: &v8_ast::LogicalExpression) -> SemanticResult<Type> {
        let left_type = self.visit_node(&logical.left)?;
        let right_type = self.visit_node(&logical.right)?;
        
        match logical.operator.as_str() {
            "&&" | "||" => {
                // Logical operators can return either operand type
                Ok(left_type.common_type(&right_type))
            }
            _ => Ok(Type::Boolean),
        }
    }
    
    /// Visit conditional expression (ternary)
    fn visit_conditional_expression(&mut self, conditional: &v8_ast::ConditionalExpression) -> SemanticResult<Type> {
        let test_type = self.visit_node(&conditional.test)?;
        
        if !test_type.is_compatible_with(&Type::Boolean) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: "boolean".to_string(),
                found: format!("{:?}", test_type),
                position: conditional.span.as_ref().map(|s| s.start.clone()),
            });
        }
        
        let consequent_type = self.visit_node(&conditional.consequent)?;
        let alternate_type = self.visit_node(&conditional.alternate)?;
        
        Ok(consequent_type.common_type(&alternate_type))
    }
    
    /// Visit arrow function expression
    fn visit_arrow_function_expression(&mut self, arrow: &v8_ast::ArrowFunctionExpression) -> SemanticResult<Type> {
        // Create function scope
        let current_scope = self.scope_stack.last().unwrap().clone();
        let function_scope = Scope::new_child(current_scope, ScopeType::Function);
        self.scope_stack.push(function_scope);
        
        // Declare parameters in function scope
        for param in &arrow.params {
            if let Node::Identifier(param_name) = param {
                let current_scope = self.scope_stack.last_mut().unwrap();
                current_scope.declare_variable(
                    param_name,
                    Type::Any, // TODO: Infer parameter types
                    true, // Parameters are mutable
                    1, // TODO: Get actual line number
                );
                current_scope.initialize_variable(param_name);
            }
        }
        
        // Analyze function body
        let return_type = self.visit_node(&arrow.body)?;
        
        // Pop function scope
        self.scope_stack.pop();
        
        Ok(Type::Function {
            params: vec![], // TODO: Get actual parameter types
            return_type: Box::new(return_type),
        })
    }
} 