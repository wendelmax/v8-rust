//! Token definitions for the V8-Rust lexer

use serde::{Deserialize, Serialize};

/// Represents a position in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// Represents a span of source code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    
    pub fn from_positions(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start: Position::new(start_line, start_col),
            end: Position::new(end_line, end_col),
        }
    }
}

/// Token kinds supported by the lexer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    // Literals
    Identifier(String),
    Number(f64),
    BigInt(String),
    String(String),
    TemplateString(String),
    Boolean(bool),
    Null,
    Undefined,
    Regex(String),
    
    // Keywords
    Keyword(String),
    
    // Symbols and operators
    Symbol(String),
    
    // Comments and whitespace
    Comment(String),
    Whitespace,
    Eof,
    
    // Specific tokens for better parsing
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
    
    // Assignment operators
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
    
    // Comparison operators
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    
    // Logical operators
    LogicalAnd,
    LogicalOr,
    NullishCoalescing,
    
    // Increment/Decrement operators
    Increment,
    Decrement,
    
    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    
    // Bitwise operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    
    // Other operators
    Arrow,
    OptionalChaining,
    Spread,
    Rest,
    PrivateField,
}

/// A token with position information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
    
    pub fn with_positions(kind: TokenKind, start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            kind,
            span: Span::from_positions(start_line, start_col, end_line, end_col),
        }
    }
    
    /// Get the start position of the token
    pub fn start(&self) -> Position {
        self.span.start
    }
    
    /// Get the end position of the token
    pub fn end(&self) -> Position {
        self.span.end
    }
    
    /// Check if the token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self.kind, TokenKind::Keyword(_))
    }
    
    /// Check if the token is an identifier
    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, TokenKind::Identifier(_))
    }
    
    /// Check if the token is a literal
    pub fn is_literal(&self) -> bool {
        matches!(self.kind, 
            TokenKind::Number(_) | 
            TokenKind::String(_) | 
            TokenKind::Boolean(_) | 
            TokenKind::Null | 
            TokenKind::Undefined
        )
    }
    
    /// Check if the token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(self.kind,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash |
            TokenKind::Percent | TokenKind::StarStar | TokenKind::Equal | TokenKind::NotEqual |
            TokenKind::StrictEqual | TokenKind::StrictNotEqual | TokenKind::LessThan |
            TokenKind::LessThanEqual | TokenKind::GreaterThan | TokenKind::GreaterThanEqual |
            TokenKind::LeftShift | TokenKind::RightShift | TokenKind::UnsignedRightShift |
            TokenKind::BitwiseAnd | TokenKind::BitwiseOr | TokenKind::BitwiseXor |
            TokenKind::LogicalAnd | TokenKind::LogicalOr | TokenKind::NullishCoalescing |
            TokenKind::Increment | TokenKind::Decrement
        )
    }
} 