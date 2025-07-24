//! Bytecode structure for the V8-Rust VM

use crate::instructions::Instruction;

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    // Futuramente: pool de constantes, metadados, etc.
}

impl Bytecode {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Bytecode { instructions }
    }
} 