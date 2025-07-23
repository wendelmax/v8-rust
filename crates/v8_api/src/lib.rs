//! Main engine for V8-Rust JavaScript engine
//! 
//! This crate provides the main engine interface that coordinates
//! all components of the JavaScript engine.

pub mod engine;
pub mod compiler;
pub mod interpreter;

pub use engine::Engine;
pub use compiler::Compiler;
pub use interpreter::Interpreter; 