//! Lexer implementation for V8-Rust JavaScript engine

use crate::{Token, TokenKind, Position, LexerError};

/// Lexer for JavaScript/ECMAScript source code
#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer for the given source code
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// Tokenize the entire source code
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        
        while self.pos < self.source.len() {
            let start_line = self.line;
            let start_col = self.column;
            
            let token = self.next_token()?;
            
            // If we got an EOF token, we're done
            if matches!(token.kind, TokenKind::Eof) {
                tokens.push(token);
                break;
            }
            
            tokens.push(token);
            
            // Update position after token
            self.update_position(start_line, start_col);
        }
        
        // Add EOF token if we don't have one already
        if tokens.is_empty() || !matches!(tokens.last().unwrap().kind, TokenKind::Eof) {
            tokens.push(Token::with_positions(
                TokenKind::Eof,
                self.line,
                self.column,
                self.line,
                self.column,
            ));
        }
        
        Ok(tokens)
    }
    
    /// Get the next token from the source
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        
        if self.pos >= self.source.len() {
            return Ok(Token::with_positions(
                TokenKind::Eof,
                self.line,
                self.column,
                self.line,
                self.column,
            ));
        }
        
        let start_line = self.line;
        let start_col = self.column;
        let c = self.source[self.pos];
        
        let token_kind = if c.is_ascii_alphabetic() || c == '_' || c == '$' || !c.is_ascii() {
            // Identifiers and keywords (including Unicode)
            self.read_identifier_or_keyword()?
        } else if c.is_ascii_digit() {
            // Numbers
            self.read_number()?
        } else if c == '"' || c == '\'' {
            // Strings
            self.read_string()?
        } else if c == '`' {
            // Template strings
            self.read_template_string()?
        } else if c == '/' {
            // Comments
            if self.peek_char(1) == Some('/') {
                self.read_line_comment()?
            } else if self.peek_char(1) == Some('*') {
                self.read_block_comment()?
            } else {
                self.read_operator()?
            }
        } else {
            // Operators and symbols
            self.read_operator()?
        };
        
        let end_line = self.line;
        let end_col = self.column;
        
        Ok(Token::with_positions(token_kind, start_line, start_col, end_line, end_col))
    }
    
    /// Read an identifier or keyword
    fn read_identifier_or_keyword(&mut self) -> Result<TokenKind, LexerError> {
        let mut identifier = String::new();
        
        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            // Support Unicode identifiers (ECMAScript 2015+)
            if c.is_alphanumeric() || c == '_' || c == '$' || c.is_alphabetic() || !c.is_ascii() {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        // Check if it's a keyword
        match identifier.as_str() {
            "true" => Ok(TokenKind::Boolean(true)),
            "false" => Ok(TokenKind::Boolean(false)),
            "null" => Ok(TokenKind::Null),
            "undefined" => Ok(TokenKind::Undefined),
            "this" => Ok(TokenKind::Keyword("this".to_string())),
            "super" => Ok(TokenKind::Keyword("super".to_string())),
            // ECMAScript keywords
            "let" | "const" | "var" | "function" | "if" | "else" | "return" |
            "async" | "await" | "yield" | "import" | "export" | "new" |
            "class" | "extends" | "static" | "get" | "set" | "try" | "catch" | "finally" |
            "throw" | "break" | "continue" | "switch" | "case" | "default" | "for" | "while" |
            "do" | "in" | "of" | "with" | "delete" | "instanceof" | "typeof" | "void" |
            "debugger" | "enum" | "interface" | "package" | "private" | "protected" | "public" |
            "implements" | "abstract" | "boolean" | "byte" | "char" | "double" | "final" |
            "float" | "goto" | "int" | "long" | "native" | "short" | "synchronized" |
            "throws" | "transient" | "volatile" => Ok(TokenKind::Keyword(identifier)),
            _ => Ok(TokenKind::Identifier(identifier)),
        }
    }
    
    /// Read a number literal
    fn read_number(&mut self) -> Result<TokenKind, LexerError> {
        let mut number = String::new();
        let mut is_hex = false;
        let mut is_binary = false;
        let mut is_octal = false;
        
        // Check for hex, binary, or octal
        if self.source[self.pos] == '0' && self.pos + 1 < self.source.len() {
            match self.source[self.pos + 1] {
                'x' | 'X' => {
                    is_hex = true;
                    number.push('0');
                    number.push(self.source[self.pos + 1]);
                    self.advance();
                    self.advance();
                }
                'b' | 'B' => {
                    is_binary = true;
                    number.push('0');
                    number.push(self.source[self.pos + 1]);
                    self.advance();
                    self.advance();
                }
                'o' | 'O' => {
                    is_octal = true;
                    number.push('0');
                    number.push(self.source[self.pos + 1]);
                    self.advance();
                    self.advance();
                }
                _ => {}
            }
        }
        
        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            
            if is_hex {
                if c.is_ascii_hexdigit() {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else if is_binary {
                if c == '0' || c == '1' {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else if is_octal {
                if c >= '0' && c <= '7' {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else {
                if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        // Check for BigInt suffix
        if self.pos < self.source.len() && self.source[self.pos] == 'n' {
            number.push('n');
            self.advance();
            return Ok(TokenKind::BigInt(number));
        }
        
        // Parse as number
        if is_hex {
            // Parse hex number
            match u64::from_str_radix(&number[2..], 16) {
                Ok(n) => Ok(TokenKind::Number(n as f64)),
                Err(_) => Err(LexerError::InvalidNumber(number)),
            }
        } else if is_binary {
            // Parse binary number
            match u64::from_str_radix(&number[2..], 2) {
                Ok(n) => Ok(TokenKind::Number(n as f64)),
                Err(_) => Err(LexerError::InvalidNumber(number)),
            }
        } else if is_octal {
            // Parse octal number
            match u64::from_str_radix(&number[2..], 8) {
                Ok(n) => Ok(TokenKind::Number(n as f64)),
                Err(_) => Err(LexerError::InvalidNumber(number)),
            }
        } else {
            // Parse decimal number
            match number.parse::<f64>() {
                Ok(n) => Ok(TokenKind::Number(n)),
                Err(_) => Err(LexerError::InvalidNumber(number)),
            }
        }
    }
    
    /// Read a string literal
    fn read_string(&mut self) -> Result<TokenKind, LexerError> {
        let quote = self.source[self.pos];
        self.advance(); // Skip opening quote
        
        let mut string = String::new();
        
        let mut found_closing_quote = false;
        
        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            
            if c == quote {
                self.advance(); // Skip closing quote
                found_closing_quote = true;
                break;
            } else if c == '\\' {
                self.advance(); // Skip backslash
                if self.pos < self.source.len() {
                    let escaped = self.source[self.pos];
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        _ => string.push(escaped),
                    }
                    self.advance();
                }
            } else {
                string.push(c);
                self.advance();
            }
        }
        
        // Check if we reached the end without finding a closing quote
        if !found_closing_quote {
            return Err(LexerError::UnterminatedString);
        }
        
        Ok(TokenKind::String(string))
    }
    
    /// Read a template string literal
    fn read_template_string(&mut self) -> Result<TokenKind, LexerError> {
        self.advance(); // Skip backtick
        
        let mut template = String::new();
        
        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            
            if c == '`' {
                self.advance(); // Skip closing backtick
                break;
            } else if c == '$' && self.peek_char(1) == Some('{') {
                // Template expression
                template.push_str("${");
                self.advance();
                self.advance();
                // TODO: Parse expression inside ${}
            } else if c == '\\' {
                self.advance(); // Skip backslash
                if self.pos < self.source.len() {
                    let escaped = self.source[self.pos];
                    match escaped {
                        'n' => template.push('\n'),
                        't' => template.push('\t'),
                        'r' => template.push('\r'),
                        '\\' => template.push('\\'),
                        '`' => template.push('`'),
                        '$' => template.push('$'),
                        _ => template.push(escaped),
                    }
                    self.advance();
                }
            } else {
                template.push(c);
                self.advance();
            }
        }
        
        Ok(TokenKind::TemplateString(template))
    }
    
    /// Read a line comment
    fn read_line_comment(&mut self) -> Result<TokenKind, LexerError> {
        self.advance(); // Skip first '/'
        self.advance(); // Skip second '/'
        
        let mut comment = String::new();
        
        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }
        
        Ok(TokenKind::Comment(comment))
    }
    
    /// Read a block comment
    fn read_block_comment(&mut self) -> Result<TokenKind, LexerError> {
        self.advance(); // Skip '/'
        self.advance(); // Skip '*'
        
        let mut comment = String::new();
        
        let mut found_closing_comment = false;
        
        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            
            if c == '*' && self.peek_char(1) == Some('/') {
                self.advance(); // Skip '*'
                self.advance(); // Skip '/'
                found_closing_comment = true;
                break;
            }
            
            comment.push(c);
            self.advance();
        }
        
        // Check if we reached the end without finding a closing comment
        if !found_closing_comment {
            return Err(LexerError::UnterminatedComment);
        }
        
        Ok(TokenKind::Comment(comment))
    }
    
    /// Read an operator or symbol
    fn read_operator(&mut self) -> Result<TokenKind, LexerError> {
        let c = self.source[self.pos];
        
        // Check for three-character operators first
        if self.pos + 2 < self.source.len() {
            let next_c = self.source[self.pos + 1];
            let next_next_c = self.source[self.pos + 2];
            let three_char_op = format!("{}{}{}", c, next_c, next_next_c);
            
            match three_char_op.as_str() {
                "===" => { self.advance(); self.advance(); self.advance(); return Ok(TokenKind::StrictEqual); }
                "!==" => { self.advance(); self.advance(); self.advance(); return Ok(TokenKind::StrictNotEqual); }
                "**=" => { self.advance(); self.advance(); self.advance(); return Ok(TokenKind::StarStarAssign); }
                "<<=" => { self.advance(); self.advance(); self.advance(); return Ok(TokenKind::LeftShiftAssign); }
                ">>=" => { self.advance(); self.advance(); self.advance(); return Ok(TokenKind::RightShiftAssign); }
                ">>>" => { self.advance(); self.advance(); self.advance(); return Ok(TokenKind::UnsignedRightShift); }
                _ => {}
            }
        }
        
        // Check for two-character operators
        if self.pos + 1 < self.source.len() {
            let next_c = self.source[self.pos + 1];
            let two_char_op = format!("{}{}", c, next_c);
            
            match two_char_op.as_str() {
                "==" => { self.advance(); self.advance(); return Ok(TokenKind::Equal); }
                "!=" => { self.advance(); self.advance(); return Ok(TokenKind::NotEqual); }
                "<=" => { self.advance(); self.advance(); return Ok(TokenKind::LessThanEqual); }
                ">=" => { self.advance(); self.advance(); return Ok(TokenKind::GreaterThanEqual); }
                "++" => { self.advance(); self.advance(); return Ok(TokenKind::Increment); }
                "--" => { self.advance(); self.advance(); return Ok(TokenKind::Decrement); }
                "&&" => { self.advance(); self.advance(); return Ok(TokenKind::LogicalAnd); }
                "||" => { self.advance(); self.advance(); return Ok(TokenKind::LogicalOr); }
                "=>" => { self.advance(); self.advance(); return Ok(TokenKind::Arrow); }
                "??" => { self.advance(); self.advance(); return Ok(TokenKind::NullishCoalescing); }
                _ => {}
            }
        }
        

        
        // Single character operators
        match c {
            '(' => { self.advance(); Ok(TokenKind::LeftParen) }
            ')' => { self.advance(); Ok(TokenKind::RightParen) }
            '{' => { self.advance(); Ok(TokenKind::LeftBrace) }
            '}' => { self.advance(); Ok(TokenKind::RightBrace) }
            '[' => { self.advance(); Ok(TokenKind::LeftBracket) }
            ']' => { self.advance(); Ok(TokenKind::RightBracket) }
            '.' => { self.advance(); Ok(TokenKind::Dot) }
            ';' => { self.advance(); Ok(TokenKind::Semicolon) }
            ',' => { self.advance(); Ok(TokenKind::Comma) }
            ':' => { self.advance(); Ok(TokenKind::Colon) }
            '?' => { self.advance(); Ok(TokenKind::Question) }
            '!' => { self.advance(); Ok(TokenKind::Exclamation) }
            '~' => { self.advance(); Ok(TokenKind::Tilde) }
            '=' => { self.advance(); Ok(TokenKind::Assign) }
            '+' => { self.advance(); Ok(TokenKind::Plus) }
            '-' => { self.advance(); Ok(TokenKind::Minus) }
            '*' => { self.advance(); Ok(TokenKind::Star) }
            '/' => { self.advance(); Ok(TokenKind::Slash) }
            '%' => { self.advance(); Ok(TokenKind::Percent) }
            '<' => { self.advance(); Ok(TokenKind::LessThan) }
            '>' => { self.advance(); Ok(TokenKind::GreaterThan) }
            '&' => { self.advance(); Ok(TokenKind::BitwiseAnd) }
            '|' => { self.advance(); Ok(TokenKind::BitwiseOr) }
            '^' => { self.advance(); Ok(TokenKind::BitwiseXor) }
            _ => Err(LexerError::UnexpectedCharacter(c)),
        }
    }
    
    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    }
    
    /// Advance to the next character
    fn advance(&mut self) {
        if self.pos < self.source.len() {
            self.pos += 1;
            self.column += 1;
        }
    }
    
    /// Peek at a character at the given offset
    fn peek_char(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.source.len() {
            Some(self.source[self.pos + offset])
        } else {
            None
        }
    }
    
    /// Update position after token
    fn update_position(&mut self, start_line: usize, start_col: usize) {
        // Position is already updated during token reading
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::new("hello");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Identifier("hello".to_string()));
    }
    
    #[test]
    fn test_number() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number(42.0));
    }
    
    #[test]
    fn test_string() {
        let mut lexer = Lexer::new("\"hello\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::String("hello".to_string()));
    }
    
    #[test]
    fn test_keyword() {
        let mut lexer = Lexer::new("let");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Keyword("let".to_string()));
    }
    
    #[test]
    fn test_boolean() {
        let mut lexer = Lexer::new("true");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Boolean(true));
    }
    
    #[test]
    fn test_null() {
        let mut lexer = Lexer::new("null");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Null);
    }
    
    #[test]
    fn test_comment_line() {
        let mut lexer = Lexer::new("// comment");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Comment(" comment".to_string()));
    }
    
    #[test]
    fn test_comment_block() {
        let mut lexer = Lexer::new("/* comment */");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Comment(" comment ".to_string()));
    }
    
    #[test]
    fn test_operator() {
        let mut lexer = Lexer::new("+");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Plus);
    }
    
    #[test]
    fn test_composite_operator() {
        let mut lexer = Lexer::new("==");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Equal);
    }
} 