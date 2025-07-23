//! Abstract Syntax Tree for V8-Rust JavaScript engine
//! 
//! This crate provides a complete AST implementation for JavaScript/ECMAScript with:
//! - All ECMAScript node types
//! - Serialization support
//! - Visitor pattern support
//! - Source location tracking

pub mod node;
pub mod visitor;

pub use node::*;
pub use visitor::*;

/// Re-export commonly used types
pub use serde::{Deserialize, Serialize}; 