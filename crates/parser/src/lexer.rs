// Lexer para JavaScript - ECMAScript completo

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f64),
    BigInt(String),
    String(String),
    TemplateString(String),
    Boolean(bool),
    Null,
    Undefined,
    Regex(String),
    Keyword(String),
    Symbol(String),
    Comment(String),
    Whitespace,
    Eof,
    // Tokens específicos para melhor parsing
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Dot,
    Semicolon,
    Comma,
    Colon,
    Question,
    Exclamation,
    Tilde,
    // Operadores de atribuição
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    StarStarAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    // Operadores de comparação
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    // Operadores lógicos
    LogicalAnd,
    LogicalOr,
    NullishCoalescing,
    // Operadores de incremento/decremento
    Increment,
    Decrement,
    // Operadores aritméticos
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    // Operadores bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    // Outros
    Arrow,
    OptionalChaining,
    Spread,
    Rest,
    PrivateField,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub start: Position,
    pub end: Position,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    while let Some(&c) = chars.peek() {
        // Atualização de linha/coluna
        if c == '\n' {
            chars.next();
            line += 1;
            column = 1;
            tokens.push(Token::Whitespace);
            continue;
        }
        if c.is_whitespace() {
            chars.next();
            column += 1;
            tokens.push(Token::Whitespace);
            continue;
        }
        
        // Comentários de linha
        if c == '/' && chars.clone().nth(1) == Some('/') {
            chars.next(); chars.next(); column += 2;
            let mut comment = String::new();
            while let Some(&ch) = chars.peek() {
                if ch == '\n' { break; }
                comment.push(ch);
                chars.next();
                column += 1;
            }
            tokens.push(Token::Comment(comment));
            continue;
        }
        
        // Comentários de bloco
        if c == '/' && chars.clone().nth(1) == Some('*') {
            chars.next(); chars.next(); column += 2;
            let mut comment = String::new();
            while let Some(&ch) = chars.peek() {
                if ch == '*' && chars.clone().nth(1) == Some('/') {
                    chars.next(); chars.next(); column += 2;
                    break;
                }
                if ch == '\n' { line += 1; column = 1; }
                comment.push(ch);
                chars.next();
                column += 1;
            }
            tokens.push(Token::Comment(comment));
            continue;
        }
        
        // Identificadores e keywords
        if c.is_ascii_alphabetic() || c == '_' || c == '$' {
            let mut ident = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                    ident.push(ch);
                    chars.next();
                    column += 1;
                } else {
                    break;
                }
            }
            match ident.as_str() {
                "true" => tokens.push(Token::Boolean(true)),
                "false" => tokens.push(Token::Boolean(false)),
                "null" => tokens.push(Token::Null),
                "undefined" => tokens.push(Token::Undefined),
                "this" => tokens.push(Token::Keyword("this".to_string())),
                "super" => tokens.push(Token::Keyword("super".to_string())),
                // Keywords ECMAScript modernas
                "let" | "const" | "var" | "function" | "if" | "else" | "return" |
                "async" | "await" | "yield" | "import" | "export" | "new" |
                "class" | "extends" | "static" | "get" | "set" | "try" | "catch" | "finally" |
                "throw" | "break" | "continue" | "switch" | "case" | "default" | "for" | "while" |
                "do" | "in" | "of" | "with" | "delete" | "instanceof" | "typeof" | "void" |
                "debugger" | "enum" | "interface" | "package" | "private" | "protected" | "public" |
                "implements" | "abstract" | "boolean" | "byte" | "char" | "double" | "final" |
                "float" | "goto" | "int" | "long" | "native" | "short" | "synchronized" |
                "throws" | "transient" | "volatile" => tokens.push(Token::Keyword(ident)),
                _ => tokens.push(Token::Identifier(ident)),
            }
            continue;
        }
        
        // BigInt literals
        if c.is_ascii_digit() {
            let mut num = String::new();
            let mut is_bigint = false;
            
            if c == '0' {
                let next = chars.clone().nth(1);
                if next == Some('x') || next == Some('X') {
                    // Hexadecimal
                    num.push('0'); chars.next(); column += 1;
                    num.push(chars.next().unwrap()); column += 1;
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_hexdigit() {
                            num.push(ch);
                            chars.next(); column += 1;
                        } else { break; }
                    }
                    if let Some(&ch) = chars.peek() {
                        if ch == 'n' {
                            chars.next(); column += 1;
                            is_bigint = true;
                        }
                    }
                    if is_bigint {
                        tokens.push(Token::BigInt(num));
                    } else if let Ok(n) = i64::from_str_radix(&num[2..], 16) {
                        tokens.push(Token::Number(n as f64));
                    }
                    continue;
                } else if next == Some('b') || next == Some('B') {
                    // Binário
                    num.push('0'); chars.next(); column += 1;
                    num.push(chars.next().unwrap()); column += 1;
                    while let Some(&ch) = chars.peek() {
                        if ch == '0' || ch == '1' {
                            num.push(ch);
                            chars.next(); column += 1;
                        } else { break; }
                    }
                    if let Some(&ch) = chars.peek() {
                        if ch == 'n' {
                            chars.next(); column += 1;
                            is_bigint = true;
                        }
                    }
                    if is_bigint {
                        tokens.push(Token::BigInt(num));
                    } else if let Ok(n) = i64::from_str_radix(&num[2..], 2) {
                        tokens.push(Token::Number(n as f64));
                    }
                    continue;
                } else if next == Some('o') || next == Some('O') {
                    // Octal
                    num.push('0'); chars.next(); column += 1;
                    num.push(chars.next().unwrap()); column += 1;
                    while let Some(&ch) = chars.peek() {
                        if ch >= '0' && ch <= '7' {
                            num.push(ch);
                            chars.next(); column += 1;
                        } else { break; }
                    }
                    if let Some(&ch) = chars.peek() {
                        if ch == 'n' {
                            chars.next(); column += 1;
                            is_bigint = true;
                        }
                    }
                    if is_bigint {
                        tokens.push(Token::BigInt(num));
                    } else if let Ok(n) = i64::from_str_radix(&num[2..], 8) {
                        tokens.push(Token::Number(n as f64));
                    }
                    continue;
                }
            }
            
            // Float ou inteiro
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' {
                    num.push(ch);
                    chars.next(); column += 1;
                } else { break; }
            }
            
            if let Some(&ch) = chars.peek() {
                if ch == 'n' {
                    chars.next(); column += 1;
                    is_bigint = true;
                }
            }
            
            if is_bigint {
                tokens.push(Token::BigInt(num));
            } else if let Ok(n) = num.parse::<f64>() {
                tokens.push(Token::Number(n));
            }
            continue;
        }
        
        // Strings
        if c == '"' || c == '\'' {
            let quote = c;
            chars.next(); column += 1;
            let mut s = String::new();
            while let Some(&ch) = chars.peek() {
                chars.next(); column += 1;
                if ch == quote { break; }
                if ch == '\\' {
                    // Escape sequences
                    if let Some(&esc) = chars.peek() {
                        match esc {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            'r' => s.push('\r'),
                            'b' => s.push('\x08'),
                            'f' => s.push('\x0c'),
                            'v' => s.push('\x0b'),
                            '0' => s.push('\0'),
                            'x' => {
                                chars.next(); column += 1;
                                let mut hex = String::new();
                                for _ in 0..2 {
                                    if let Some(&h) = chars.peek() {
                                        if h.is_ascii_hexdigit() {
                                            hex.push(h);
                                            chars.next(); column += 1;
                                        }
                                    }
                                }
                                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                    s.push(byte as char);
                                }
                            }
                            'u' => {
                                chars.next(); column += 1;
                                let mut unicode = String::new();
                                for _ in 0..4 {
                                    if let Some(&u) = chars.peek() {
                                        if u.is_ascii_hexdigit() {
                                            unicode.push(u);
                                            chars.next(); column += 1;
                                        }
                                    }
                                }
                                if let Ok(code) = u32::from_str_radix(&unicode, 16) {
                                    if let Some(ch) = char::from_u32(code) {
                                        s.push(ch);
                                    }
                                }
                            }
                            _ => {
                                s.push(esc);
                                chars.next(); column += 1;
                            }
                        }
                    }
                } else {
                    s.push(ch);
                }
            }
            tokens.push(Token::String(s));
            continue;
        }
        
        // Template strings
        if c == '`' {
            chars.next(); column += 1;
            let mut s = String::new();
            while let Some(&ch) = chars.peek() {
                chars.next(); column += 1;
                if ch == '`' { break; }
                if ch == '$' && chars.peek() == Some(&'{') {
                    chars.next(); column += 1;
                    s.push_str("${");
                } else {
                    s.push(ch);
                }
            }
            tokens.push(Token::TemplateString(s));
            continue;
        }
        
        // Operadores e símbolos específicos
        let two = chars.clone().take(2).collect::<String>();
        let three = chars.clone().take(3).collect::<String>();
        let four = chars.clone().take(4).collect::<String>();
        
        // Operadores de 4 caracteres
        if four == ">>>=" {
            for _ in 0..4 { chars.next(); column += 1; }
            tokens.push(Token::UnsignedRightShiftAssign);
            continue;
        }
        
        // Operadores de 3 caracteres
        if three == "===" || three == "!==" || three == ">>>" || three == "<<=" || three == ">>=" {
            for _ in 0..3 { chars.next(); column += 1; }
            match three.as_str() {
                "===" => tokens.push(Token::StrictEqual),
                "!==" => tokens.push(Token::StrictNotEqual),
                ">>>" => tokens.push(Token::UnsignedRightShift),
                "<<=" => tokens.push(Token::LeftShiftAssign),
                ">>=" => tokens.push(Token::RightShiftAssign),
                _ => unreachable!(),
            }
            continue;
        }
        
        // Operadores de 2 caracteres
        if two == "==" || two == "!=" || two == "<=" || two == ">=" || two == "++" || two == "--" ||
           two == "+=" || two == "-=" || two == "*=" || two == "/=" || two == "%=" || two == "**" ||
           two == "&&" || two == "||" || two == "=>" || two == "??" || two == "?." || two == "<<" ||
           two == ">>" || two == "|=" || two == "&=" || two == "^=" || two == "**" {
            for _ in 0..2 { chars.next(); column += 1; }
            match two.as_str() {
                "==" => tokens.push(Token::Equal),
                "!=" => tokens.push(Token::NotEqual),
                "<=" => tokens.push(Token::LessThanEqual),
                ">=" => tokens.push(Token::GreaterThanEqual),
                "++" => tokens.push(Token::Increment),
                "--" => tokens.push(Token::Decrement),
                "+=" => tokens.push(Token::PlusAssign),
                "-=" => tokens.push(Token::MinusAssign),
                "*=" => tokens.push(Token::StarAssign),
                "/=" => tokens.push(Token::SlashAssign),
                "%=" => tokens.push(Token::PercentAssign),
                "**" => tokens.push(Token::StarStar),
                "&&" => tokens.push(Token::LogicalAnd),
                "||" => tokens.push(Token::LogicalOr),
                "=>" => tokens.push(Token::Arrow),
                "??" => tokens.push(Token::NullishCoalescing),
                "?." => tokens.push(Token::OptionalChaining),
                "<<" => tokens.push(Token::LeftShift),
                ">>" => tokens.push(Token::RightShift),
                "|=" => tokens.push(Token::BitwiseOrAssign),
                "&=" => tokens.push(Token::BitwiseAndAssign),
                "^=" => tokens.push(Token::BitwiseXorAssign),
                _ => unreachable!(),
            }
            continue;
        }
        
        // Operadores de 1 caractere
        match c {
            '(' => { chars.next(); column += 1; tokens.push(Token::LeftParen); }
            ')' => { chars.next(); column += 1; tokens.push(Token::RightParen); }
            '{' => { chars.next(); column += 1; tokens.push(Token::LeftBrace); }
            '}' => { chars.next(); column += 1; tokens.push(Token::RightBrace); }
            '[' => { chars.next(); column += 1; tokens.push(Token::LeftBracket); }
            ']' => { chars.next(); column += 1; tokens.push(Token::RightBracket); }
            '.' => { chars.next(); column += 1; tokens.push(Token::Dot); }
            ';' => { chars.next(); column += 1; tokens.push(Token::Semicolon); }
            ',' => { chars.next(); column += 1; tokens.push(Token::Comma); }
            ':' => { chars.next(); column += 1; tokens.push(Token::Colon); }
            '?' => { chars.next(); column += 1; tokens.push(Token::Question); }
            '!' => { chars.next(); column += 1; tokens.push(Token::Exclamation); }
            '~' => { chars.next(); column += 1; tokens.push(Token::Tilde); }
            '=' => { chars.next(); column += 1; tokens.push(Token::Assign); }
            '<' => { chars.next(); column += 1; tokens.push(Token::LessThan); }
            '>' => { chars.next(); column += 1; tokens.push(Token::GreaterThan); }
            '+' => { chars.next(); column += 1; tokens.push(Token::Plus); }
            '-' => { chars.next(); column += 1; tokens.push(Token::Minus); }
            '*' => { chars.next(); column += 1; tokens.push(Token::Star); }
            '/' => { chars.next(); column += 1; tokens.push(Token::Slash); }
            '%' => { chars.next(); column += 1; tokens.push(Token::Percent); }
            '&' => { chars.next(); column += 1; tokens.push(Token::BitwiseAnd); }
            '|' => { chars.next(); column += 1; tokens.push(Token::BitwiseOr); }
            '^' => { chars.next(); column += 1; tokens.push(Token::BitwiseXor); }
            '#' => { chars.next(); column += 1; tokens.push(Token::PrivateField); }
            _ => { chars.next(); column += 1; }
        }
    }
    tokens.push(Token::Eof);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        let tokens = tokenize("foo");
        assert_eq!(tokens[0], Token::Identifier("foo".to_string()));
    }

    #[test]
    fn test_number() {
        let tokens = tokenize("42");
        assert_eq!(tokens[0], Token::Number(42.0));
    }

    #[test]
    fn test_string() {
        let tokens = tokenize("\"bar\"");
        assert_eq!(tokens[0], Token::String("bar".to_string()));
    }

    #[test]
    fn test_keyword() {
        let tokens = tokenize("let");
        assert_eq!(tokens[0], Token::Keyword("let".to_string()));
    }

    #[test]
    fn test_symbol() {
        let tokens = tokenize("=");
        assert_eq!(tokens[0], Token::Symbol("=".to_string()));
    }

    #[test]
    fn test_boolean_true() {
        let tokens = tokenize("true");
        assert_eq!(tokens[0], Token::Boolean(true));
    }

    #[test]
    fn test_boolean_false() {
        let tokens = tokenize("false");
        assert_eq!(tokens[0], Token::Boolean(false));
    }

    #[test]
    fn test_null() {
        let tokens = tokenize("null");
        assert_eq!(tokens[0], Token::Null);
    }

    #[test]
    fn test_undefined() {
        let tokens = tokenize("undefined");
        assert_eq!(tokens[0], Token::Undefined);
    }

    #[test]
    fn test_comment_line() {
        let tokens = tokenize("// hello\nfoo");
        assert_eq!(tokens[0], Token::Comment(" hello".to_string()));
        assert_eq!(tokens[2], Token::Identifier("foo".to_string()));
    }

    #[test]
    fn test_comment_block() {
        let tokens = tokenize("/* block */foo");
        assert_eq!(tokens[0], Token::Comment(" block ".to_string()));
        assert_eq!(tokens[1], Token::Identifier("foo".to_string()));
    }

    #[test]
    fn test_hex_number() {
        let tokens = tokenize("0x2A");
        assert_eq!(tokens[0], Token::Number(42.0));
    }

    #[test]
    fn test_bin_number() {
        let tokens = tokenize("0b1010");
        assert_eq!(tokens[0], Token::Number(10.0));
    }

    #[test]
    fn test_oct_number() {
        let tokens = tokenize("0o77");
        assert_eq!(tokens[0], Token::Number(63.0));
    }

    #[test]
    fn test_float_number() {
        let tokens = tokenize("3.14");
        assert_eq!(tokens[0], Token::Number(3.14));
    }

    #[test]
    fn test_template_string() {
        let tokens = tokenize("`hello world`");
        assert_eq!(tokens[0], Token::TemplateString("hello world".to_string()));
    }

    #[test]
    fn test_composite_operator() {
        let tokens = tokenize("===");
        assert_eq!(tokens[0], Token::Symbol("===".to_string()));
    }

    #[test]
    fn test_keyword_async() {
        let tokens = tokenize("async");
        assert_eq!(tokens[0], Token::Keyword("async".to_string()));
    }
} 