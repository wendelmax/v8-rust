//! Executor for the V8-Rust VM

use crate::bytecode::Bytecode;
use crate::stack::Stack;
use crate::frame::Frame;
use crate::registers::Registers;
use crate::instructions::Instruction;

pub struct Executor {
    pub stack: Stack,
    pub frame: Frame,
    pub registers: Registers,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            stack: Stack::new(),
            frame: Frame::new(),
            registers: Registers::new(),
        }
    }

    pub fn execute(&mut self, bytecode: &Bytecode, constants: &[i64]) {
        let mut ip = 0;
        let mut locals = vec![0i64; 16]; // Exemplo: 16 variáveis locais
        while ip < bytecode.instructions.len() {
            match &bytecode.instructions[ip] {
                Instruction::PushConst(idx) => {
                    let value = constants.get(*idx).copied().unwrap_or(0);
                    self.stack.push(value);
                }
                Instruction::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                Instruction::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                Instruction::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
                Instruction::Jump(target) => {
                    ip = *target;
                    continue;
                }
                Instruction::JumpIfTrue(target) => {
                    let cond = self.stack.pop().unwrap();
                    if cond != 0 {
                        ip = *target;
                        continue;
                    }
                }
                Instruction::JumpIfFalse(target) => {
                    let cond = self.stack.pop().unwrap();
                    if cond == 0 {
                        ip = *target;
                        continue;
                    }
                }
                Instruction::LoadLocal(idx) => {
                    let value = locals.get(*idx).copied().unwrap_or(0);
                    self.stack.push(value);
                }
                Instruction::StoreLocal(idx) => {
                    let value = self.stack.pop().unwrap();
                    if let Some(slot) = locals.get_mut(*idx) {
                        *slot = value;
                    }
                }
                Instruction::Call(_argc) => {
                    // Placeholder: chamada de função
                }
                Instruction::Return => {
                    // Placeholder: retorno de função
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
                Instruction::Dup => {
                    if let Some(top) = self.stack.values.last().copied() {
                        self.stack.push(top);
                    }
                }
                _ => todo!("Instrução não implementada ainda"),
            }
            ip += 1;
        }
    }
} 