//! Virtual Machine for V8-Rust JavaScript engine
//! 
//! This crate provides the bytecode execution engine and
//! instruction set for the JavaScript engine.

pub mod bytecode;
pub mod executor;
pub mod frame;
pub mod instructions;
pub mod registers;
pub mod stack;
pub mod value;
pub mod heap;

pub use bytecode::Bytecode;
pub use executor::Executor;
pub use frame::Frame;
pub use instructions::Instruction;
pub use registers::Registers;
pub use stack::Stack; 