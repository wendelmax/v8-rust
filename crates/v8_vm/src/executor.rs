//! Executor for the V8-Rust VM

use crate::bytecode::Bytecode;
use crate::stack::Stack;
use crate::frame::Frame;
use crate::registers::Registers;
use crate::instructions::Instruction;
use crate::value::Value;
use crate::heap::Heap;

pub struct Executor {
    pub stack: Stack,
    pub frame: Frame,
    pub registers: Registers,
    pub heap: Heap,
    pub globals: Vec<Value>, // Variáveis globais
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            stack: Stack::new(),
            frame: Frame::new(),
            registers: Registers::new(),
            heap: Heap::new(),
            globals: vec![Value::Undefined; 32], // 32 variáveis globais
        }
    }

    pub fn execute(&mut self, bytecode: &Bytecode, constants: &[Value]) {
        let mut ip = 0;
        let mut locals = vec![Value::Undefined; 16]; // 16 variáveis locais
        let mut call_stack = Vec::new(); // Stack de chamadas para Return
        
        while ip < bytecode.instructions.len() {
            match &bytecode.instructions[ip] {
                Instruction::PushConst(idx) => {
                    let value = constants.get(*idx).cloned().unwrap_or(Value::Undefined);
                    self.stack.push(value);
                }
                Instruction::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    // Implementar adição para diferentes tipos
                    match (a.clone(), b.clone()) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a + b));
                        }
                        _ => {
                            // Para outros tipos, converter para string e concatenar
                            let a_str = format!("{:?}", a);
                            let b_str = format!("{:?}", b);
                            self.stack.push(Value::String(format!("{}{}", a_str, b_str)));
                        }
                    }
                }
                Instruction::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a - b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a * b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a / b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(a == b));
                }
                Instruction::Ne => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(a != b));
                }
                Instruction::Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a < b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a > b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Le => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a <= b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Ge => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a >= b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Jump(target) => {
                    ip = *target;
                    continue;
                }
                Instruction::JumpIfTrue(target) => {
                    let cond = self.stack.pop().unwrap();
                    if cond.as_bool().unwrap_or(false) {
                        ip = *target;
                        continue;
                    }
                }
                Instruction::JumpIfFalse(target) => {
                    let cond = self.stack.pop().unwrap();
                    if !cond.as_bool().unwrap_or(false) {
                        ip = *target;
                        continue;
                    }
                }
                Instruction::LoadLocal(idx) => {
                    let value = locals.get(*idx).cloned().unwrap_or(Value::Undefined);
                    self.stack.push(value);
                }
                Instruction::StoreLocal(idx) => {
                    let value = self.stack.pop().unwrap();
                    if let Some(slot) = locals.get_mut(*idx) {
                        *slot = value;
                    }
                }
                Instruction::LoadGlobal(idx) => {
                    // Implementar acesso a variáveis globais
                    self.stack.push(self.globals.get(*idx).cloned().unwrap_or(Value::Undefined));
                }
                Instruction::StoreGlobal(idx) => {
                    // Implementar armazenamento em variáveis globais
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
                    if let Some(top) = self.stack.values.last().cloned() {
                        self.stack.push(top);
                    }
                }
                Instruction::NewObject => {
                    let handle = self.heap.alloc_object();
                    self.stack.push(Value::Object(handle));
                }
                Instruction::NewArray(_size) => {
                    let handle = self.heap.alloc_array();
                    self.stack.push(Value::Array(handle));
                }
                Instruction::SetProperty => {
                    let value = self.stack.pop().unwrap();
                    let key = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    if let (Value::Object(handle), Value::String(key)) = (obj, key) {
                        self.heap.set_object_property(handle, key, value);
                    }
                }
                Instruction::GetProperty => {
                    let key = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    if let (Value::Object(handle), Value::String(key)) = (obj, key) {
                        if let Some(val) = self.heap.get_object_property(handle, &key) {
                            self.stack.push(val.clone());
                        } else {
                            self.stack.push(Value::Undefined);
                        }
                    } else {
                        self.stack.push(Value::Undefined);
                    }
                }
                _ => todo!("Instrução não implementada ainda"),
            }
            ip += 1;
        }
    }
} 