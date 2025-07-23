// Instruction set for ECMAScript VM

use crate::runtime::Value;

#[derive(Debug, Clone)]
pub enum Instruction {
    // Stack operations
    Pop,
    Dup,
    Swap,
    
    // Constants
    LoadConstant(usize),
    LoadNull,
    LoadUndefined,
    LoadTrue,
    LoadFalse,
    LoadThis,
    
    // Variables
    LoadLocal(usize),
    StoreLocal(usize),
    LoadGlobal(usize),
    StoreGlobal(usize),
    LoadProperty,
    StoreProperty,
    DeleteProperty,
    
    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponentiate,
    UnaryPlus,
    UnaryMinus,
    
    // Comparison operations
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    
    // Logical operations
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    
    // Bitwise operations
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    
    // Increment/Decrement
    Increment,
    Decrement,
    PostIncrement,
    PostDecrement,
    
    // Type operations
    TypeOf,
    InstanceOf,
    In,
    Void,
    Delete,
    
    // Control flow
    Jump(usize),
    JumpIfTrue(usize),
    JumpIfFalse(usize),
    JumpIfNull(usize),
    JumpIfNotNull(usize),
    JumpIfUndefined(usize),
    JumpIfNotUndefined(usize),
    
    // Function operations
    Call(usize), // argument count
    New(usize),  // argument count
    Return,
    Yield,
    YieldDelegate,
    Await,
    
    // Object operations
    CreateObject(usize), // property count
    CreateArray(usize),  // element count
    CreateFunction(usize), // function index
    CreateClass(usize),  // class index
    
    // Exception handling
    Throw,
    Try,
    Catch,
    Finally,
    
    // Debug
    Debugger,
    Breakpoint,
    
    // Special operations
    Spread,
    Rest,
    OptionalChaining,
    NullishCoalescing,
    
    // Module operations
    Import(usize), // module index
    Export(usize), // export index
    DynamicImport,
    
    // Generator operations
    GeneratorNext,
    GeneratorReturn,
    GeneratorThrow,
    
    // Async operations
    AsyncFunctionStart,
    AsyncFunctionEnd,
    
    // Optimization hints
    Optimize,
    Deoptimize,
    
    // Memory management
    Allocate(usize), // size in bytes
    Free,
    GcHint,
}

impl Instruction {
    pub fn disassemble(&self, bytecode: &super::bytecode::Bytecode) -> String {
        match self {
            Instruction::Pop => "POP".to_string(),
            Instruction::Dup => "DUP".to_string(),
            Instruction::Swap => "SWAP".to_string(),
            
            Instruction::LoadConstant(index) => {
                if let Some(constant) = bytecode.get_constant(*index) {
                    format!("LOAD_CONST {} ; {:?}", index, constant)
                } else {
                    format!("LOAD_CONST {} ; <invalid>", index)
                }
            }
            Instruction::LoadNull => "LOAD_NULL".to_string(),
            Instruction::LoadUndefined => "LOAD_UNDEFINED".to_string(),
            Instruction::LoadTrue => "LOAD_TRUE".to_string(),
            Instruction::LoadFalse => "LOAD_FALSE".to_string(),
            Instruction::LoadThis => "LOAD_THIS".to_string(),
            
            Instruction::LoadLocal(index) => format!("LOAD_LOCAL {}", index),
            Instruction::StoreLocal(index) => format!("STORE_LOCAL {}", index),
            Instruction::LoadGlobal(index) => {
                if let Some(name) = bytecode.get_string(*index) {
                    format!("LOAD_GLOBAL {} ; {}", index, name)
                } else {
                    format!("LOAD_GLOBAL {} ; <invalid>", index)
                }
            }
            Instruction::StoreGlobal(index) => {
                if let Some(name) = bytecode.get_string(*index) {
                    format!("STORE_GLOBAL {} ; {}", index, name)
                } else {
                    format!("STORE_GLOBAL {} ; <invalid>", index)
                }
            }
            Instruction::LoadProperty => "LOAD_PROPERTY".to_string(),
            Instruction::StoreProperty => "STORE_PROPERTY".to_string(),
            Instruction::DeleteProperty => "DELETE_PROPERTY".to_string(),
            
            Instruction::Add => "ADD".to_string(),
            Instruction::Subtract => "SUBTRACT".to_string(),
            Instruction::Multiply => "MULTIPLY".to_string(),
            Instruction::Divide => "DIVIDE".to_string(),
            Instruction::Modulo => "MODULO".to_string(),
            Instruction::Exponentiate => "EXPONENTIATE".to_string(),
            Instruction::UnaryPlus => "UNARY_PLUS".to_string(),
            Instruction::UnaryMinus => "UNARY_MINUS".to_string(),
            
            Instruction::Equal => "EQUAL".to_string(),
            Instruction::NotEqual => "NOT_EQUAL".to_string(),
            Instruction::StrictEqual => "STRICT_EQUAL".to_string(),
            Instruction::StrictNotEqual => "STRICT_NOT_EQUAL".to_string(),
            Instruction::LessThan => "LESS_THAN".to_string(),
            Instruction::LessThanEqual => "LESS_THAN_EQUAL".to_string(),
            Instruction::GreaterThan => "GREATER_THAN".to_string(),
            Instruction::GreaterThanEqual => "GREATER_THAN_EQUAL".to_string(),
            
            Instruction::LogicalAnd => "LOGICAL_AND".to_string(),
            Instruction::LogicalOr => "LOGICAL_OR".to_string(),
            Instruction::LogicalNot => "LOGICAL_NOT".to_string(),
            
            Instruction::BitwiseAnd => "BITWISE_AND".to_string(),
            Instruction::BitwiseOr => "BITWISE_OR".to_string(),
            Instruction::BitwiseXor => "BITWISE_XOR".to_string(),
            Instruction::BitwiseNot => "BITWISE_NOT".to_string(),
            Instruction::LeftShift => "LEFT_SHIFT".to_string(),
            Instruction::RightShift => "RIGHT_SHIFT".to_string(),
            Instruction::UnsignedRightShift => "UNSIGNED_RIGHT_SHIFT".to_string(),
            
            Instruction::Increment => "INCREMENT".to_string(),
            Instruction::Decrement => "DECREMENT".to_string(),
            Instruction::PostIncrement => "POST_INCREMENT".to_string(),
            Instruction::PostDecrement => "POST_DECREMENT".to_string(),
            
            Instruction::TypeOf => "TYPEOF".to_string(),
            Instruction::InstanceOf => "INSTANCEOF".to_string(),
            Instruction::In => "IN".to_string(),
            Instruction::Void => "VOID".to_string(),
            Instruction::Delete => "DELETE".to_string(),
            
            Instruction::Jump(target) => format!("JUMP {}", target),
            Instruction::JumpIfTrue(target) => format!("JUMP_IF_TRUE {}", target),
            Instruction::JumpIfFalse(target) => format!("JUMP_IF_FALSE {}", target),
            Instruction::JumpIfNull(target) => format!("JUMP_IF_NULL {}", target),
            Instruction::JumpIfNotNull(target) => format!("JUMP_IF_NOT_NULL {}", target),
            Instruction::JumpIfUndefined(target) => format!("JUMP_IF_UNDEFINED {}", target),
            Instruction::JumpIfNotUndefined(target) => format!("JUMP_IF_NOT_UNDEFINED {}", target),
            
            Instruction::Call(arg_count) => format!("CALL {}", arg_count),
            Instruction::New(arg_count) => format!("NEW {}", arg_count),
            Instruction::Return => "RETURN".to_string(),
            Instruction::Yield => "YIELD".to_string(),
            Instruction::YieldDelegate => "YIELD_DELEGATE".to_string(),
            Instruction::Await => "AWAIT".to_string(),
            
            Instruction::CreateObject(prop_count) => format!("CREATE_OBJECT {}", prop_count),
            Instruction::CreateArray(elem_count) => format!("CREATE_ARRAY {}", elem_count),
            Instruction::CreateFunction(func_index) => format!("CREATE_FUNCTION {}", func_index),
            Instruction::CreateClass(class_index) => format!("CREATE_CLASS {}", class_index),
            
            Instruction::Throw => "THROW".to_string(),
            Instruction::Try => "TRY".to_string(),
            Instruction::Catch => "CATCH".to_string(),
            Instruction::Finally => "FINALLY".to_string(),
            
            Instruction::Debugger => "DEBUGGER".to_string(),
            Instruction::Breakpoint => "BREAKPOINT".to_string(),
            
            Instruction::Spread => "SPREAD".to_string(),
            Instruction::Rest => "REST".to_string(),
            Instruction::OptionalChaining => "OPTIONAL_CHAINING".to_string(),
            Instruction::NullishCoalescing => "NULLISH_COALESCING".to_string(),
            
            Instruction::Import(module_index) => format!("IMPORT {}", module_index),
            Instruction::Export(export_index) => format!("EXPORT {}", export_index),
            Instruction::DynamicImport => "DYNAMIC_IMPORT".to_string(),
            
            Instruction::GeneratorNext => "GENERATOR_NEXT".to_string(),
            Instruction::GeneratorReturn => "GENERATOR_RETURN".to_string(),
            Instruction::GeneratorThrow => "GENERATOR_THROW".to_string(),
            
            Instruction::AsyncFunctionStart => "ASYNC_FUNCTION_START".to_string(),
            Instruction::AsyncFunctionEnd => "ASYNC_FUNCTION_END".to_string(),
            
            Instruction::Optimize => "OPTIMIZE".to_string(),
            Instruction::Deoptimize => "DEOPTIMIZE".to_string(),
            
            Instruction::Allocate(size) => format!("ALLOCATE {}", size),
            Instruction::Free => "FREE".to_string(),
            Instruction::GcHint => "GC_HINT".to_string(),
        }
    }
    
    pub fn get_stack_effect(&self) -> i32 {
        match self {
            // Stack operations
            Instruction::Pop => -1,
            Instruction::Dup => 1,
            Instruction::Swap => 0,
            
            // Constants (push one value)
            Instruction::LoadConstant(_) => 1,
            Instruction::LoadNull => 1,
            Instruction::LoadUndefined => 1,
            Instruction::LoadTrue => 1,
            Instruction::LoadFalse => 1,
            Instruction::LoadThis => 1,
            
            // Variables (push one value)
            Instruction::LoadLocal(_) => 1,
            Instruction::StoreLocal(_) => -1,
            Instruction::LoadGlobal(_) => 1,
            Instruction::StoreGlobal(_) => -1,
            Instruction::LoadProperty => -1, // pops object and property, pushes value
            Instruction::StoreProperty => -3, // pops object, property, and value
            Instruction::DeleteProperty => -2, // pops object and property, pushes boolean
            
            // Binary arithmetic (pops 2, pushes 1)
            Instruction::Add => -1,
            Instruction::Subtract => -1,
            Instruction::Multiply => -1,
            Instruction::Divide => -1,
            Instruction::Modulo => -1,
            Instruction::Exponentiate => -1,
            
            // Unary arithmetic (pops 1, pushes 1)
            Instruction::UnaryPlus => 0,
            Instruction::UnaryMinus => 0,
            
            // Binary comparison (pops 2, pushes 1)
            Instruction::Equal => -1,
            Instruction::NotEqual => -1,
            Instruction::StrictEqual => -1,
            Instruction::StrictNotEqual => -1,
            Instruction::LessThan => -1,
            Instruction::LessThanEqual => -1,
            Instruction::GreaterThan => -1,
            Instruction::GreaterThanEqual => -1,
            
            // Logical operations
            Instruction::LogicalAnd => -1, // pops 2, pushes 1 (short-circuit)
            Instruction::LogicalOr => -1,  // pops 2, pushes 1 (short-circuit)
            Instruction::LogicalNot => 0,  // pops 1, pushes 1
            
            // Binary bitwise (pops 2, pushes 1)
            Instruction::BitwiseAnd => -1,
            Instruction::BitwiseOr => -1,
            Instruction::BitwiseXor => -1,
            Instruction::BitwiseNot => 0,  // pops 1, pushes 1
            Instruction::LeftShift => -1,
            Instruction::RightShift => -1,
            Instruction::UnsignedRightShift => -1,
            
            // Increment/Decrement (pops 1, pushes 1)
            Instruction::Increment => 0,
            Instruction::Decrement => 0,
            Instruction::PostIncrement => 0,
            Instruction::PostDecrement => 0,
            
            // Type operations (pops 1, pushes 1)
            Instruction::TypeOf => 0,
            Instruction::InstanceOf => -1, // pops 2, pushes 1
            Instruction::In => -1,         // pops 2, pushes 1
            Instruction::Void => 0,        // pops 1, pushes 1
            Instruction::Delete => -1,     // pops 1, pushes 1
            
            // Control flow (no stack effect)
            Instruction::Jump(_) => 0,
            Instruction::JumpIfTrue(_) => -1,
            Instruction::JumpIfFalse(_) => -1,
            Instruction::JumpIfNull(_) => -1,
            Instruction::JumpIfNotNull(_) => -1,
            Instruction::JumpIfUndefined(_) => -1,
            Instruction::JumpIfNotUndefined(_) => -1,
            
            // Function operations
            Instruction::Call(arg_count) => -(*arg_count as i32) + 1, // pops args + function, pushes result
            Instruction::New(arg_count) => -(*arg_count as i32) + 1,  // pops args + constructor, pushes object
            Instruction::Return => -1, // pops return value
            Instruction::Yield => 0,   // no stack effect
            Instruction::YieldDelegate => 0,
            Instruction::Await => 0,   // pops promise, pushes result
            
            // Object operations
            Instruction::CreateObject(prop_count) => -(*prop_count as i32 * 2) + 1, // pops key-value pairs, pushes object
            Instruction::CreateArray(elem_count) => -(*elem_count as i32) + 1,      // pops elements, pushes array
            Instruction::CreateFunction(_) => 1,  // pushes function
            Instruction::CreateClass(_) => 1,     // pushes class
            
            // Exception handling
            Instruction::Throw => -1, // pops exception
            Instruction::Try => 0,
            Instruction::Catch => 0,
            Instruction::Finally => 0,
            
            // Debug (no stack effect)
            Instruction::Debugger => 0,
            Instruction::Breakpoint => 0,
            
            // Special operations
            Instruction::Spread => 0, // pops array, pushes elements
            Instruction::Rest => 0,   // no stack effect
            Instruction::OptionalChaining => 0,
            Instruction::NullishCoalescing => -1, // pops 2, pushes 1
            
            // Module operations
            Instruction::Import(_) => 1, // pushes module
            Instruction::Export(_) => 0,
            Instruction::DynamicImport => 0,
            
            // Generator operations
            Instruction::GeneratorNext => 0,
            Instruction::GeneratorReturn => 0,
            Instruction::GeneratorThrow => 0,
            
            // Async operations
            Instruction::AsyncFunctionStart => 0,
            Instruction::AsyncFunctionEnd => 0,
            
            // Optimization hints (no stack effect)
            Instruction::Optimize => 0,
            Instruction::Deoptimize => 0,
            
            // Memory management (no stack effect)
            Instruction::Allocate(_) => 0,
            Instruction::Free => 0,
            Instruction::GcHint => 0,
        }
    }
    
    pub fn is_terminator(&self) -> bool {
        matches!(self,
            Instruction::Return |
            Instruction::Throw |
            Instruction::Jump(_) |
            Instruction::JumpIfTrue(_) |
            Instruction::JumpIfFalse(_) |
            Instruction::JumpIfNull(_) |
            Instruction::JumpIfNotNull(_) |
            Instruction::JumpIfUndefined(_) |
            Instruction::JumpIfNotUndefined(_)
        )
    }
    
    pub fn is_branch(&self) -> bool {
        matches!(self,
            Instruction::JumpIfTrue(_) |
            Instruction::JumpIfFalse(_) |
            Instruction::JumpIfNull(_) |
            Instruction::JumpIfNotNull(_) |
            Instruction::JumpIfUndefined(_) |
            Instruction::JumpIfNotUndefined(_)
        )
    }
    
    pub fn is_unconditional_jump(&self) -> bool {
        matches!(self, Instruction::Jump(_))
    }
    
    pub fn get_jump_target(&self) -> Option<usize> {
        match self {
            Instruction::Jump(target) => Some(*target),
            Instruction::JumpIfTrue(target) => Some(*target),
            Instruction::JumpIfFalse(target) => Some(*target),
            Instruction::JumpIfNull(target) => Some(*target),
            Instruction::JumpIfNotNull(target) => Some(*target),
            Instruction::JumpIfUndefined(target) => Some(*target),
            Instruction::JumpIfNotUndefined(target) => Some(*target),
            _ => None,
        }
    }
    
    pub fn set_jump_target(&mut self, target: usize) -> Result<(), String> {
        match self {
            Instruction::Jump(t) => { *t = target; Ok(()) }
            Instruction::JumpIfTrue(t) => { *t = target; Ok(()) }
            Instruction::JumpIfFalse(t) => { *t = target; Ok(()) }
            Instruction::JumpIfNull(t) => { *t = target; Ok(()) }
            Instruction::JumpIfNotNull(t) => { *t = target; Ok(()) }
            Instruction::JumpIfUndefined(t) => { *t = target; Ok(()) }
            Instruction::JumpIfNotUndefined(t) => { *t = target; Ok(()) }
            _ => Err("Instruction is not a jump instruction".to_string()),
        }
    }
    
    pub fn is_call(&self) -> bool {
        matches!(self, Instruction::Call(_) | Instruction::New(_))
    }
    
    pub fn is_arithmetic(&self) -> bool {
        matches!(self,
            Instruction::Add |
            Instruction::Subtract |
            Instruction::Multiply |
            Instruction::Divide |
            Instruction::Modulo |
            Instruction::Exponentiate |
            Instruction::UnaryPlus |
            Instruction::UnaryMinus
        )
    }
    
    pub fn is_comparison(&self) -> bool {
        matches!(self,
            Instruction::Equal |
            Instruction::NotEqual |
            Instruction::StrictEqual |
            Instruction::StrictNotEqual |
            Instruction::LessThan |
            Instruction::LessThanEqual |
            Instruction::GreaterThan |
            Instruction::GreaterThanEqual
        )
    }
    
    pub fn is_logical(&self) -> bool {
        matches!(self,
            Instruction::LogicalAnd |
            Instruction::LogicalOr |
            Instruction::LogicalNot
        )
    }
    
    pub fn is_bitwise(&self) -> bool {
        matches!(self,
            Instruction::BitwiseAnd |
            Instruction::BitwiseOr |
            Instruction::BitwiseXor |
            Instruction::BitwiseNot |
            Instruction::LeftShift |
            Instruction::RightShift |
            Instruction::UnsignedRightShift
        )
    }
} 