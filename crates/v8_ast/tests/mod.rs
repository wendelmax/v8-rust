//! Test suite for v8_ast crate

// Test modules
mod common;
mod node_tests;
mod visitor_tests;
mod serialization_tests;

// Re-export test utilities
pub use common::*;
pub use node_tests::*;
pub use visitor_tests::*;
pub use serialization_tests::*; 