//! Frame for the V8-Rust VM

use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Frame {
    pub return_address: usize,
    pub arg_count: usize,
    pub locals: Vec<i64>,
    pub base_pointer: usize,
    pub arguments: Vec<Value>,
    pub closure_vars: HashMap<String, Value>,
    pub function_handle: Option<usize>, // Handle da função atual (para recursão)
    pub this_value: Option<Value>, // Valor de this da função atual
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            return_address: 0,
            arg_count: 0,
            locals: vec![0i64; 16], // 16 variáveis locais por frame
            base_pointer: 0,
            arguments: Vec::new(),
            closure_vars: HashMap::new(),
            function_handle: None,
            this_value: None,
        }
    }
    
    pub fn with_return_address(return_address: usize) -> Self {
        Frame {
            return_address,
            arg_count: 0,
            locals: vec![0i64; 16],
            base_pointer: 0,
            arguments: Vec::new(),
            closure_vars: HashMap::new(),
            function_handle: None,
            this_value: None,
        }
    }
} 