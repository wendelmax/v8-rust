// Virtual Machine for ECMAScript

pub mod bytecode;
pub mod executor;
pub mod instructions;
pub mod frame;
pub mod stack;
pub mod registers;
pub mod heap;
pub mod gc;
pub mod jit;
pub mod optimizer;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime::{Runtime, Value, Context};
use crate::parser::{lexer, ast, parser};

pub use bytecode::Bytecode;
pub use executor::Executor;
pub use instructions::Instruction;
pub use frame::Frame;
pub use stack::Stack;
pub use registers::Registers;
pub use heap::Heap;
pub use gc::GarbageCollector;
pub use jit::JitCompiler;
pub use optimizer::Optimizer;

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    pub runtime: Runtime,
    pub executor: Executor,
    pub heap: Heap,
    pub gc: GarbageCollector,
    pub jit: JitCompiler,
    pub optimizer: Optimizer,
    pub contexts: HashMap<String, Context>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let runtime = Runtime::new();
        let executor = Executor::new();
        let heap = Heap::new();
        let gc = GarbageCollector::new(heap.clone());
        let jit = JitCompiler::new();
        let optimizer = Optimizer::new();
        
        Self {
            runtime,
            executor,
            heap,
            gc,
            jit,
            optimizer,
            contexts: HashMap::new(),
        }
    }
    
    pub fn execute(&mut self, source: &str) -> Result<Value, String> {
        // Parse the source code
        let tokens = lexer::tokenize(source);
        let ast = parser::parse(&tokens);
        
        // Generate bytecode
        let bytecode = Bytecode::from_ast(&ast)?;
        
        // Execute the bytecode
        self.executor.execute(&bytecode, &mut self.runtime)
    }
    
    pub fn execute_bytecode(&mut self, bytecode: &Bytecode) -> Result<Value, String> {
        self.executor.execute(bytecode, &mut self.runtime)
    }
    
    pub fn create_context(&mut self, name: String) -> Context {
        let context = self.runtime.create_context(name.clone());
        self.contexts.insert(name, context.clone());
        context
    }
    
    pub fn get_context(&self, name: &str) -> Option<&Context> {
        self.contexts.get(name)
    }
    
    pub fn run_gc(&mut self) {
        self.gc.collect(&mut self.heap);
    }
    
    pub fn optimize(&mut self, bytecode: &mut Bytecode) {
        self.optimizer.optimize(bytecode);
    }
    
    pub fn compile_jit(&mut self, bytecode: &Bytecode) -> Result<Vec<u8>, String> {
        self.jit.compile(bytecode)
    }
    
    pub fn get_heap_stats(&self) -> HeapStats {
        self.heap.get_stats()
    }
    
    pub fn get_gc_stats(&self) -> GcStats {
        self.gc.get_stats()
    }
}

#[derive(Debug, Clone)]
pub struct HeapStats {
    pub total_allocated: usize,
    pub total_used: usize,
    pub total_free: usize,
    pub object_count: usize,
    pub array_count: usize,
    pub function_count: usize,
    pub string_count: usize,
}

#[derive(Debug, Clone)]
pub struct GcStats {
    pub collections: usize,
    pub total_time: f64,
    pub objects_freed: usize,
    pub memory_freed: usize,
}

// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vm_creation() {
        let vm = VirtualMachine::new();
        assert!(vm.contexts.is_empty());
    }
    
    #[test]
    fn test_simple_execution() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("42");
        assert!(result.is_ok());
        if let Ok(Value::Number(n)) = result {
            assert_eq!(n, 42.0);
        } else {
            panic!("Expected number value");
        }
    }
    
    #[test]
    fn test_variable_declaration() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("let x = 10; x");
        assert!(result.is_ok());
        if let Ok(Value::Number(n)) = result {
            assert_eq!(n, 10.0);
        } else {
            panic!("Expected number value");
        }
    }
    
    #[test]
    fn test_binary_expression() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("2 + 3");
        assert!(result.is_ok());
        if let Ok(Value::Number(n)) = result {
            assert_eq!(n, 5.0);
        } else {
            panic!("Expected number value");
        }
    }
    
    #[test]
    fn test_string_literal() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("\"hello world\"");
        assert!(result.is_ok());
        if let Ok(Value::String(s)) = result {
            assert_eq!(s, "hello world");
        } else {
            panic!("Expected string value");
        }
    }
    
    #[test]
    fn test_boolean_literal() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("true");
        assert!(result.is_ok());
        if let Ok(Value::Boolean(b)) = result {
            assert!(b);
        } else {
            panic!("Expected boolean value");
        }
    }
    
    #[test]
    fn test_null_literal() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("null");
        assert!(result.is_ok());
        if let Ok(Value::Null) = result {
            // Success
        } else {
            panic!("Expected null value");
        }
    }
    
    #[test]
    fn test_undefined_literal() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("undefined");
        assert!(result.is_ok());
        if let Ok(Value::Undefined) = result {
            // Success
        } else {
            panic!("Expected undefined value");
        }
    }
    
    #[test]
    fn test_array_literal() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("[1, 2, 3]");
        assert!(result.is_ok());
        if let Ok(Value::Array(arr)) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Value::Number(1.0));
            assert_eq!(arr[1], Value::Number(2.0));
            assert_eq!(arr[2], Value::Number(3.0));
        } else {
            panic!("Expected array value");
        }
    }
    
    #[test]
    fn test_object_literal() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("{a: 1, b: 2}");
        assert!(result.is_ok());
        if let Ok(Value::Object(obj)) = result {
            // TODO: Test object properties
        } else {
            panic!("Expected object value");
        }
    }
    
    #[test]
    fn test_function_call() {
        let mut vm = VirtualMachine::new();
        let result = vm.execute("parseInt('42')");
        assert!(result.is_ok());
        if let Ok(Value::Number(n)) = result {
            assert_eq!(n, 42.0);
        } else {
            panic!("Expected number value");
        }
    }
    
    #[test]
    fn test_context_creation() {
        let mut vm = VirtualMachine::new();
        let context = vm.create_context("test".to_string());
        assert!(vm.get_context("test").is_some());
    }
    
    #[test]
    fn test_gc_stats() {
        let mut vm = VirtualMachine::new();
        let stats = vm.get_gc_stats();
        assert_eq!(stats.collections, 0);
        assert_eq!(stats.total_time, 0.0);
        assert_eq!(stats.objects_freed, 0);
        assert_eq!(stats.memory_freed, 0);
    }
    
    #[test]
    fn test_heap_stats() {
        let vm = VirtualMachine::new();
        let stats = vm.get_heap_stats();
        assert_eq!(stats.object_count, 0);
        assert_eq!(stats.array_count, 0);
        assert_eq!(stats.function_count, 0);
        assert_eq!(stats.string_count, 0);
    }
}
