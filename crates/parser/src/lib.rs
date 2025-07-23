//! Parser de JavaScript para o projeto v8-rust

pub mod lexer;
pub mod ast;
pub mod parser;

// Exemplo de uso (futuro):
// let tokens = lexer::tokenize(source_code);
// let ast = parser::parse(tokens);
