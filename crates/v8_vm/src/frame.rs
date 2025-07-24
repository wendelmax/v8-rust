//! Frame for the V8-Rust VM

#[derive(Debug, Clone)]
pub struct Frame {
    pub return_address: usize,
    pub arg_count: usize,
    pub locals: Vec<i64>,
    pub base_pointer: usize,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            return_address: 0,
            arg_count: 0,
            locals: vec![0i64; 16], // 16 variáveis locais por frame
            base_pointer: 0,
        }
    }
    
    pub fn with_return_address(return_address: usize) -> Self {
        Frame {
            return_address,
            arg_count: 0,
            locals: vec![0i64; 16],
            base_pointer: 0,
        }
    }
} 