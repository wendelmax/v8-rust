use v8_vm::executor::Executor;
use v8_vm::bytecode::Bytecode;
use v8_vm::instructions::Instruction;
use v8_vm::value::Value;

#[test]
fn test_execute_basic_arithmetic() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0),
            Instruction::PushConst(1),
            Instruction::Add,
        ],
    };
    let constants = vec![Value::Number(3.0), Value::Number(2.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(5.0)]);
}

#[test]
fn test_execute_sub_mul_div() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // 10
            Instruction::PushConst(1), // 3
            Instruction::Sub,          // 10 - 3 = 7
            Instruction::PushConst(2), // 2
            Instruction::Mul,          // 7 * 2 = 14
            Instruction::PushConst(3), // 7
            Instruction::Div,          // 14 / 7 = 2
        ],
    };
    let constants = vec![Value::Number(10.0), Value::Number(3.0), Value::Number(2.0), Value::Number(7.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(2.0)]);
}

#[test]
fn test_execute_pop_dup() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0),
            Instruction::PushConst(1),
            Instruction::Dup,
            Instruction::Pop,
        ],
    };
    let constants = vec![Value::Number(42.0), Value::Number(100.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(42.0), Value::Number(100.0)]);
}

#[test]
fn test_execute_load_store_local() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // 42
            Instruction::StoreLocal(0), // Store in local[0]
            Instruction::LoadLocal(0),  // Load from local[0]
            Instruction::PushConst(1), // 10
            Instruction::Add,           // 42 + 10 = 52
        ],
    };
    let constants = vec![Value::Number(42.0), Value::Number(10.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(52.0)]);
}

#[test]
fn test_stack_push_pop_frame() {
    let mut exec = Executor::new();
    exec.stack.push_frame(exec.frame.clone());
    assert_eq!(exec.stack.frames.len(), 1);
    
    let popped_frame = exec.stack.pop_frame();
    assert!(popped_frame.is_some());
    assert_eq!(exec.stack.frames.len(), 0);
}

// Novos testes para Fase 2: Controle de Fluxo e Variáveis

#[test]
fn test_execute_jump() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // 42
            Instruction::Jump(3),      // Jump to instruction 3
            Instruction::PushConst(1), // This should be skipped
            Instruction::PushConst(2), // 100
        ],
    };
    let constants = vec![Value::Number(42.0), Value::Number(999.0), Value::Number(100.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(42.0), Value::Number(100.0)]);
}

#[test]
fn test_execute_jump_if_true() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // 1 (true)
            Instruction::JumpIfTrue(4), // Jump if true
            Instruction::PushConst(1), // This should be skipped
            Instruction::PushConst(2), // This should be skipped
            Instruction::PushConst(3), // 100
        ],
    };
    let constants = vec![Value::Boolean(true), Value::Number(999.0), Value::Number(888.0), Value::Number(100.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(100.0)]);
}

#[test]
fn test_execute_jump_if_false() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // 0 (false)
            Instruction::JumpIfFalse(4), // Jump if false
            Instruction::PushConst(1), // This should be skipped
            Instruction::PushConst(2), // This should be skipped
            Instruction::PushConst(3), // 100
        ],
    };
    let constants = vec![Value::Boolean(false), Value::Number(999.0), Value::Number(888.0), Value::Number(100.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(100.0)]);
}

#[test]
fn test_execute_load_store_global() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // 42
            Instruction::StoreGlobal(0), // Store in global[0]
            Instruction::LoadGlobal(0),  // Load from global[0]
            Instruction::PushConst(1), // 10
            Instruction::Add,           // 42 + 10 = 52
        ],
    };
    let constants = vec![Value::Number(42.0), Value::Number(10.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(52.0)]);
}

#[test]
fn test_execute_simple_function_call() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // 42
            Instruction::Call(0),      // Call function with 0 args
            Instruction::Return,       // Return from function
        ],
    };
    let constants = vec![Value::Number(42.0)];
    exec.execute(&bytecode, &constants);
    // A função deve retornar sem erro
    assert!(exec.stack.values.is_empty());
}

#[test]
fn test_execute_conditional_logic() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // 10
            Instruction::PushConst(1), // 5
            Instruction::Gt,           // 10 > 5 = true (1)
            Instruction::JumpIfTrue(6), // Jump if true
            Instruction::PushConst(2), // This should be skipped
            Instruction::PushConst(3), // This should be skipped
            Instruction::PushConst(4), // 100 (result if condition is true)
        ],
    };
    let constants = vec![Value::Number(10.0), Value::Number(5.0), Value::Number(999.0), Value::Number(888.0), Value::Number(100.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(100.0)]); // apenas o valor final após o jump
} 