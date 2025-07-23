// Parser for JavaScript

use super::lexer::Token;
use super::ast::{Node, Program, VariableDeclaration, BinaryExpression, WhileStatement, DoWhileStatement, ForStatement, SwitchStatement, SwitchCase, TryStatement, CatchClause, ReturnStatement, BreakStatement, ContinueStatement, AssignmentExpression, LogicalExpression, ConditionalExpression, UnaryExpression, UpdateExpression};

struct ExpressionParser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> ExpressionParser<'a> {
    fn new(tokens: &'a [Token], pos: usize) -> Self {
        Self { tokens, pos }
    }

    fn parse_expression(&mut self) -> Option<Node> {
        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        
        // Assignment expressions (lowest precedence)
        if let Some(node) = self.parse_assignment_expression() {
            return Some(node);
        }
        
        // Conditional expressions
        if let Some(node) = self.parse_conditional_expression() {
            return Some(node);
        }
        
        // Logical expressions
        if let Some(node) = self.parse_logical_expression() {
            return Some(node);
        }
        
        // Binary expressions
        if let Some(node) = self.parse_binary_expression() {
            return Some(node);
        }
        
        // Unary expressions
        if let Some(node) = self.parse_unary_expression() {
            return Some(node);
        }
        
        // Primary expressions
        if let Some(node) = self.parse_primary() {
            return Some(node);
        }
        
        None
    }

    fn parse_assignment_expression(&mut self) -> Option<Node> {
        let left = self.parse_conditional_expression()?;
        
        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        
        if let Some(Token::Symbol(op)) = self.tokens.get(self.pos) {
            let assignment_ops = ["=", "+=", "-=", "*=", "/=", "%=", "**=", "<<=", ">>=", ">>>=", "&=", "^=", "|="];
            if assignment_ops.contains(&op.as_str()) {
                let operator = op.clone();
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(right) = self.parse_assignment_expression() {
                    return Some(Node::AssignmentExpression(AssignmentExpression {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    }));
                }
            }
        }
        
        Some(left)
    }

    fn parse_conditional_expression(&mut self) -> Option<Node> {
        let test = self.parse_logical_expression()?;
        
        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        
        if let Some(Token::Symbol(op)) = self.tokens.get(self.pos) {
            if op == "?" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                let consequent = self.parse_assignment_expression()?;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Symbol(op)) = self.tokens.get(self.pos) {
                    if op == ":" {
                        self.pos += 1;
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        let alternate = self.parse_assignment_expression()?;
                        return Some(Node::ConditionalExpression(ConditionalExpression {
                            test: Box::new(test),
                            consequent: Box::new(consequent),
                            alternate: Box::new(alternate),
                        }));
                    }
                }
            }
        }
        
        Some(test)
    }

    fn parse_logical_expression(&mut self) -> Option<Node> {
        let mut left = self.parse_binary_expression()?;
        
        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        
        while let Some(Token::Symbol(op)) = self.tokens.get(self.pos) {
            let logical_ops = ["||", "&&", "??"];
            if logical_ops.contains(&op.as_str()) {
                let operator = op.clone();
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(right) = self.parse_binary_expression() {
                    left = Node::LogicalExpression(LogicalExpression {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    });
                } else {
                    break;
                }
            } else {
                break;
            }
            while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        }
        
        Some(left)
    }

    fn parse_binary_expression(&mut self) -> Option<Node> {
        let mut left = self.parse_unary_expression()?;
        
        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        
        while let Some(Token::Symbol(op)) = self.tokens.get(self.pos) {
            let binary_ops = ["+", "-", "*", "/", "%", "**", "==", "!=", "===", "!==", "<", "<=", ">", ">=", "<<", ">>", ">>>", "&", "^", "|"];
            if binary_ops.contains(&op.as_str()) {
                let operator = op.clone();
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(right) = self.parse_unary_expression() {
                    left = Node::BinaryExpression(BinaryExpression {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    });
                } else {
                    break;
                }
            } else {
                break;
            }
            while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        }
        
        Some(left)
    }

    fn parse_unary_expression(&mut self) -> Option<Node> {
        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        
        // Unary operators
        if let Some(Token::Symbol(op)) = self.tokens.get(self.pos) {
            let unary_ops = ["!", "~", "+", "-", "typeof", "void", "delete"];
            if unary_ops.contains(&op.as_str()) {
                let operator = op.clone();
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(argument) = self.parse_unary_expression() {
                    return Some(Node::UnaryExpression(UnaryExpression {
                        operator,
                        argument: Box::new(argument),
                        prefix: true,
                    }));
                }
            }
        }
        
        // Update expressions (++ and --)
        if let Some(Token::Symbol(op)) = self.tokens.get(self.pos) {
            if op == "++" || op == "--" {
                let operator = op.clone();
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(argument) = self.parse_primary() {
                    return Some(Node::UpdateExpression(UpdateExpression {
                        operator,
                        argument: Box::new(argument),
                        prefix: true,
                    }));
                }
            }
        }
        
        // Postfix update expressions
        let primary = self.parse_primary()?;
        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        if let Some(Token::Symbol(op)) = self.tokens.get(self.pos) {
            if op == "++" || op == "--" {
                let operator = op.clone();
                self.pos += 1;
                return Some(Node::UpdateExpression(UpdateExpression {
                    operator,
                    argument: Box::new(primary),
                    prefix: false,
                }));
            }
        }
        
        Some(primary)
    }

    fn parse_primary(&mut self) -> Option<Node> {
        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        
        // Parentheses
        if let Some(Token::Symbol(sym)) = self.tokens.get(self.pos) {
            if sym == "(" {
                self.pos += 1;
                let expr = self.parse_expression();
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == ")" { self.pos += 1; }
                }
                return expr;
            }
        }
        
        // Function call: identifier(...)
        if let Some(Token::Identifier(name)) = self.tokens.get(self.pos) {
            if let Some(Token::Symbol(sym)) = self.tokens.get(self.pos + 1) {
                if sym == "(" {
                    return self.parse_call_expression();
                }
            }
            // Just identifier
            let name = name.clone();
            self.pos += 1;
            return Some(Node::Identifier(name));
        }
        
        // Array
        if let Some(Token::Symbol(sym)) = self.tokens.get(self.pos) {
            if sym == "[" {
                return self.parse_array_literal();
            }
        }
        
        // Object literal or block
        if let Some(Token::Symbol(sym)) = self.tokens.get(self.pos) {
            if sym == "{" {
                // If it's the start of an expression, parse as object, otherwise block
                // Heuristic: if next token is identifier/string/number and then ':', it's an object
                let lookahead = self.tokens.get(self.pos + 1);
                let lookahead2 = self.tokens.get(self.pos + 2);
                let is_object = matches!(lookahead, Some(Token::Identifier(_)) | Some(Token::String(_))) && matches!(lookahead2, Some(Token::Symbol(s)) if s == ":");
                if is_object {
                    return self.parse_object_literal();
                } else {
                    return self.parse_block_statement();
                }
            }
        }
        
        // Literals
        match self.tokens.get(self.pos) {
            Some(Token::Number(n)) => {
                let node = Node::Number(*n);
                self.pos += 1;
                Some(node)
            }
            Some(Token::Boolean(b)) => {
                let node = Node::Boolean(*b);
                self.pos += 1;
                Some(node)
            }
            Some(Token::Null) => {
                self.pos += 1;
                Some(Node::Null)
            }
            Some(Token::Undefined) => {
                self.pos += 1;
                Some(Node::Undefined)
            }
            Some(Token::String(s)) => {
                let node = Node::String(s.clone());
                self.pos += 1;
                Some(node)
            }
            Some(Token::Symbol(sym)) if sym == "(" => {
                self.pos += 1;
                let expr = self.parse_expression();
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == ")" { self.pos += 1; }
                }
                expr
            }
            _ => None,
        }
    }

    fn parse_call_expression(&mut self) -> Option<Node> {
        if let Some(Token::Identifier(name)) = self.tokens.get(self.pos) {
            let name = name.clone();
            if let Some(Token::Symbol(sym)) = self.tokens.get(self.pos + 1) {
                if sym == "(" {
                    self.pos += 2; // skip identifier and '('
                    let mut args = Vec::new();
                    loop {
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ")" { self.pos += 1; break; }
                        }
                        // If next is ')', don't expect argument
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ")" { self.pos += 1; break; }
                        }
                        if let Some(arg) = self.parse_expression() {
                            args.push(arg);
                        } else {
                            // If couldn't parse argument but found ')', end
                            if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                if s == ")" { self.pos += 1; break; }
                            }
                            break;
                        }
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == "," { self.pos += 1; continue; }
                            if s == ")" { self.pos += 1; break; }
                        } else { break; }
                    }
                    return Some(Node::CallExpression(crate::ast::CallExpression {
                        callee: Box::new(Node::Identifier(name)),
                        arguments: args,
                    }));
                }
            }
        }
        None
    }

    fn parse_array_literal(&mut self) -> Option<Node> {
        if let Some(Token::Symbol(sym)) = self.tokens.get(self.pos) {
            if sym == "[" {
                self.pos += 1;
                let mut elements = Vec::new();
                let mut expect_element = true;
                loop {
                    while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                    if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                        if s == "]" { self.pos += 1; break; }
                    }
                    if expect_element {
                        if let Some(elem) = self.parse_primary() {
                            elements.push(elem);
                            expect_element = false;
                        } else {
                            break;
                        }
                    }
                    while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                    if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                        if s == "," { self.pos += 1; expect_element = true; continue; }
                        if s == "]" { self.pos += 1; break; }
                    } else { break; }
                }
                return Some(Node::ArrayLiteral(crate::ast::ArrayLiteral { 
                    elements: elements.into_iter().map(Some).collect() 
                }));
            }
        }
        None
    }

    fn parse_object_literal(&mut self) -> Option<Node> {
        if let Some(Token::Symbol(sym)) = self.tokens.get(self.pos) {
            if sym == "{" {
                self.pos += 1;
                let mut properties = Vec::new();
                let mut expect_property = true;
                loop {
                    while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                    if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                        if s == "}" { self.pos += 1; break; }
                    }
                    if expect_property {
                        // key
                        let key = if let Some(Token::Identifier(id)) = self.tokens.get(self.pos) {
                            let k = id.clone();
                            self.pos += 1;
                            k
                        } else if let Some(Token::String(s)) = self.tokens.get(self.pos) {
                            let k = s.clone();
                            self.pos += 1;
                            k
                        } else { break; };
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ":" { self.pos += 1; }
                        }
                        let value = if let Some(val) = self.parse_primary() {
                            val
                        } else { break; };
                        properties.push(Node::Property(crate::ast::Property { 
                            key: Box::new(Node::Identifier(key)), 
                            value: Box::new(value),
                            kind: "init".to_string(),
                            computed: false,
                            method: false,
                            shorthand: false,
                        }));
                        expect_property = false;
                    }
                    while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                    if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                        if s == "," { self.pos += 1; expect_property = true; continue; }
                        if s == "}" { self.pos += 1; break; }
                    } else { break; }
                }
                return Some(Node::ObjectLiteral(crate::ast::ObjectLiteral { properties }));
            }
        }
        None
    }

    fn parse_block_statement(&mut self) -> Option<Node> {
        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
        if let Some(Token::Symbol(sym)) = self.tokens.get(self.pos) {
            if sym == "{" {
                self.pos += 1;
                let mut body = Vec::new();
                loop {
                    while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                    if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                        if s == "}" { 
                            self.pos += 1; 
                            break; 
                        }
                    }
                    if let Some(stmt) = self.parse_expression() {
                        body.push(stmt);
                    } else {
                        self.pos += 1;
                    }
                }
                return Some(Node::BlockStatement(crate::ast::BlockStatement { body }));
            }
        }
        None
    }

    fn parse_function_declaration(&mut self) -> Option<Node> {
        // function [name] (params) { body }
        let mut is_async = false;
        let mut is_generator = false;
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "async" {
                is_async = true;
                self.pos += 1;
            }
        }
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "function" {
                self.pos += 1;
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == "*" {
                        is_generator = true;
                        self.pos += 1;
                    }
                }
                let id = if let Some(Token::Identifier(name)) = self.tokens.get(self.pos) {
                    let n = name.clone();
                    self.pos += 1;
                    Some(Box::new(Node::Identifier(n)))
                } else { None };
                // Params
                let mut params = Vec::new();
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == "(" {
                        self.pos += 1;
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        while let Some(Token::Identifier(param)) = self.tokens.get(self.pos) {
                            params.push(Node::Identifier(param.clone()));
                            self.pos += 1;
                            while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                            if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                if s == "," { self.pos += 1; continue; }
                                if s == ")" { self.pos += 1; break; }
                            }
                        }
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ")" { self.pos += 1; }
                        }
                    }
                }
                // Body
                let body = self.parse_block_statement().map(Box::new)?;
                return Some(Node::FunctionDeclaration(crate::ast::FunctionDeclaration {
                    id,
                    params,
                    body,
                    generator: is_generator,
                    r#async: is_async,
                }));
            }
        }
        None
    }

    fn parse_if_statement(&mut self) -> Option<Node> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "if" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == "(" {
                        self.pos += 1;
                        let test = self.parse_expression()?;
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ")" { self.pos += 1; }
                        }
                        let consequent = self.parse_block_statement().map(Box::new)?;
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        let alternate = if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
                            if k == "else" {
                                self.pos += 1;
                                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                                Some(Box::new(self.parse_block_statement()?))
                            } else { None }
                        } else { None };
                        return Some(Node::IfStatement(crate::ast::IfStatement {
                            test: Box::new(test),
                            consequent,
                            alternate,
                        }));
                    }
                }
            }
        }
        None
    }

    fn parse_while_statement(&mut self) -> Option<Node> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "while" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == "(" {
                        self.pos += 1;
                        let test = self.parse_expression()?;
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ")" { self.pos += 1; }
                        }
                        let body = self.parse_block_statement().map(Box::new)?;
                        return Some(Node::WhileStatement(WhileStatement {
                            test: Box::new(test),
                            body,
                        }));
                    }
                }
            }
        }
        None
    }

    fn parse_do_while_statement(&mut self) -> Option<Node> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "do" {
                self.pos += 1;
                let body = self.parse_block_statement().map(Box::new)?;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
                    if k == "while" {
                        self.pos += 1;
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == "(" {
                                self.pos += 1;
                                let test = self.parse_expression()?;
                                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                    if s == ")" { self.pos += 1; }
                                }
                                return Some(Node::DoWhileStatement(DoWhileStatement {
                                    body,
                                    test: Box::new(test),
                                }));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn parse_for_statement(&mut self) -> Option<Node> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "for" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == "(" {
                        self.pos += 1;
                        
                        // Parse init
                        let init = if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ";" {
                                self.pos += 1;
                                None
                            } else {
                                let expr = self.parse_expression();
                                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                    if s == ";" { self.pos += 1; }
                                }
                                expr.map(Box::new)
                            }
                        } else if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
                            if k == "let" || k == "const" || k == "var" {
                                let kind = k.clone();
                                self.pos += 1;
                                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                                if let Some(Token::Identifier(name)) = self.tokens.get(self.pos) {
                                    self.pos += 1;
                                    while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                                    let init_expr = if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                        if s == "=" {
                                            self.pos += 1;
                                            while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                                            let expr = self.parse_expression();
                                            if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                                if s == ";" { self.pos += 1; }
                                            }
                                            expr
                                        } else { None }
                                    } else { None };
                                    let declarator = crate::ast::VariableDeclarator {
                                        id: Box::new(Node::Identifier(name.clone())),
                                        init: init_expr.map(Box::new),
                                    };
                                    let decl = VariableDeclaration {
                                        kind,
                                        declarations: vec![declarator],
                                    };
                                    Some(Box::new(Node::VariableDeclaration(decl)))
                                } else { None }
                            } else { None }
                        } else { None };
                        
                        // Parse test
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        let test = if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ";" {
                                self.pos += 1;
                                None
                            } else {
                                None
                            }
                        } else {
                            let expr = self.parse_expression();
                            if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                if s == ";" { self.pos += 1; }
                            }
                            expr.map(Box::new)
                        };
                        
                        // Parse update
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        let update = if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ")" {
                                self.pos += 1;
                                None
                            } else {
                                None
                            }
                        } else {
                            let expr = self.parse_expression();
                            if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                if s == ")" { self.pos += 1; }
                            }
                            expr.map(Box::new)
                        };
                        
                        let body = self.parse_block_statement().map(Box::new)?;
                        return Some(Node::ForStatement(ForStatement {
                            init,
                            test,
                            update,
                            body,
                        }));
                    }
                }
            }
        }
        None
    }

    fn parse_switch_statement(&mut self) -> Option<Node> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "switch" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == "(" {
                        self.pos += 1;
                        let discriminant = self.parse_expression().map(Box::new)?;
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == ")" { self.pos += 1; }
                        }
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == "{" {
                                self.pos += 1;
                                let mut cases = Vec::new();
                                loop {
                                    while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                                    if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                        if s == "}" { self.pos += 1; break; }
                                    }
                                    if let Some(case) = self.parse_switch_case() {
                                        cases.push(case);
                                    } else {
                                        break;
                                    }
                                }
                                return Some(Node::SwitchStatement(SwitchStatement {
                                    discriminant,
                                    cases,
                                }));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn parse_switch_case(&mut self) -> Option<SwitchCase> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "case" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                let test = self.parse_expression().map(Box::new);
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == ":" { self.pos += 1; }
                }
                let mut consequent = Vec::new();
                loop {
                    while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                    if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
                        if k == "case" || k == "default" || k == "}" {
                            break;
                        }
                    }
                    if let Some(stmt) = self.parse_expression() {
                        consequent.push(stmt);
                    } else {
                        break;
                    }
                }
                return Some(SwitchCase { test, consequent });
            } else if k == "default" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == ":" { self.pos += 1; }
                }
                let mut consequent = Vec::new();
                loop {
                    while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                    if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
                        if k == "case" || k == "default" || k == "}" {
                            break;
                        }
                    }
                    if let Some(stmt) = self.parse_expression() {
                        consequent.push(stmt);
                    } else {
                        break;
                    }
                }
                return Some(SwitchCase { test: None, consequent });
            }
        }
        None
    }

    fn parse_try_statement(&mut self) -> Option<Node> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "try" {
                self.pos += 1;
                let block = self.parse_block_statement().map(Box::new)?;
                
                let mut handler = None;
                let mut finalizer = None;
                
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                
                // Parse catch
                if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
                    if k == "catch" {
                        self.pos += 1;
                        while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                        if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                            if s == "(" {
                                self.pos += 1;
                                let param = if let Some(Token::Identifier(name)) = self.tokens.get(self.pos) {
                                    let n = name.clone();
                                    self.pos += 1;
                                    Node::Identifier(n)
                                } else { Node::Identifier("error".to_string()) };
                                if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                                    if s == ")" { self.pos += 1; }
                                }
                                let body = self.parse_block_statement().map(Box::new)?;
                                handler = Some(Box::new(Node::CatchClause(CatchClause {
                                    param: Box::new(param),
                                    body,
                                })));
                            }
                        }
                    }
                }
                
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                
                // Parse finally
                if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
                    if k == "finally" {
                        self.pos += 1;
                        finalizer = self.parse_block_statement().map(Box::new);
                    }
                }
                
                return Some(Node::TryStatement(TryStatement {
                    block,
                    handler,
                    finalizer,
                }));
            }
        }
        None
    }

    fn parse_return_statement(&mut self) -> Option<Node> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "return" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                let argument = if let Some(Token::Symbol(s)) = self.tokens.get(self.pos) {
                    if s == ";" {
                        None
                    } else {
                        let expr = self.parse_expression();
                        expr.map(Box::new)
                    }
                } else {
                    let expr = self.parse_expression();
                    expr.map(Box::new)
                };
                return Some(Node::ReturnStatement(ReturnStatement { argument }));
            }
        }
        None
    }

    fn parse_break_statement(&mut self) -> Option<Node> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "break" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                let label = if let Some(Token::Identifier(name)) = self.tokens.get(self.pos) {
                    let n = name.clone();
                    self.pos += 1;
                    Some(Box::new(Node::Identifier(n)))
                } else { None };
                return Some(Node::BreakStatement(BreakStatement { label }));
            }
        }
        None
    }

    fn parse_continue_statement(&mut self) -> Option<Node> {
        if let Some(Token::Keyword(k)) = self.tokens.get(self.pos) {
            if k == "continue" {
                self.pos += 1;
                while let Some(Token::Whitespace) = self.tokens.get(self.pos) { self.pos += 1; }
                let label = if let Some(Token::Identifier(name)) = self.tokens.get(self.pos) {
                    let n = name.clone();
                    self.pos += 1;
                    Some(Box::new(Node::Identifier(n)))
                } else { None };
                return Some(Node::ContinueStatement(ContinueStatement { label }));
            }
        }
        None
    }

    fn get_pos(&self) -> usize {
        self.pos
    }
}

pub fn parse(tokens: &[Token]) -> Program {
    let mut pos = 0;
    let mut body = Vec::new();

    while pos < tokens.len() {
        match &tokens[pos] {
            Token::Whitespace => { pos += 1; },
            Token::Keyword(k) if k == "let" || k == "const" || k == "var" => {
                let kind = k.clone();
                pos += 1;
                while let Some(Token::Whitespace) = tokens.get(pos) { pos += 1; }
                if let Some(Token::Identifier(name)) = tokens.get(pos) {
                    pos += 1;
                    while let Some(Token::Whitespace) = tokens.get(pos) { pos += 1; }
                    let init = if let Some(Token::Symbol(s)) = tokens.get(pos) {
                        if s == "=" {
                            pos += 1;
                            while let Some(Token::Whitespace) = tokens.get(pos) { pos += 1; }
                            let mut expr_parser = ExpressionParser::new(tokens, pos);
                            if let Some(expr) = expr_parser.parse_expression() {
                                pos = expr_parser.get_pos();
                                Some(Box::new(expr))
                            } else { None }
                        } else { None }
                    } else { None };
                    let declarator = crate::ast::VariableDeclarator {
                        id: Box::new(Node::Identifier(name.clone())),
                        init,
                    };
                    let decl = VariableDeclaration {
                        kind,
                        declarations: vec![declarator],
                    };
                    body.push(Node::VariableDeclaration(decl));
                }
            }
            Token::Keyword(k) if k == "function" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(func) = expr_parser.parse_function_declaration() {
                    pos = expr_parser.get_pos();
                    body.push(func);
                } else {
                    pos += 1;
                }
            }
            Token::Keyword(k) if k == "if" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(if_stmt) = expr_parser.parse_if_statement() {
                    pos = expr_parser.get_pos();
                    body.push(if_stmt);
                } else {
                    pos += 1;
                }
            }
            Token::Keyword(k) if k == "while" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(while_stmt) = expr_parser.parse_while_statement() {
                    pos = expr_parser.get_pos();
                    body.push(while_stmt);
                } else {
                    pos += 1;
                }
            }
            Token::Keyword(k) if k == "do" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(do_while_stmt) = expr_parser.parse_do_while_statement() {
                    pos = expr_parser.get_pos();
                    body.push(do_while_stmt);
                } else {
                    pos += 1;
                }
            }
            Token::Keyword(k) if k == "for" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(for_stmt) = expr_parser.parse_for_statement() {
                    pos = expr_parser.get_pos();
                    body.push(for_stmt);
                } else {
                    pos += 1;
                }
            }
            Token::Keyword(k) if k == "switch" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(switch_stmt) = expr_parser.parse_switch_statement() {
                    pos = expr_parser.get_pos();
                    body.push(switch_stmt);
                } else {
                    pos += 1;
                }
            }
            Token::Keyword(k) if k == "try" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(try_stmt) = expr_parser.parse_try_statement() {
                    pos = expr_parser.get_pos();
                    body.push(try_stmt);
                } else {
                    pos += 1;
                }
            }
            Token::Keyword(k) if k == "return" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(return_stmt) = expr_parser.parse_return_statement() {
                    pos = expr_parser.get_pos();
                    body.push(return_stmt);
                } else {
                    pos += 1;
                }
            }
            Token::Keyword(k) if k == "break" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(break_stmt) = expr_parser.parse_break_statement() {
                    pos = expr_parser.get_pos();
                    body.push(break_stmt);
                } else {
                    pos += 1;
                }
            }
            Token::Keyword(k) if k == "continue" => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(continue_stmt) = expr_parser.parse_continue_statement() {
                    pos = expr_parser.get_pos();
                    body.push(continue_stmt);
                } else {
                    pos += 1;
                }
            }
            Token::Eof => break,
            Token::Identifier(_) => {
                let mut expr_parser = ExpressionParser::new(tokens, pos);
                if let Some(expr) = expr_parser.parse_expression() {
                    pos = expr_parser.get_pos();
                    body.push(Node::ExpressionStatement(crate::ast::ExpressionStatement {
                        expression: Box::new(expr),
                    }));
                } else {
                    pos += 1;
                }
            }
            _ => pos += 1,
        }
    }
    Program { body, source_type: "script".to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::ast::{Node, VariableDeclaration, BinaryExpression};

    #[test]
    fn test_parse_variable_declaration() {
        let tokens = tokenize("let x = 10");
        let program = parse(&tokens);
        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("x".to_string()));
                        assert_eq!(**init.as_ref().unwrap(), Node::Number(10.0));
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_binary_expression() {
        let tokens = tokenize("let y = 1 + 2");
        let program = parse(&tokens);
        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("y".to_string()));
                        match &**init.as_ref().unwrap() {
                            Node::BinaryExpression(BinaryExpression { left, operator, right }) => {
                                assert_eq!(**left, Node::Number(1.0));
                                assert_eq!(operator, "+");
                                assert_eq!(**right, Node::Number(2.0));
                            }
                            _ => panic!("Expected binary expression"),
                        }
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_literal_boolean() {
        let tokens = tokenize("let b = true");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("b".to_string()));
                        assert_eq!(**init.as_ref().unwrap(), Node::Boolean(true));
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_literal_null() {
        let tokens = tokenize("let n = null");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("n".to_string()));
                        assert_eq!(**init.as_ref().unwrap(), Node::Null);
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_literal_undefined() {
        let tokens = tokenize("let u = undefined");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("u".to_string()));
                        assert_eq!(**init.as_ref().unwrap(), Node::Undefined);
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_literal_string() {
        let tokens = tokenize("let s = \"hello\"");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("s".to_string()));
                        assert_eq!(**init.as_ref().unwrap(), Node::String("hello".to_string()));
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_parenthesized_expression() {
        let tokens = tokenize("let z = (1 + 2)");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("z".to_string()));
                        match &**init.as_ref().unwrap() {
                            Node::BinaryExpression(BinaryExpression { left, operator, right }) => {
                                assert_eq!(**left, Node::Number(1.0));
                                assert_eq!(operator, "+");
                                assert_eq!(**right, Node::Number(2.0));
                            }
                            _ => panic!("Expected binary expression"),
                        }
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_array_literal() {
        let tokens = tokenize("let arr = [1, 2, 3]");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("arr".to_string()));
                        match &**init.as_ref().unwrap() {
                            Node::ArrayLiteral(arr) => {
                                assert_eq!(arr.elements.len(), 3);
                                assert_eq!(arr.elements[0].as_ref().unwrap(), &Node::Number(1.0));
                                assert_eq!(arr.elements[1].as_ref().unwrap(), &Node::Number(2.0));
                                assert_eq!(arr.elements[2].as_ref().unwrap(), &Node::Number(3.0));
                            }
                            _ => panic!("Expected array literal"),
                        }
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_object_literal() {
        let tokens = tokenize("let obj = {a: 1, b: 2}");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("obj".to_string()));
                        match &**init.as_ref().unwrap() {
                            Node::ObjectLiteral(obj) => {
                                assert_eq!(obj.properties.len(), 2);
                                match &obj.properties[0] {
                                    Node::Property(prop) => {
                                        assert_eq!(*prop.key, Node::Identifier("a".to_string()));
                                        assert_eq!(*prop.value, Node::Number(1.0));
                                    }
                                    _ => panic!("Expected property"),
                                }
                                match &obj.properties[1] {
                                    Node::Property(prop) => {
                                        assert_eq!(*prop.key, Node::Identifier("b".to_string()));
                                        assert_eq!(*prop.value, Node::Number(2.0));
                                    }
                                    _ => panic!("Expected property"),
                                }
                            }
                            _ => panic!("Expected object literal"),
                        }
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_call_expression() {
        let tokens = tokenize("let r = foo(1, 2)");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("r".to_string()));
                        match &**init.as_ref().unwrap() {
                            Node::CallExpression(call) => {
                                assert_eq!(*call.callee, Node::Identifier("foo".to_string()));
                                assert_eq!(call.arguments.len(), 2);
                                assert_eq!(call.arguments[0], Node::Number(1.0));
                                assert_eq!(call.arguments[1], Node::Number(2.0));
                            }
                            _ => panic!("Expected call expression"),
                        }
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_block_statement() {
        let tokens = tokenize("let b = { 1 2 }");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("b".to_string()));
                        match &**init.as_ref().unwrap() {
                            Node::BlockStatement(block) => {
                                assert_eq!(block.body.len(), 2);
                                assert_eq!(block.body[0], Node::Number(1.0));
                                assert_eq!(block.body[1], Node::Number(2.0));
                            }
                            _ => panic!("Expected block statement"),
                        }
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_while_statement() {
        let tokens = tokenize("while (x > 0) { x-- }");
        let program = parse(&tokens);
        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Node::WhileStatement(while_stmt) => {
                match &*while_stmt.test {
                    Node::BinaryExpression(bin_expr) => {
                        assert_eq!(*bin_expr.left, Node::Identifier("x".to_string()));
                        assert_eq!(bin_expr.operator, ">");
                        assert_eq!(*bin_expr.right, Node::Number(0.0));
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected while statement"),
        }
    }

    #[test]
    fn test_parse_for_statement() {
        let tokens = tokenize("for (let i = 0; i < 10; i++) { console.log(i) }");
        let program = parse(&tokens);
        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Node::ForStatement(for_stmt) => {
                assert!(for_stmt.init.is_some());
                assert!(for_stmt.test.is_some());
                assert!(for_stmt.update.is_some());
            }
            _ => panic!("Expected for statement"),
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let tokens = tokenize("if (x > 0) { return x } else { return 0 }");
        let program = parse(&tokens);
        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Node::IfStatement(if_stmt) => {
                assert!(if_stmt.alternate.is_some());
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn test_parse_assignment_expression() {
        let tokens = tokenize("let x = 5; x += 3");
        let program = parse(&tokens);
        assert_eq!(program.body.len(), 2);
        match &program.body[1] {
            Node::ExpressionStatement(expr_stmt) => {
                match &*expr_stmt.expression {
                    Node::AssignmentExpression(assign) => {
                        assert_eq!(assign.operator, "+=");
                    }
                    _ => panic!("Expected assignment expression"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_logical_expression() {
        let tokens = tokenize("let result = true && false || true");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("result".to_string()));
                        match &**init.as_ref().unwrap() {
                            Node::LogicalExpression(logical) => {
                                assert_eq!(logical.operator, "||");
                            }
                            _ => panic!("Expected logical expression"),
                        }
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_conditional_expression() {
        let tokens = tokenize("let max = a > b ? a : b");
        let program = parse(&tokens);
        match &program.body[0] {
            Node::VariableDeclaration(VariableDeclaration { declarations, .. }) => {
                match &declarations[0] {
                    crate::ast::VariableDeclarator { id, init } => {
                        assert_eq!(**id, Node::Identifier("max".to_string()));
                        match &**init.as_ref().unwrap() {
                            Node::ConditionalExpression(conditional) => {
                                assert_eq!(*conditional.consequent, Node::Identifier("a".to_string()));
                                assert_eq!(*conditional.alternate, Node::Identifier("b".to_string()));
                            }
                            _ => panic!("Expected conditional expression"),
                        }
                    }
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }
}
