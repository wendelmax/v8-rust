//! Main parser implementation

use crate::error::{ParseError, ParseResult};
use crate::recovery::{ErrorRecovery, RecoveryContext, RecoveryStrategy, ParsingContext};
use v8_ast::{
    Node, Position, Span, Program, VariableDeclaration, VariableDeclarator, FunctionDeclaration, 
    FunctionExpression, ClassDeclaration, ClassExpression, IfStatement, WhileStatement, 
    DoWhileStatement, ForStatement, SwitchStatement, SwitchCase, TryStatement, CatchClause, 
    WithStatement, DebuggerStatement, ReturnStatement, BreakStatement, ContinueStatement, 
    ThrowStatement, ExpressionStatement, BlockStatement, BinaryExpression, LogicalExpression,
    UnaryExpression, UpdateExpression, AssignmentExpression, CallExpression, NewExpression, 
    MemberExpression, ArrayLiteral, ObjectLiteral, Property, Super, ImportDeclaration, 
    ExportDeclaration, ArrowFunctionExpression, SpreadElement,
};
use v8_lexer::{Lexer, Token, TokenKind};

/// Main parser for JavaScript/ECMAScript
pub struct Parser {
    /// Source code being parsed
    source: String,
    
    /// Lexer for tokenization
    lexer: Lexer,
    
    /// Current token
    current: Option<Token>,
    
    /// Previous token
    previous: Option<Token>,
    
    /// Error recovery manager
    error_recovery: ErrorRecovery,
    
    /// Current parsing context
    context: ParsingContext,
    
    /// Whether we're in strict mode
    strict_mode: bool,
}

impl Parser {
    /// Create a new parser
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = match lexer.next_token() {
            Ok(token) => Some(token),
            Err(_) => None,
        };
        
        Self {
            source: source.to_string(),
            lexer,
            current,
            previous: None,
            error_recovery: ErrorRecovery::default(),
            context: ParsingContext::TopLevel,
            strict_mode: false,
        }
    }

    /// Parse the entire source code
    pub fn parse(&mut self) -> ParseResult<Node> {
        self.parse_program()
    }

    /// Parse with error recovery
    pub fn parse_with_recovery(&mut self) -> (Option<Node>, Vec<ParseError>) {
        match self.parse() {
            Ok(ast) => (Some(ast), self.error_recovery.errors().to_vec()),
            Err(error) => {
                self.error_recovery.add_error(error);
                (None, self.error_recovery.errors().to_vec())
            }
        }
    }

    /// Parse a program (top-level)
    fn parse_program(&mut self) -> ParseResult<Node> {
        let mut body = Vec::new();
        let start_pos = self.current_position();
        
        // Handle empty input
        if self.is_eof() {
            let end_pos = self.previous_position();
            let span = self.create_span(start_pos, end_pos);
            return Ok(Node::Program(Program {
                body,
                source_type: "script".to_string(),
                span: Some(span),
            }));
        }
        
        while !self.is_eof() {
            match self.parse_statement() {
                Ok(stmt) => body.push(stmt),
                Err(error) => {
                    if !self.try_recover_from_error(error.clone()) {
                        return Err(error);
                    }
                    // If we're still at EOF after recovery, break
                    if self.is_eof() {
                        break;
                    }
                }
            }
        }
        
        let end_pos = self.previous_position();
        let span = self.create_span(start_pos, end_pos);
        
        Ok(Node::Program(Program {
            body,
            source_type: "script".to_string(),
            span: Some(span),
        }))
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> ParseResult<Node> {
        let old_context = self.context.clone();
        self.context = ParsingContext::Statement;
        
        let result = if let Some(token) = &self.current {
            match &token.kind {
                TokenKind::Keyword(kw) => match kw.as_str() {
                    "let" | "const" | "var" => self.parse_declaration(),
                    "function" => self.parse_function_declaration(),
                    "class" => self.parse_class_declaration(),
                    "if" => self.parse_if_statement(),
                    "while" => self.parse_while_statement(),
                    "for" => self.parse_for_statement(),
                    "return" => self.parse_return_statement(),
                    "break" => self.parse_break_statement(),
                    "continue" => self.parse_continue_statement(),
                    "throw" => self.parse_throw_statement(),
                    "try" => self.parse_try_statement(),
                    "switch" => self.parse_switch_statement(),
                    "do" => self.parse_do_while_statement(),
                    "with" => self.parse_with_statement(),
                    "debugger" => self.parse_debugger_statement(),
                    "import" => self.parse_import_declaration(),
                    "export" => self.parse_export_declaration(),
                    _ => self.parse_expression_statement(),
                },
                TokenKind::LeftBrace => self.parse_block_statement(),
                TokenKind::Semicolon => self.parse_empty_statement(),
                _ => self.parse_expression_statement(),
            }
        } else {
            Err(ParseError::unexpected_end_of_input(None))
        };
        
        self.context = old_context;
        result
    }

    /// Parse a declaration
    fn parse_declaration(&mut self) -> ParseResult<Node> {
        if let Some(token) = &self.current {
            match &token.kind {
                TokenKind::Keyword(kw) => match kw.as_str() {
                    "let" | "const" | "var" => self.parse_variable_declaration(),
                    "function" => self.parse_function_declaration(),
                    "class" => self.parse_class_declaration(),
                    _ => Err(ParseError::invalid_declaration(
                        "Expected declaration",
                        self.current_position().unwrap_or_default(),
                    )),
                },
                _ => Err(ParseError::invalid_declaration(
                    "Expected declaration",
                    self.current_position().unwrap_or_default(),
                )),
            }
        } else {
            Err(ParseError::unexpected_end_of_input(None))
        }
    }

    /// Parse a variable declaration
    fn parse_variable_declaration(&mut self) -> ParseResult<Node> {
        let kind = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                match kw.as_str() {
                    "let" => "let",
                    "const" => "const", 
                    "var" => "var",
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        };
        
        self.advance(); // Consume let/const/var
        
        let mut declarations = Vec::new();
        
        loop {
            let id = self.parse_identifier()?;
            let init = if self.check(TokenKind::Assign) {
                self.advance(); // Consume =
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };
            
            let span = self.create_span_from_tokens();
            declarations.push(VariableDeclarator {
                id: Box::new(id),
                init,
                span: Some(span),
            });
            
            if !self.check(TokenKind::Comma) {
                break;
            }
            self.advance(); // Consume comma
        }
        
        // Consume semicolon if present
        if self.check(TokenKind::Semicolon) {
            self.advance();
        }
        
        let span = self.create_span_from_tokens();
        Ok(Node::VariableDeclaration(VariableDeclaration {
            kind: kind.to_string(),
            declarations,
            span: Some(span),
        }))
    }

    /// Parse a function declaration
    fn parse_function_declaration(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'function'
        
        let id = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };
        
        self.expect(TokenKind::LeftParen)?;
        let params = self.parse_parameters()?;
        self.expect(TokenKind::RightParen)?;
        
        let body = Box::new(self.parse_function_body()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::FunctionDeclaration(FunctionDeclaration {
            id,
            params,
            body,
            generator: false,
            r#async: false,
            span: Some(span),
        }))
    }

    /// Parse a class declaration
    fn parse_class_declaration(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'class'
        
        let id = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };
        
        let super_class = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "extends" {
                    self.advance(); // Consume 'extends'
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        let body = Box::new(self.parse_class_body()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::ClassDeclaration(ClassDeclaration {
            id,
            super_class,
            body,
            span: Some(span),
        }))
    }

    /// Parse an if statement
    fn parse_if_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'if'
        
        self.expect(TokenKind::LeftParen)?;
        let test = Box::new(self.parse_expression()?);
        self.expect(TokenKind::RightParen)?;
        
        let consequent = Box::new(self.parse_statement()?);
        
        let alternate = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "else" {
                    self.advance(); // Consume 'else'
                    Some(Box::new(self.parse_statement()?))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        let span = self.create_span_from_tokens();
        Ok(Node::IfStatement(IfStatement {
            test,
            consequent,
            alternate,
            span: Some(span),
        }))
    }

    /// Parse a while statement
    fn parse_while_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'while'
        
        self.expect(TokenKind::LeftParen)?;
        let test = Box::new(self.parse_expression()?);
        self.expect(TokenKind::RightParen)?;
        
        let body = Box::new(self.parse_statement()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::WhileStatement(WhileStatement {
            test,
            body,
            span: Some(span),
        }))
    }

    /// Parse a for statement
    fn parse_for_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'for'
        
        self.expect(TokenKind::LeftParen)?;
        
        let init = if !self.check(TokenKind::Semicolon) {
            Some(Box::new(if self.is_declaration() {
                self.parse_declaration()?
            } else {
                self.parse_expression()?
            }))
        } else {
            None
        };
        
        self.expect(TokenKind::Semicolon)?;
        
        let test = if !self.check(TokenKind::Semicolon) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        
        self.expect(TokenKind::Semicolon)?;
        
        let update = if !self.check(TokenKind::RightParen) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        
        self.expect(TokenKind::RightParen)?;
        
        let body = Box::new(self.parse_statement()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::ForStatement(ForStatement {
            init,
            test,
            update,
            body,
            span: Some(span),
        }))
    }

    /// Parse a return statement
    fn parse_return_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'return'
        
        let argument = if !self.check(TokenKind::Semicolon) && !self.is_eof() {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        
        let span = self.create_span_from_tokens();
        Ok(Node::ReturnStatement(ReturnStatement {
            argument,
            span: Some(span),
        }))
    }

    /// Parse a break statement
    fn parse_break_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'break'
        
        let label = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };
        
        let span = self.create_span_from_tokens();
        Ok(Node::BreakStatement(BreakStatement {
            label,
            span: Some(span),
        }))
    }

    /// Parse a continue statement
    fn parse_continue_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'continue'
        
        let label = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };
        
        let span = self.create_span_from_tokens();
        Ok(Node::ContinueStatement(ContinueStatement {
            label,
            span: Some(span),
        }))
    }

    /// Parse a throw statement
    fn parse_throw_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'throw'
        
        let argument = Box::new(self.parse_expression()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::ThrowStatement(ThrowStatement {
            argument,
            span: Some(span),
        }))
    }

    /// Parse a try statement
    fn parse_try_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'try'
        
        let block = Box::new(self.parse_block_statement()?);
        
        let handler = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "catch" {
                    Some(Box::new(self.parse_catch_clause()?))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        let finalizer = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "finally" {
                    self.advance(); // Consume 'finally'
                    Some(Box::new(self.parse_block_statement()?))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        let span = self.create_span_from_tokens();
        Ok(Node::TryStatement(TryStatement {
            block,
            handler,
            finalizer,
            span: Some(span),
        }))
    }

    /// Parse a catch clause
    fn parse_catch_clause(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'catch'
        
        self.expect(TokenKind::LeftParen)?;
        let param = Box::new(self.parse_identifier()?);
        self.expect(TokenKind::RightParen)?;
        
        let body = Box::new(self.parse_block_statement()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::CatchClause(CatchClause {
            param,
            body,
            span: Some(span),
        }))
    }

    /// Parse a switch statement
    fn parse_switch_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'switch'
        
        self.expect(TokenKind::LeftParen)?;
        let discriminant = Box::new(self.parse_expression()?);
        self.expect(TokenKind::RightParen)?;
        
        self.expect(TokenKind::LeftBrace)?;
        
        let mut cases = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_eof() {
            cases.push(self.parse_switch_case()?);
        }
        
        self.expect(TokenKind::RightBrace)?;
        
        let span = self.create_span_from_tokens();
        Ok(Node::SwitchStatement(SwitchStatement {
            discriminant,
            cases,
            span: Some(span),
        }))
    }

    /// Parse a switch case
    fn parse_switch_case(&mut self) -> ParseResult<SwitchCase> {
        let test = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "default" {
                    self.advance(); // Consume 'default'
                    None
                } else {
                    self.advance(); // Consume 'case'
                    Some(Box::new(self.parse_expression()?))
                }
            } else {
                self.advance(); // Consume 'case'
                Some(Box::new(self.parse_expression()?))
            }
        } else {
            None
        };
        
        self.expect(TokenKind::Colon)?;
        
        let mut consequent = Vec::new();
        while !self.check(TokenKind::Keyword("case".to_string())) && 
              !self.check(TokenKind::Keyword("default".to_string())) && 
              !self.check(TokenKind::RightBrace) && 
              !self.is_eof() {
            consequent.push(self.parse_statement()?);
        }
        
        let span = self.create_span_from_tokens();
        Ok(SwitchCase {
            test,
            consequent,
            span: Some(span),
        })
    }

    /// Parse a do-while statement
    fn parse_do_while_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'do'
        
        let body = Box::new(self.parse_statement()?);
        
        if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "while" {
                    self.advance(); // Consume 'while'
                    self.expect(TokenKind::LeftParen)?;
                    let test = Box::new(self.parse_expression()?);
                    self.expect(TokenKind::RightParen)?;
                    
                    let span = self.create_span_from_tokens();
                    return Ok(Node::DoWhileStatement(DoWhileStatement {
                        body,
                        test,
                        span: Some(span),
                    }));
                }
            }
        }
        
        Err(ParseError::invalid_statement(
            "Expected 'while' after 'do'",
            self.current_position().unwrap_or_default(),
        ))
    }

    /// Parse a with statement
    fn parse_with_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'with'
        
        self.expect(TokenKind::LeftParen)?;
        let object = Box::new(self.parse_expression()?);
        self.expect(TokenKind::RightParen)?;
        
        let body = Box::new(self.parse_statement()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::WithStatement(WithStatement {
            object,
            body,
            span: Some(span),
        }))
    }

    /// Parse a debugger statement
    fn parse_debugger_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'debugger'
        
        let span = self.create_span_from_tokens();
        Ok(Node::DebuggerStatement(DebuggerStatement {
            span: Some(span),
        }))
    }

    /// Parse a block statement
    fn parse_block_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume '{'
        
        let old_context = self.context.clone();
        self.context = ParsingContext::Block;
        
        let mut body = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_eof() {
            match self.parse_statement() {
                Ok(stmt) => body.push(stmt),
                Err(error) => {
                    if !self.try_recover_from_error(error.clone()) {
                        self.context = old_context;
                        return Err(error);
                    }
                }
            }
        }
        
        self.expect(TokenKind::RightBrace)?;
        
        self.context = old_context;
        
        let span = self.create_span_from_tokens();
        Ok(Node::BlockStatement(BlockStatement {
            body,
            span: Some(span),
        }))
    }

    /// Parse an empty statement
    fn parse_empty_statement(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume ';'
        
        let span = self.create_span_from_tokens();
        // Return a simple ExpressionStatement with a null expression
        Ok(Node::ExpressionStatement(ExpressionStatement {
            expression: Box::new(Node::Null),
            span: Some(span),
        }))
    }

    /// Parse an expression statement
    fn parse_expression_statement(&mut self) -> ParseResult<Node> {
        let expression = Box::new(self.parse_expression()?);
        
        // Consume semicolon if present
        if self.check(TokenKind::Semicolon) {
            self.advance();
        }
        
        let span = self.create_span_from_tokens();
        Ok(Node::ExpressionStatement(ExpressionStatement {
            expression,
            span: Some(span),
        }))
    }

    /// Parse an import declaration
    fn parse_import_declaration(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'import'
        
        // This is a simplified implementation
        // Full import parsing would be more complex
        
        let span = self.create_span_from_tokens();
        Ok(Node::ImportDeclaration(ImportDeclaration {
            specifiers: Vec::new(),
            source: Box::new(Node::String("".to_string())),
            span: Some(span),
        }))
    }

    /// Parse an export declaration
    fn parse_export_declaration(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'export'
        
        // This is a simplified implementation
        // Full export parsing would be more complex
        
        let span = self.create_span_from_tokens();
        Ok(Node::ExportDeclaration(ExportDeclaration {
            declaration: None,
            specifiers: Vec::new(),
            source: None,
            default: false,
            span: Some(span),
        }))
    }

    /// Parse an expression
    fn parse_expression(&mut self) -> ParseResult<Node> {
        self.parse_assignment_expression()
    }

    /// Parse an assignment expression
    fn parse_assignment_expression(&mut self) -> ParseResult<Node> {
        let left = self.parse_logical_or_expression()?;
        
        if self.is_assignment_operator() {
            let operator = self.current_token_string();
            self.advance(); // Consume operator
            let right = Box::new(self.parse_assignment_expression()?);
            
            let span = self.create_span_from_tokens();
            Ok(Node::AssignmentExpression(AssignmentExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            }))
        } else if self.check(TokenKind::Arrow) {
            // Arrow function expression
            self.parse_arrow_function_expression(false)
        } else {
            Ok(left)
        }
    }

    /// Parse a logical OR expression (including nullish coalescing)
    fn parse_logical_or_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_logical_and_expression()?;
        
        while self.is_logical_or_operator() {
            let operator = self.current_token_string();
            self.advance(); // Consume operator
            let right = Box::new(self.parse_logical_and_expression()?);
            
            let span = self.create_span_from_tokens();
            left = Node::LogicalExpression(LogicalExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }
        
        Ok(left)
    }
    
    /// Check if the current token is a logical OR operator
    fn is_logical_or_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind,
                TokenKind::LogicalOr | TokenKind::NullishCoalescing
            )
        } else {
            false
        }
    }

    /// Parse a logical AND expression
    fn parse_logical_and_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_equality_expression()?;
        
        while self.check(TokenKind::LogicalAnd) {
            let operator = self.current_token_string();
            self.advance(); // Consume operator
            let right = Box::new(self.parse_equality_expression()?);
            
            let span = self.create_span_from_tokens();
            left = Node::LogicalExpression(LogicalExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }
        
        Ok(left)
    }

    /// Parse an equality expression
    fn parse_equality_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_relational_expression()?;
        
        while self.is_equality_operator() {
            let operator = self.current_token_string();
            self.advance(); // Consume operator
            let right = Box::new(self.parse_relational_expression()?);
            
            let span = self.create_span_from_tokens();
            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }
        
        Ok(left)
    }

    /// Parse a relational expression
    fn parse_relational_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_shift_expression()?;
        
        while self.is_relational_operator() {
            let operator = self.current_token_string();
            self.advance(); // Consume operator
            let right = Box::new(self.parse_shift_expression()?);
            
            let span = self.create_span_from_tokens();
            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }
        
        Ok(left)
    }

    /// Parse a shift expression
    fn parse_shift_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_additive_expression()?;
        
        while self.is_shift_operator() {
            let operator = self.current_token_string();
            self.advance(); // Consume operator
            let right = Box::new(self.parse_additive_expression()?);
            
            let span = self.create_span_from_tokens();
            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }
        
        Ok(left)
    }

    /// Parse an additive expression
    fn parse_additive_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_multiplicative_expression()?;
        
        while self.is_additive_operator() {
            let operator = self.current_token_string();
            self.advance(); // Consume operator
            let right = Box::new(self.parse_multiplicative_expression()?);
            
            let span = self.create_span_from_tokens();
            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }
        
        Ok(left)
    }

    /// Parse a multiplicative expression
    fn parse_multiplicative_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_unary_expression()?;
        
        while self.is_multiplicative_operator() {
            let operator = self.current_token_string();
            self.advance(); // Consume operator
            let right = Box::new(self.parse_unary_expression()?);
            
            let span = self.create_span_from_tokens();
            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }
        
        Ok(left)
    }

    /// Parse a unary expression
    fn parse_unary_expression(&mut self) -> ParseResult<Node> {
        if self.is_unary_operator() {
            let operator = self.current_token_string();
            let prefix = true;
            self.advance(); // Consume operator
            let argument = Box::new(self.parse_unary_expression()?);
            
            let span = self.create_span_from_tokens();
            return Ok(Node::UnaryExpression(UnaryExpression {
                operator,
                argument,
                prefix,
                span: Some(span),
            }));
        }
        
        self.parse_postfix_expression()
    }

    /// Parse a postfix expression
    fn parse_postfix_expression(&mut self) -> ParseResult<Node> {
        let mut expr = self.parse_primary_expression()?;
        
        loop {
            if let Some(token) = &self.current {
                match &token.kind {
                    TokenKind::LeftBracket => {
                        self.advance(); // Consume '['
                        let property = Box::new(self.parse_expression()?);
                        self.expect(TokenKind::RightBracket)?;
                        
                        let span = self.create_span_from_tokens();
                        expr = Node::MemberExpression(MemberExpression {
                            object: Box::new(expr),
                            property,
                            computed: true,
                            optional: false,
                            span: Some(span),
                        });
                    }
                    
                    TokenKind::Dot => {
                        self.advance(); // Consume '.'
                        let property = Box::new(self.parse_identifier()?);
                        
                        let span = self.create_span_from_tokens();
                        expr = Node::MemberExpression(MemberExpression {
                            object: Box::new(expr),
                            property,
                            computed: false,
                            optional: false,
                            span: Some(span),
                        });
                    }
                    
                    TokenKind::LeftParen => {
                        self.advance(); // Consume '('
                        let arguments = self.parse_arguments()?;
                        self.expect(TokenKind::RightParen)?;
                        
                        let span = self.create_span_from_tokens();
                        expr = Node::CallExpression(CallExpression {
                            callee: Box::new(expr),
                            arguments,
                            span: Some(span),
                        });
                    }
                    
                    TokenKind::Increment | TokenKind::Decrement => {
                        let operator = self.current_token_string();
                        let prefix = false;
                        self.advance(); // Consume operator
                        
                        let span = self.create_span_from_tokens();
                        expr = Node::UpdateExpression(UpdateExpression {
                            operator,
                            argument: Box::new(expr),
                            prefix,
                            span: Some(span),
                        });
                    }
                    
                    _ => break,
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    /// Parse a primary expression
    fn parse_primary_expression(&mut self) -> ParseResult<Node> {
        if let Some(token) = &self.current {
            match &token.kind {
                TokenKind::Number(n) => {
                    let value = *n;
                    self.advance();
                    Ok(Node::Number(value))
                }
                TokenKind::String(s) => {
                    let value = s.clone();
                    self.advance();
                    Ok(Node::String(value))
                }
                TokenKind::TemplateString(s) => {
                    let value = s.clone();
                    self.advance();
                    // For now, treat template strings as regular strings
                    // TODO: Implement proper template literal parsing with expressions
                    Ok(Node::String(value))
                }
                TokenKind::Boolean(b) => {
                    let value = *b;
                    self.advance();
                    Ok(Node::Boolean(value))
                }
                TokenKind::Null => {
                    self.advance();
                    Ok(Node::Null)
                }
                TokenKind::Undefined => {
                    self.advance();
                    Ok(Node::Undefined)
                }
                TokenKind::Keyword(kw) if kw == "this" => {
                    self.advance();
                    Ok(Node::This)
                }
                TokenKind::LeftParen => {
                    self.advance(); // Consume '('
                    let expr = self.parse_expression()?;
                    self.expect(TokenKind::RightParen)?;
                    Ok(expr)
                }
                TokenKind::LeftBracket => {
                    self.parse_array_literal()
                }
                TokenKind::LeftBrace => {
                    self.parse_object_literal()
                }
                TokenKind::Keyword(kw) if kw == "function" => {
                    self.parse_function_expression()
                }
                TokenKind::Keyword(kw) if kw == "class" => {
                    self.parse_class_expression()
                }
                TokenKind::Keyword(kw) if kw == "new" => {
                    self.parse_new_expression()
                }
                TokenKind::Keyword(kw) if kw == "async" => {
                    // Check if next token is function or =>
                    self.advance(); // Consume 'async'
                    if self.check(TokenKind::Keyword("function".to_string())) {
                        self.parse_function_expression()
                    } else {
                        // Async arrow function
                        self.parse_arrow_function_expression(true)
                    }
                }
                _ => {
                    if self.check_identifier() {
                        self.parse_identifier()
                    } else {
                        Err(ParseError::invalid_expression(
                            "Unexpected token in expression",
                            self.current_position().unwrap_or_default(),
                        ))
                    }
                }
            }
        } else {
            Err(ParseError::unexpected_end_of_input(None))
        }
    }

    /// Parse an array literal
    fn parse_array_literal(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume '['
        
        let mut elements = Vec::new();
        
        while !self.check(TokenKind::RightBracket) && !self.is_eof() {
            if self.check(TokenKind::Comma) {
                elements.push(None); // Empty slot
                self.advance(); // Consume comma
            } else {
                elements.push(Some(self.parse_expression()?));
                
                if self.check(TokenKind::Comma) {
                    self.advance(); // Consume comma
                }
            }
        }
        
        self.expect(TokenKind::RightBracket)?;
        
        let span = self.create_span_from_tokens();
        Ok(Node::ArrayLiteral(ArrayLiteral {
            elements,
            span: Some(span),
        }))
    }

    /// Parse an object literal
    fn parse_object_literal(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume '{'
        
        let mut properties = Vec::new();
        
        while !self.check(TokenKind::RightBrace) && !self.is_eof() {
            properties.push(self.parse_property()?);
            
            if self.check(TokenKind::Comma) {
                self.advance(); // Consume comma
            }
        }
        
        self.expect(TokenKind::RightBrace)?;
        
        let span = self.create_span_from_tokens();
        Ok(Node::ObjectLiteral(ObjectLiteral {
            properties,
            span: Some(span),
        }))
    }

    /// Parse a property
    fn parse_property(&mut self) -> ParseResult<Node> {
        let key = if self.check_identifier() {
            Box::new(self.parse_identifier()?)
        } else if let Some(token) = &self.current {
            if let TokenKind::String(_) = &token.kind {
                Box::new(self.parse_primary_expression()?)
            } else {
                return Err(ParseError::invalid_syntax(
                    "Expected identifier or string literal",
                    self.current_position().unwrap_or_default(),
                ));
            }
        } else {
            return Err(ParseError::unexpected_end_of_input(None));
        };
        
        self.expect(TokenKind::Colon)?;
        let value = Box::new(self.parse_expression()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::Property(Property {
            key,
            value,
            kind: "init".to_string(),
            computed: false,
            method: false,
            shorthand: false,
            span: Some(span),
        }))
    }

    /// Parse a function expression
    fn parse_function_expression(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'function'
        
        let id = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };
        
        self.expect(TokenKind::LeftParen)?;
        let params = self.parse_parameters()?;
        self.expect(TokenKind::RightParen)?;
        
        let body = Box::new(self.parse_function_body()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::FunctionExpression(FunctionExpression {
            id,
            params,
            body,
            generator: false,
            r#async: false,
            span: Some(span),
        }))
    }

    /// Parse a class expression
    fn parse_class_expression(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'class'
        
        let id = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };
        
        let super_class = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "extends" {
                    self.advance(); // Consume 'extends'
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        let body = Box::new(self.parse_class_body()?);
        
        let span = self.create_span_from_tokens();
        Ok(Node::ClassExpression(ClassExpression {
            id,
            super_class,
            body,
            span: Some(span),
        }))
    }

    /// Parse a new expression
    fn parse_new_expression(&mut self) -> ParseResult<Node> {
        self.advance(); // Consume 'new'
        
        let callee = Box::new(self.parse_primary_expression()?);
        
        let arguments = if self.check(TokenKind::LeftParen) {
            self.advance(); // Consume '('
            let args = self.parse_arguments()?;
            self.expect(TokenKind::RightParen)?;
            args
        } else {
            Vec::new()
        };
        
        let span = self.create_span_from_tokens();
        Ok(Node::NewExpression(NewExpression {
            callee,
            arguments,
            span: Some(span),
        }))
    }

    /// Parse function parameters
    fn parse_parameters(&mut self) -> ParseResult<Vec<Node>> {
        let mut params = Vec::new();
        
        while !self.check(TokenKind::RightParen) && !self.is_eof() {
            params.push(self.parse_identifier()?);
            
            if self.check(TokenKind::Comma) {
                self.advance(); // Consume comma
            }
        }
        
        Ok(params)
    }

    /// Parse function arguments
    fn parse_arguments(&mut self) -> ParseResult<Vec<Node>> {
        let mut arguments = Vec::new();
        
        while !self.check(TokenKind::RightParen) && !self.is_eof() {
            arguments.push(self.parse_expression()?);
            
            if self.check(TokenKind::Comma) {
                self.advance(); // Consume comma
            }
        }
        
        Ok(arguments)
    }

    /// Parse function body
    fn parse_function_body(&mut self) -> ParseResult<Node> {
        self.parse_block_statement()
    }

    /// Parse class body
    fn parse_class_body(&mut self) -> ParseResult<Node> {
        self.expect(TokenKind::LeftBrace)?;
        
        let mut body = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_eof() {
            // Simplified class body parsing
            body.push(self.parse_statement()?);
        }
        
        self.expect(TokenKind::RightBrace)?;
        
        let span = self.create_span_from_tokens();
        Ok(Node::BlockStatement(BlockStatement {
            body,
            span: Some(span),
        }))
    }

    /// Parse an identifier
    fn parse_identifier(&mut self) -> ParseResult<Node> {
        if let Some(token) = &self.current {
            if let TokenKind::Identifier(ident) = &token.kind {
                let name = ident.clone();
                self.advance();
                Ok(Node::Identifier(name))
            } else {
                Err(ParseError::invalid_syntax(
                    "Expected identifier",
                    self.current_position().unwrap_or_default(),
                ))
            }
        } else {
            Err(ParseError::unexpected_end_of_input(None))
        }
    }

    /// Get the current token
    fn current_token(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    /// Get the current token as a string
    fn current_token_string(&self) -> String {
        if let Some(token) = &self.current {
            match &token.kind {
                TokenKind::Plus => "+".to_string(),
                TokenKind::Minus => "-".to_string(),
                TokenKind::Star => "*".to_string(),
                TokenKind::Slash => "/".to_string(),
                TokenKind::Percent => "%".to_string(),
                TokenKind::StarStar => "**".to_string(),
                TokenKind::Equal => "==".to_string(),
                TokenKind::NotEqual => "!=".to_string(),
                TokenKind::StrictEqual => "===".to_string(),
                TokenKind::StrictNotEqual => "!==".to_string(),
                TokenKind::LessThan => "<".to_string(),
                TokenKind::GreaterThan => ">".to_string(),
                TokenKind::LessThanEqual => "<=".to_string(),
                TokenKind::GreaterThanEqual => ">=".to_string(),
                TokenKind::LeftShift => "<<".to_string(),
                TokenKind::RightShift => ">>".to_string(),
                TokenKind::UnsignedRightShift => ">>>".to_string(),
                TokenKind::Assign => "=".to_string(),
                TokenKind::PlusAssign => "+=".to_string(),
                TokenKind::MinusAssign => "-=".to_string(),
                TokenKind::StarAssign => "*=".to_string(),
                TokenKind::SlashAssign => "/=".to_string(),
                TokenKind::PercentAssign => "%=".to_string(),
                TokenKind::StarStarAssign => "**=".to_string(),
                TokenKind::LeftShiftAssign => "<<=".to_string(),
                TokenKind::RightShiftAssign => ">>=".to_string(),
                TokenKind::UnsignedRightShiftAssign => ">>>=".to_string(),
                TokenKind::BitwiseAndAssign => "&=".to_string(),
                TokenKind::BitwiseOrAssign => "|=".to_string(),
                TokenKind::BitwiseXorAssign => "^=".to_string(),
                TokenKind::LogicalAnd => "&&".to_string(),
                TokenKind::LogicalOr => "||".to_string(),
                TokenKind::Exclamation => "!".to_string(),
                TokenKind::BitwiseAnd => "&".to_string(),
                TokenKind::BitwiseOr => "|".to_string(),
                TokenKind::BitwiseXor => "^".to_string(),
                TokenKind::Tilde => "~".to_string(),
                TokenKind::Increment => "++".to_string(),
                TokenKind::Decrement => "--".to_string(),
                TokenKind::Question => "?".to_string(),
                TokenKind::Colon => ":".to_string(),
                TokenKind::Comma => ",".to_string(),
                TokenKind::Semicolon => ";".to_string(),
                TokenKind::Dot => ".".to_string(),
                TokenKind::LeftParen => "(".to_string(),
                TokenKind::RightParen => ")".to_string(),
                TokenKind::LeftBracket => "[".to_string(),
                TokenKind::RightBracket => "]".to_string(),
                TokenKind::LeftBrace => "{".to_string(),
                TokenKind::RightBrace => "}".to_string(),
                TokenKind::Arrow => "=>".to_string(),
                TokenKind::Spread => "...".to_string(),
                TokenKind::NullishCoalescing => "??".to_string(),
                TokenKind::OptionalChaining => "?.".to_string(),
                TokenKind::Identifier(id) => id.clone(),
                TokenKind::String(s) => s.clone(),
                TokenKind::Number(n) => n.to_string(),
                TokenKind::Keyword(kw) => kw.clone(),
                TokenKind::Boolean(b) => b.to_string(),
                TokenKind::Eof => "EOF".to_string(),
                _ => format!("{:?}", token.kind),
            }
        } else {
            "EOF".to_string()
        }
    }

    /// Check if the current token matches the given token kind
    fn check(&self, token_kind: TokenKind) -> bool {
        if let Some(token) = &self.current {
            std::mem::discriminant(&token.kind) == std::mem::discriminant(&token_kind)
        } else {
            false
        }
    }

    /// Check if the current token is an identifier
    fn check_identifier(&self) -> bool {
        self.current_token()
            .map(|t| t.is_identifier())
            .unwrap_or(false)
    }

    /// Expect a specific token kind
    fn expect(&mut self, token_kind: TokenKind) -> ParseResult<()> {
        if self.check(token_kind.clone()) {
            self.advance();
            Ok(())
        } else {
            let current = self.current_token()
                .map(|t| format!("{:?}", t.kind))
                .unwrap_or_else(|| "EOF".to_string());
            Err(ParseError::unexpected_token(
                self.current_token().unwrap_or_else(|| panic!("No current token")),
                Some(&format!("{:?}", token_kind)),
            ))
        }
    }

    /// Advance to the next token
    fn advance(&mut self) {
        self.previous = self.current.take();
        self.current = match self.lexer.next_token() {
            Ok(token) => Some(token),
            Err(_) => None,
        };
    }

    /// Check if we're at the end of input
    fn is_eof(&self) -> bool {
        self.current.is_none() || matches!(self.current.as_ref().map(|t| &t.kind), Some(TokenKind::Eof))
    }

    /// Get the current position
    fn current_position(&self) -> Option<Position> {
        self.current
            .as_ref()
            .map(|t| Position {
                line: t.start().line,
                column: t.start().column,
            })
    }

    /// Get the previous position
    fn previous_position(&self) -> Option<Position> {
        self.previous
            .as_ref()
            .map(|t| Position {
                line: t.end().line,
                column: t.end().column,
            })
    }

    /// Create a span from the current tokens
    fn create_span_from_tokens(&self) -> Span {
        let start = self.previous_position().unwrap_or_default();
        let end = self.current_position().unwrap_or_default();
        Span::new(start, end)
    }

    /// Create a span from positions
    fn create_span(&self, start: Option<Position>, end: Option<Position>) -> Span {
        let start = start.unwrap_or_default();
        let end = end.unwrap_or_default();
        Span::new(start, end)
    }

    /// Check if the current token is a declaration
    fn is_declaration(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind,
                TokenKind::Keyword(ref kw) if kw == "let" || kw == "const" || kw == "var" || kw == "function" || kw == "class"
            )
        } else {
            false
        }
    }

    /// Check if the current token is an assignment operator
    fn is_assignment_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind,
                TokenKind::Assign | TokenKind::PlusAssign | TokenKind::MinusAssign |
                TokenKind::StarAssign | TokenKind::SlashAssign | TokenKind::PercentAssign |
                TokenKind::StarStarAssign | TokenKind::LeftShiftAssign | TokenKind::RightShiftAssign |
                TokenKind::UnsignedRightShiftAssign | TokenKind::BitwiseAndAssign |
                TokenKind::BitwiseOrAssign | TokenKind::BitwiseXorAssign
            )
        } else {
            false
        }
    }

    /// Check if the current token is an equality operator
    fn is_equality_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind,
                TokenKind::Equal | TokenKind::NotEqual | TokenKind::StrictEqual | TokenKind::StrictNotEqual
            )
        } else {
            false
        }
    }

    /// Check if the current token is a relational operator
    fn is_relational_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind,
                TokenKind::LessThan | TokenKind::GreaterThan | TokenKind::LessThanEqual |
                TokenKind::GreaterThanEqual
            ) || matches!(token.kind, TokenKind::Keyword(ref kw) if kw == "instanceof" || kw == "in")
        } else {
            false
        }
    }

    /// Check if the current token is a shift operator
    fn is_shift_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind,
                TokenKind::LeftShift | TokenKind::RightShift | TokenKind::UnsignedRightShift
            )
        } else {
            false
        }
    }

    /// Check if the current token is an additive operator
    fn is_additive_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind, TokenKind::Plus | TokenKind::Minus)
        } else {
            false
        }
    }

    /// Check if the current token is a multiplicative operator
    fn is_multiplicative_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind, TokenKind::Star | TokenKind::Slash | TokenKind::Percent)
        } else {
            false
        }
    }

    /// Check if the current token is a unary operator
    fn is_unary_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind,
                TokenKind::Plus | TokenKind::Minus | TokenKind::Exclamation | TokenKind::Tilde
            ) || matches!(token.kind, TokenKind::Keyword(ref kw) if kw == "typeof" || kw == "void" || kw == "delete")
        } else {
            false
        }
    }

    /// Try to recover from a parsing error
    fn try_recover_from_error(&mut self, error: ParseError) -> bool {
        if !self.error_recovery.can_recover() {
            return false;
        }
        
        self.error_recovery.add_error(error);
        
        let context = RecoveryContext::new(
            self.current.clone(),
            self.previous.clone(),
            self.context.clone(),
        );
        
        let strategy = context.determine_strategy();
        
        match strategy {
            RecoveryStrategy::SkipUntil(tokens) => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        if tokens.iter().any(|t| {
                            format!("{:?}", token.kind).contains(t)
                        }) {
                            break;
                        }
                    }
                    self.advance();
                }
                true
            }
            
            RecoveryStrategy::SkipUntilStatement => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        match token.kind {
                            TokenKind::Semicolon | TokenKind::RightBrace => break,
                            _ => self.advance(),
                        }
                    } else {
                        break;
                    }
                }
                true
            }
            
            RecoveryStrategy::SkipUntilBlock => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        if matches!(token.kind, TokenKind::RightBrace) {
                            break;
                        }
                    }
                    self.advance();
                }
                true
            }
            
            RecoveryStrategy::SkipUntilFunction => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        match token.kind {
                            TokenKind::RightBrace | TokenKind::Semicolon => break,
                            _ => self.advance(),
                        }
                    } else {
                        break;
                    }
                }
                true
            }
            
            RecoveryStrategy::SkipUntilClass => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        if matches!(token.kind, TokenKind::RightBrace) {
                            break;
                        }
                    }
                    self.advance();
                }
                true
            }
            
            RecoveryStrategy::SkipUntilModule => {
                while !self.is_eof() {
                    let should_break = if let Some(token) = &self.current {
                        matches!(token.kind, TokenKind::RightBrace) ||
                        matches!(token.kind, TokenKind::Keyword(ref kw) if kw == "import" || kw == "export")
                    } else {
                        false
                    };
                    
                    if should_break {
                        break;
                    }
                    self.advance();
                }
                true
            }
            
            RecoveryStrategy::InsertToken(_) => {
                // Simplified: just advance
                self.advance();
                true
            }
            
            RecoveryStrategy::ReplaceToken(_) => {
                // Simplified: just advance
                self.advance();
                true
            }
            
            RecoveryStrategy::DeleteToken => {
                self.advance();
                true
            }
            
            RecoveryStrategy::NoRecovery => false,
        }
    }

    /// Parse an arrow function expression
    fn parse_arrow_function_expression(&mut self, is_async: bool) -> ParseResult<Node> {
        let mut params = Vec::new();
        
        // Parse parameters
        if self.check(TokenKind::LeftParen) {
            self.advance(); // Consume '('
            if !self.check(TokenKind::RightParen) {
                params = self.parse_parameters()?;
            }
            self.expect(TokenKind::RightParen)?;
        } else {
            // Single parameter without parentheses
            params.push(self.parse_identifier()?);
        }
        
        self.expect(TokenKind::Arrow)?;
        
        // Parse body
        let body = if self.check(TokenKind::LeftBrace) {
            // Block body
            Box::new(self.parse_function_body()?)
        } else {
            // Expression body
            Box::new(self.parse_expression()?)
        };
        
        let span = self.create_span_from_tokens();
        Ok(Node::ArrowFunctionExpression(ArrowFunctionExpression {
            params,
            body,
            expression: !self.check(TokenKind::LeftBrace),
            r#async: is_async,
            span: Some(span),
        }))
    }

    /// Parse a destructuring pattern
    fn parse_destructuring_pattern(&mut self) -> ParseResult<Node> {
        if self.check(TokenKind::LeftBrace) {
            // Object destructuring
            self.advance(); // Consume '{'
            let mut properties = Vec::new();
            
            while !self.check(TokenKind::RightBrace) && !self.is_eof() {
                if self.check_identifier() {
                    let key = self.parse_identifier()?;
                    let value = if self.check(TokenKind::Colon) {
                        self.advance(); // Consume ':'
                        Some(Box::new(self.parse_expression()?))
                    } else {
                        None
                    };
                    
                    let span = self.create_span_from_tokens();
                    let is_shorthand = value.is_none();
                    properties.push(Node::Property(Property {
                        key: Box::new(key),
                        value: value.unwrap_or_else(|| Box::new(Node::Identifier("".to_string()))),
                        kind: "init".to_string(),
                        computed: false,
                        method: false,
                        shorthand: is_shorthand,
                        span: Some(span),
                    }));
                } else if self.check(TokenKind::Spread) {
                    self.advance(); // Consume '...'
                    let argument = Box::new(self.parse_expression()?);
                    let span = self.create_span_from_tokens();
                    properties.push(Node::SpreadElement(SpreadElement {
                        argument,
                        span: Some(span),
                    }));
                }
                
                if self.check(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            
            self.expect(TokenKind::RightBrace)?;
            
            let span = self.create_span_from_tokens();
            Ok(Node::ObjectLiteral(ObjectLiteral {
                properties,
                span: Some(span),
            }))
        } else if self.check(TokenKind::LeftBracket) {
            // Array destructuring
            self.advance(); // Consume '['
            let mut elements = Vec::new();
            
            while !self.check(TokenKind::RightBracket) && !self.is_eof() {
                if self.check(TokenKind::Comma) {
                    elements.push(None);
                    self.advance();
                } else if self.check(TokenKind::Spread) {
                    self.advance(); // Consume '...'
                    let argument = Box::new(self.parse_expression()?);
                    let span = self.create_span_from_tokens();
                    elements.push(Some(Node::SpreadElement(SpreadElement {
                        argument,
                        span: Some(span),
                    })));
                } else {
                    elements.push(Some(self.parse_expression()?));
                }
                
                if self.check(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            
            self.expect(TokenKind::RightBracket)?;
            
            let span = self.create_span_from_tokens();
            Ok(Node::ArrayLiteral(ArrayLiteral {
                elements,
                span: Some(span),
            }))
        } else {
            // Single identifier
            self.parse_identifier()
        }
    }
}