//! Semantic Analysis for v8-rust
//! 
//! This module provides semantic analysis capabilities for JavaScript code,
//! including type checking, scope analysis, and error detection.

pub mod analyzer;
pub mod scope;
pub mod types;
pub mod errors;

pub use analyzer::SemanticAnalyzer;
pub use errors::SemanticError;
pub use scope::Scope;
pub use types::Type;

/// Result type for semantic analysis operations
pub type SemanticResult<T> = Result<T, SemanticError>;

/// Main entry point for semantic analysis
pub fn analyze(ast: &v8_ast::Node) -> SemanticResult<()> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(ast)
} 