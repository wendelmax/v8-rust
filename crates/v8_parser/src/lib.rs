//! V8 Parser - A modular JavaScript parser inspired by V8 Engine
//! 
//! This crate provides a robust JavaScript parser that follows
//! the patterns established by Boa Engine and Rust best practices.

pub mod parser;
pub mod error;
pub mod recovery;

pub use parser::Parser;
pub use error::{ParseError, ParseResult};
pub use recovery::{RecoveryStrategy, ParsingContext, RecoveryContext};

/// Parse JavaScript source code into an AST
pub fn parse(source: &str) -> ParseResult<v8_ast::Node> {
    let mut parser = Parser::new(source);
    parser.parse()
}

/// Parse JavaScript source code with error recovery
pub fn parse_with_recovery(source: &str) -> (Option<v8_ast::Node>, Vec<ParseError>) {
    let mut parser = Parser::new(source);
    parser.parse_with_recovery()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parse() {
        let result = parse("");
        assert!(result.is_ok());
    }
} 