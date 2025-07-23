// Bytecode executor for ECMAScript VM

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime::{Runtime, Value, Context, Object, Function};
use super::bytecode::Bytecode;
use super::instructions::Instruction;
use super::frame::Frame;
use super::stack::Stack;
use super::registers::Registers;

#[derive(Debug, Clone)]
pub struct Executor {
    pub stack: Stack,
    pub registers: Registers,
    pub frames: Vec<Frame>,
    pub current_frame: Option<usize>,
    pub call_stack: Vec<usize>,
    pub exception_handler: Option<ExceptionHandler>,
    pub debug_mode: bool,
}

#[derive(Debug, Clone)]
pub struct ExceptionHandler {
    pub try_pc: usize,
    pub catch_pc: usize,
    pub finally_pc: Option<usize>,
    pub frame_index: usize,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            registers: Registers::new(),
            frames: Vec::new(),
            current_frame: None,
            call_stack: Vec::new(),
            exception_handler: None,
            debug_mode: false,
        }
    }
    
    pub fn execute(&mut self, bytecode: &Bytecode, runtime: &mut Runtime) -> Result<Value, String> {
        // Create initial frame
        let frame = Frame::new(bytecode.clone(), 0);
        let frame_index = self.frames.len();
        self.frames.push(frame);
        self.current_frame = Some(frame_index);
        
        // Execute bytecode
        let result = self.execute_frame(frame_index, runtime);
        
        // Clean up
        self.frames.clear();
        self.current_frame = None;
        self.call_stack.clear();
        self.exception_handler = None;
        
        result
    }
    
    fn execute_frame(&mut self, frame_index: usize, runtime: &mut Runtime) -> Result<Value, String> {
        let frame = &mut self.frames[frame_index];
        let mut pc = frame.pc;
        let bytecode = &frame.bytecode;
        
        while pc < bytecode.len() {
            if let Some(instruction) = bytecode.get_instruction(pc) {
                if self.debug_mode {
                    println!("PC: {:04x}, Stack: {:?}, Instruction: {}", 
                        pc, self.stack, instruction.disassemble(bytecode));
                }
                
                match self.execute_instruction(instruction, bytecode, runtime) {
                    Ok(ExecutionResult::Continue) => {
                        pc += 1;
                        frame.pc = pc;
                    }
                    Ok(ExecutionResult::Jump(target)) => {
                        pc = target;
                        frame.pc = pc;
                    }
                    Ok(ExecutionResult::Return(value)) => {
                        return Ok(value);
                    }
                    Ok(ExecutionResult::Call(target_frame)) => {
                        // Push current frame onto call stack
                        self.call_stack.push(frame_index);
                        
                        // Execute the called frame
                        let result = self.execute_frame(target_frame, runtime)?;
                        
                        // Pop result onto stack
                        self.stack.push(result);
                        
                        // Restore current frame
                        if let Some(prev_frame) = self.call_stack.pop() {
                            self.current_frame = Some(prev_frame);
                        }
                        
                        pc += 1;
                        frame.pc = pc;
                    }
                    Err(e) => {
                        // Handle exception
                        if let Some(handler) = &self.exception_handler {
                            pc = handler.catch_pc;
                            frame.pc = pc;
                            self.stack.push(Value::String(e));
                        } else {
                            return Err(e);
                        }
                    }
                }
            } else {
                return Err(format!("Invalid PC: {}", pc));
            }
        }
        
        // If we reach the end without a return, return undefined
        Ok(Value::Undefined)
    }
    
    fn execute_instruction(&mut self, instruction: &Instruction, bytecode: &Bytecode, runtime: &mut Runtime) -> Result<ExecutionResult, String> {
        match instruction {
            // Stack operations
            Instruction::Pop => {
                self.stack.pop().ok_or("Stack underflow")?;
                Ok(ExecutionResult::Continue)
            }
            Instruction::Dup => {
                let value = self.stack.peek().ok_or("Stack underflow")?.clone();
                self.stack.push(value);
                Ok(ExecutionResult::Continue)
            }
            Instruction::Swap => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                self.stack.push(b);
                self.stack.push(a);
                Ok(ExecutionResult::Continue)
            }
            
            // Constants
            Instruction::LoadConstant(index) => {
                let constant = bytecode.get_constant(*index)
                    .ok_or(format!("Invalid constant index: {}", index))?
                    .clone();
                self.stack.push(constant);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LoadNull => {
                self.stack.push(Value::Null);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LoadUndefined => {
                self.stack.push(Value::Undefined);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LoadTrue => {
                self.stack.push(Value::Boolean(true));
                Ok(ExecutionResult::Continue)
            }
            Instruction::LoadFalse => {
                self.stack.push(Value::Boolean(false));
                Ok(ExecutionResult::Continue)
            }
            Instruction::LoadThis => {
                // TODO: Get 'this' from current context
                self.stack.push(Value::Undefined);
                Ok(ExecutionResult::Continue)
            }
            
            // Variables
            Instruction::LoadLocal(index) => {
                let value = self.registers.get_local(*index)
                    .ok_or(format!("Invalid local index: {}", index))?
                    .clone();
                self.stack.push(value);
                Ok(ExecutionResult::Continue)
            }
            Instruction::StoreLocal(index) => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                self.registers.set_local(*index, value);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LoadGlobal(index) => {
                let name = bytecode.get_string(*index)
                    .ok_or(format!("Invalid string index: {}", index))?;
                let value = runtime.global_object.borrow().get_property(name)
                    .unwrap_or(Value::Undefined);
                self.stack.push(value);
                Ok(ExecutionResult::Continue)
            }
            Instruction::StoreGlobal(index) => {
                let name = bytecode.get_string(*index)
                    .ok_or(format!("Invalid string index: {}", index))?;
                let value = self.stack.pop().ok_or("Stack underflow")?;
                runtime.global_object.borrow_mut().set_property(name.to_string(), value);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LoadProperty => {
                let property = self.stack.pop().ok_or("Stack underflow")?;
                let object = self.stack.pop().ok_or("Stack underflow")?;
                
                let value = match object {
                    Value::Object(obj) => {
                        let prop_name = property.to_string();
                        obj.borrow().get_property(&prop_name)
                            .unwrap_or(Value::Undefined)
                    }
                    _ => Value::Undefined,
                };
                
                self.stack.push(value);
                Ok(ExecutionResult::Continue)
            }
            Instruction::StoreProperty => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                let property = self.stack.pop().ok_or("Stack underflow")?;
                let object = self.stack.pop().ok_or("Stack underflow")?;
                
                match object {
                    Value::Object(obj) => {
                        let prop_name = property.to_string();
                        obj.borrow_mut().set_property(prop_name, value);
                    }
                    _ => return Err("Cannot set property on non-object".to_string()),
                }
                
                Ok(ExecutionResult::Continue)
            }
            Instruction::DeleteProperty => {
                let property = self.stack.pop().ok_or("Stack underflow")?;
                let object = self.stack.pop().ok_or("Stack underflow")?;
                
                let result = match object {
                    Value::Object(obj) => {
                        let prop_name = property.to_string();
                        obj.borrow_mut().delete_property(&prop_name)
                    }
                    _ => false,
                };
                
                self.stack.push(Value::Boolean(result));
                Ok(ExecutionResult::Continue)
            }
            
            // Arithmetic operations
            Instruction::Add => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.add(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::Subtract => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.subtract(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::Multiply => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.multiply(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::Divide => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.divide(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::Modulo => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.modulo(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::Exponentiate => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.exponentiate(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::UnaryPlus => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                let result = value.unary_plus();
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::UnaryMinus => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                let result = value.unary_minus();
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            
            // Comparison operations
            Instruction::Equal => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = Value::Boolean(a.equals(&b));
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::NotEqual => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = Value::Boolean(!a.equals(&b));
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::StrictEqual => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = Value::Boolean(a.strict_equals(&b));
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::StrictNotEqual => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = Value::Boolean(!a.strict_equals(&b));
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LessThan => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = Value::Boolean(a.to_number() < b.to_number());
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LessThanEqual => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = Value::Boolean(a.to_number() <= b.to_number());
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::GreaterThan => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = Value::Boolean(a.to_number() > b.to_number());
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::GreaterThanEqual => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = Value::Boolean(a.to_number() >= b.to_number());
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            
            // Logical operations
            Instruction::LogicalAnd => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.logical_and(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LogicalOr => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.logical_or(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LogicalNot => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                let result = value.logical_not();
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            
            // Bitwise operations
            Instruction::BitwiseAnd => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.bitwise_and(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::BitwiseOr => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.bitwise_or(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::BitwiseXor => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.bitwise_xor(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::BitwiseNot => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                let result = value.bitwise_not();
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::LeftShift => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.left_shift(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::RightShift => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.right_shift(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::UnsignedRightShift => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = a.unsigned_right_shift(&b);
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            
            // Increment/Decrement
            Instruction::Increment => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                let result = match value {
                    Value::Number(n) => Value::Number(n + 1.0),
                    _ => Value::Number(value.to_number() + 1.0),
                };
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::Decrement => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                let result = match value {
                    Value::Number(n) => Value::Number(n - 1.0),
                    _ => Value::Number(value.to_number() - 1.0),
                };
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::PostIncrement => {
                let value = self.stack.peek().ok_or("Stack underflow")?.clone();
                let result = match value {
                    Value::Number(n) => Value::Number(n + 1.0),
                    _ => Value::Number(value.to_number() + 1.0),
                };
                *self.stack.peek_mut().unwrap() = result;
                self.stack.push(value);
                Ok(ExecutionResult::Continue)
            }
            Instruction::PostDecrement => {
                let value = self.stack.peek().ok_or("Stack underflow")?.clone();
                let result = match value {
                    Value::Number(n) => Value::Number(n - 1.0),
                    _ => Value::Number(value.to_number() - 1.0),
                };
                *self.stack.peek_mut().unwrap() = result;
                self.stack.push(value);
                Ok(ExecutionResult::Continue)
            }
            
            // Type operations
            Instruction::TypeOf => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                let result = Value::String(value.typeof_());
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            Instruction::InstanceOf => {
                let constructor = self.stack.pop().ok_or("Stack underflow")?;
                let object = self.stack.pop().ok_or("Stack underflow")?;
                // TODO: Implement instanceof check
                self.stack.push(Value::Boolean(false));
                Ok(ExecutionResult::Continue)
            }
            Instruction::In => {
                let property = self.stack.pop().ok_or("Stack underflow")?;
                let object = self.stack.pop().ok_or("Stack underflow")?;
                // TODO: Implement 'in' check
                self.stack.push(Value::Boolean(false));
                Ok(ExecutionResult::Continue)
            }
            Instruction::Void => {
                self.stack.pop().ok_or("Stack underflow")?;
                self.stack.push(Value::Undefined);
                Ok(ExecutionResult::Continue)
            }
            Instruction::Delete => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                // TODO: Implement delete operation
                self.stack.push(Value::Boolean(true));
                Ok(ExecutionResult::Continue)
            }
            
            // Control flow
            Instruction::Jump(target) => {
                Ok(ExecutionResult::Jump(*target))
            }
            Instruction::JumpIfTrue(target) => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                if value.to_boolean() {
                    Ok(ExecutionResult::Jump(*target))
                } else {
                    Ok(ExecutionResult::Continue)
                }
            }
            Instruction::JumpIfFalse(target) => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                if !value.to_boolean() {
                    Ok(ExecutionResult::Jump(*target))
                } else {
                    Ok(ExecutionResult::Continue)
                }
            }
            Instruction::JumpIfNull(target) => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                if value.is_null() {
                    Ok(ExecutionResult::Jump(*target))
                } else {
                    Ok(ExecutionResult::Continue)
                }
            }
            Instruction::JumpIfNotNull(target) => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                if !value.is_null() {
                    Ok(ExecutionResult::Jump(*target))
                } else {
                    Ok(ExecutionResult::Continue)
                }
            }
            Instruction::JumpIfUndefined(target) => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                if value.is_undefined() {
                    Ok(ExecutionResult::Jump(*target))
                } else {
                    Ok(ExecutionResult::Continue)
                }
            }
            Instruction::JumpIfNotUndefined(target) => {
                let value = self.stack.pop().ok_or("Stack underflow")?;
                if !value.is_undefined() {
                    Ok(ExecutionResult::Jump(*target))
                } else {
                    Ok(ExecutionResult::Continue)
                }
            }
            
            // Function operations
            Instruction::Call(arg_count) => {
                // TODO: Implement function call
                self.stack.push(Value::Undefined);
                Ok(ExecutionResult::Continue)
            }
            Instruction::New(arg_count) => {
                // TODO: Implement constructor call
                self.stack.push(Value::Object(Object::new()));
                Ok(ExecutionResult::Continue)
            }
            Instruction::Return => {
                let value = self.stack.pop().unwrap_or(Value::Undefined);
                Ok(ExecutionResult::Return(value))
            }
            Instruction::Yield => {
                // TODO: Implement yield
                Ok(ExecutionResult::Continue)
            }
            Instruction::YieldDelegate => {
                // TODO: Implement yield delegate
                Ok(ExecutionResult::Continue)
            }
            Instruction::Await => {
                // TODO: Implement await
                Ok(ExecutionResult::Continue)
            }
            
            // Object operations
            Instruction::CreateObject(prop_count) => {
                let mut obj = Object::new();
                for _ in 0..*prop_count {
                    let value = self.stack.pop().ok_or("Stack underflow")?;
                    let key = self.stack.pop().ok_or("Stack underflow")?;
                    obj.set_property(key.to_string(), value);
                }
                self.stack.push(Value::Object(obj));
                Ok(ExecutionResult::Continue)
            }
            Instruction::CreateArray(elem_count) => {
                let mut arr = Vec::new();
                for _ in 0..*elem_count {
                    let element = self.stack.pop().ok_or("Stack underflow")?;
                    arr.insert(0, element);
                }
                self.stack.push(Value::Array(arr));
                Ok(ExecutionResult::Continue)
            }
            Instruction::CreateFunction(func_index) => {
                // TODO: Implement function creation
                self.stack.push(Value::Undefined);
                Ok(ExecutionResult::Continue)
            }
            Instruction::CreateClass(class_index) => {
                // TODO: Implement class creation
                self.stack.push(Value::Undefined);
                Ok(ExecutionResult::Continue)
            }
            
            // Exception handling
            Instruction::Throw => {
                let exception = self.stack.pop().ok_or("Stack underflow")?;
                Err(exception.to_string())
            }
            Instruction::Try => {
                // TODO: Implement try
                Ok(ExecutionResult::Continue)
            }
            Instruction::Catch => {
                // TODO: Implement catch
                Ok(ExecutionResult::Continue)
            }
            Instruction::Finally => {
                // TODO: Implement finally
                Ok(ExecutionResult::Continue)
            }
            
            // Debug
            Instruction::Debugger => {
                // TODO: Implement debugger
                Ok(ExecutionResult::Continue)
            }
            Instruction::Breakpoint => {
                // TODO: Implement breakpoint
                Ok(ExecutionResult::Continue)
            }
            
            // Special operations
            Instruction::Spread => {
                // TODO: Implement spread
                Ok(ExecutionResult::Continue)
            }
            Instruction::Rest => {
                // TODO: Implement rest
                Ok(ExecutionResult::Continue)
            }
            Instruction::OptionalChaining => {
                // TODO: Implement optional chaining
                Ok(ExecutionResult::Continue)
            }
            Instruction::NullishCoalescing => {
                let b = self.stack.pop().ok_or("Stack underflow")?;
                let a = self.stack.pop().ok_or("Stack underflow")?;
                let result = if a.is_null() || a.is_undefined() { b } else { a };
                self.stack.push(result);
                Ok(ExecutionResult::Continue)
            }
            
            // Module operations
            Instruction::Import(module_index) => {
                // TODO: Implement import
                self.stack.push(Value::Undefined);
                Ok(ExecutionResult::Continue)
            }
            Instruction::Export(export_index) => {
                // TODO: Implement export
                Ok(ExecutionResult::Continue)
            }
            Instruction::DynamicImport => {
                // TODO: Implement dynamic import
                self.stack.push(Value::Undefined);
                Ok(ExecutionResult::Continue)
            }
            
            // Generator operations
            Instruction::GeneratorNext => {
                // TODO: Implement generator next
                Ok(ExecutionResult::Continue)
            }
            Instruction::GeneratorReturn => {
                // TODO: Implement generator return
                Ok(ExecutionResult::Continue)
            }
            Instruction::GeneratorThrow => {
                // TODO: Implement generator throw
                Ok(ExecutionResult::Continue)
            }
            
            // Async operations
            Instruction::AsyncFunctionStart => {
                // TODO: Implement async function start
                Ok(ExecutionResult::Continue)
            }
            Instruction::AsyncFunctionEnd => {
                // TODO: Implement async function end
                Ok(ExecutionResult::Continue)
            }
            
            // Optimization hints
            Instruction::Optimize => {
                // TODO: Implement optimization hint
                Ok(ExecutionResult::Continue)
            }
            Instruction::Deoptimize => {
                // TODO: Implement deoptimization hint
                Ok(ExecutionResult::Continue)
            }
            
            // Memory management
            Instruction::Allocate(size) => {
                // TODO: Implement memory allocation
                Ok(ExecutionResult::Continue)
            }
            Instruction::Free => {
                // TODO: Implement memory deallocation
                Ok(ExecutionResult::Continue)
            }
            Instruction::GcHint => {
                // TODO: Implement GC hint
                Ok(ExecutionResult::Continue)
            }
        }
    }
    
    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }
    
    pub fn get_stack(&self) -> &Stack {
        &self.stack
    }
    
    pub fn get_registers(&self) -> &Registers {
        &self.registers
    }
    
    pub fn get_current_frame(&self) -> Option<&Frame> {
        self.current_frame.and_then(|index| self.frames.get(index))
    }
    
    pub fn get_call_stack(&self) -> &[usize] {
        &self.call_stack
    }
}

#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Continue,
    Jump(usize),
    Return(Value),
    Call(usize), // frame index
} 