//! Executor for the V8-Rust VM

use crate::bytecode::Bytecode;
use crate::frame::Frame;
use crate::heap::HeapEntry;
use crate::heap::{HandleId, Heap};
use crate::instructions::Instruction;
use crate::registers::Registers;
use crate::stack::Stack;
use crate::value::Value;

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
                    // Verificar se o valor no topo da stack é uma função
                    let func_value = if let Some(top_value) = self.stack.values.last() {
                        if let Value::Function(_) = top_value {
                            // Se o topo é uma função, fazer pop
                            self.stack.pop().unwrap()
                        } else {
                            // Se não é uma função, procurar pela função na stack
                            // Isso pode acontecer quando LoadThisFunction foi usado
                            let mut found_func = None;
                            for (i, value) in self.stack.values.iter().enumerate().rev() {
                                if let Value::Function(_) = value {
                                    found_func = Some((i, value.clone()));
                                    break;
                                }
                            }
                            if let Some((index, func)) = found_func {
                                // Remover a função da posição encontrada
                                self.stack.values.remove(index);
                                func
                            } else {
                                panic!("Nenhuma função encontrada na stack para Call");
                            }
                        }
                    } else {
                        panic!("Stack vazia ao executar Call");
                    };
                    
                    if let Value::Function(handle) = func_value {
                        // Extrair dados necessários antes de chamar self.execute
                        let (bytecode, closure_vars) = if let Some(HeapEntry::Function { bytecode, closure_vars, .. }) = self.heap.get(handle) {
                            (bytecode.clone(), closure_vars.clone())
                        } else {
                            panic!("Handle de função inválido no heap");
                        };
                        // Preparar argumentos
                        let mut args = Vec::new();
                        for _ in 0..*argc {
                            args.push(self.stack.pop().unwrap());
                        }
                        args.reverse(); // Ordem correta
                        // Verificar se há um valor de this na stack (opcional)
                        // Se não há mais valores na stack, this_value será None
                        let this_value = self.stack.pop();
                        // Criar novo frame
                        let mut new_frame = Frame::new();
                        new_frame.return_address = ip + 1;
                        new_frame.arg_count = *argc;
                        new_frame.arguments = args;
                        new_frame.closure_vars = closure_vars;
                        new_frame.function_handle = Some(handle); // Passar handle da função
                        new_frame.this_value = this_value; // Passar valor de this (pode ser None)
                        // Empilhar o frame atual e usar o novo
                        self.stack.push_frame(self.frame.clone());
                        self.frame = new_frame;
                        // Executar o bytecode da função
                        self.execute(&bytecode, constants); // Passar pool de constantes correto
                        // Após execução, restaurar frame anterior
                        if let Some(prev_frame) = self.stack.pop_frame() {
                            self.frame = prev_frame;
                        }
                        // Restaurar endereço de retorno
                        if let Some(return_ip) = call_stack.pop() {
                            ip = return_ip;
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        panic!("Topo da stack não é uma função ao executar Call");
                    }
                }
                Instruction::CallFunction(handle, argc) => {
                    println!(
                        "DEBUG: CallFunction({}, {}) - Stack antes: {:?}",
                        handle, argc, self.stack.values
                    );

                    let handle = HandleId::from(handle);

                    if let Some(HeapEntry::Function {
                        bytecode,
                        closure_vars,
                        ..
                    }) = self.heap.get(handle)
                    {
                        let bytecode = bytecode.clone();
                        let closure_vars = closure_vars.clone();
                        
                        // Preparar argumentos
                        let mut args = Vec::new();
                        for _ in 0..*argc {
                            args.push(self.stack.pop().unwrap());
                        }
                        args.reverse(); // Ordem correta
                        
                        // Verificar se há um valor de this na stack (opcional)
                        let this_value = self.stack.pop();
                        println!("DEBUG: CallFunction - Argumentos: {:?}, This: {:?}", args, this_value);
                        
                        // Criar novo frame
                        let mut new_frame = Frame::new();
                        new_frame.return_address = ip + 1;
                        new_frame.arg_count = *argc;
                        new_frame.arguments = args;
                        new_frame.closure_vars = closure_vars;
                        new_frame.function_handle = Some(handle);
                        new_frame.this_value = this_value;
                        
                        // Empilhar o frame atual e usar o novo
                        self.stack.push_frame(self.frame.clone());
                        self.frame = new_frame;
                        
                        // Executar o bytecode da função
                        self.execute(&bytecode, constants);
                        
                        // Após execução, restaurar frame anterior
                        if let Some(prev_frame) = self.stack.pop_frame() {
                            self.frame = prev_frame;
                        }
                        
                        // Restaurar endereço de retorno
                        if let Some(return_ip) = call_stack.pop() {
                            ip = return_ip;
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        panic!("Handle de função inválido no heap: {:?}", handle);
                    }
                }
                Instruction::Return => {
                    // Recuperar valor de retorno (se houver)
                    let return_value = self.stack.pop();
                    
                    // Restaurar frame anterior
                    if let Some(prev_frame) = self.stack.pop_frame() {
                        self.frame = prev_frame;
                    }
                    
                    // Empilhar o valor retornado na stack do chamador
                    if let Some(value) = return_value {
                        self.stack.push(value);
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
                Instruction::LoadArg(idx) => {
                    let value = self.frame.arguments.get(*idx).cloned().unwrap_or(Value::Undefined);
                    self.stack.push(value);
                }
                Instruction::LoadThisFunction => {
                    // Empilha o handle da função atual
                    if let Some(func_handle) = self.frame.function_handle {
                        self.stack.push(Value::Function(func_handle));
                    } else {
                        panic!("LoadThisFunction chamado fora de uma função");
                    }
                }
                Instruction::LoadThis => {
                    // Empilha o valor de this da função atual
                    if let Some(this_val) = &self.frame.this_value {
                        self.stack.push(this_val.clone());
                    } else {
                        self.stack.push(Value::Undefined);
                    }
                }
                Instruction::LoadClosureVar(name) => {
                    // Empilha uma variável capturada do escopo externo
                    if let Some(value) = self.frame.closure_vars.get(name) {
                        self.stack.push(value.clone());
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
