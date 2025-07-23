//! Test suite for v8_parser crate
//! 
//! This module contains comprehensive tests for the parser following
//! the patterns established by Boa Engine and Rust best practices.

mod parser_tests;
mod expression_tests;
mod statement_tests;
mod declaration_tests;
mod error_tests;
mod recovery_tests;
mod performance_tests;

// Re-export test modules for external use
pub use parser_tests::*;
pub use expression_tests::*;
pub use statement_tests::*;
pub use declaration_tests::*;
pub use error_tests::*;
pub use recovery_tests::*;
pub use performance_tests::*; 