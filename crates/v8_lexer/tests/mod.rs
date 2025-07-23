//! Test suite for v8_lexer crate
//! 
//! This module contains comprehensive tests for the lexer following
//! the patterns established by Boa Engine and Rust best practices.

// Test modules
mod common;
mod lexer_tests;
mod error_tests;

// TODO: Implement these test modules
// mod token_tests;
// mod integration_tests;
// mod benchmark_tests;

// Re-export test utilities
pub use common::*;
pub use lexer_tests::*;
pub use error_tests::*;
// pub use token_tests::*;
// pub use integration_tests::*;
// pub use benchmark_tests::*; 