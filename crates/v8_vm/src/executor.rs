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
    pub globals: Vec<i64>, // Variáveis globais
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            stack: Stack::new(),
            frame: Frame::new(),
            registers: Registers::new(),
            globals: vec![0i64; 32], // 32 variáveis globais
        }
    }

    pub fn execute(&mut self, bytecode: &Bytecode, constants: &[i64]) {
        let mut ip = 0;
        let mut locals = vec![0i64; 16]; // Exemplo: 16 variáveis locais
        let mut call_stack = Vec::new(); // Stack de chamadas para Return
        
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
                Instruction::Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a == b { 1 } else { 0 });
                }
                Instruction::Ne => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a != b { 1 } else { 0 });
                }
                Instruction::Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a < b { 1 } else { 0 });
                }
                Instruction::Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a > b { 1 } else { 0 });
                }
                Instruction::Le => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a <= b { 1 } else { 0 });
                }
                Instruction::Ge => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(if a >= b { 1 } else { 0 });
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
                Instruction::LoadGlobal(idx) => {
                    let value = self.globals.get(*idx).copied().unwrap_or(0);
                    self.stack.push(value);
                }
                Instruction::StoreGlobal(idx) => {
                    let value = self.stack.pop().unwrap();
                    if let Some(slot) = self.globals.get_mut(*idx) {
                        *slot = value;
                    }
                }
                Instruction::Call(argc) => {
                    // Salvar o endereço de retorno
                    call_stack.push(ip + 1);
                    
                    // Criar novo frame
                    let mut new_frame = Frame::new();
                    new_frame.return_address = ip + 1;
                    new_frame.arg_count = *argc;
                    
                    // Empilhar o frame atual e usar o novo
                    self.stack.push_frame(self.frame.clone());
                    self.frame = new_frame;
                    
                    // Jump para a função (próxima instrução será o endereço da função)
                    ip += 1;
                    continue;
                }
                Instruction::Return => {
                    // Recuperar valor de retorno (se houver)
                    let return_value = self.stack.pop();
                    
                    // Restaurar frame anterior
                    if let Some(prev_frame) = self.stack.pop_frame() {
                        self.frame = prev_frame;
                    }
                    
                    // Restaurar endereço de retorno
                    if let Some(return_ip) = call_stack.pop() {
                        ip = return_ip;
                        continue;
                    } else {
                        // Se não há call stack, terminar execução
                        break;
                    }
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