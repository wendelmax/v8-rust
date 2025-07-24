use v8_vm::instructions::Instruction;
use v8_vm::bytecode::Bytecode;
use v8_vm::executor::Executor;
use v8_vm::frame::Frame;

#[test]
fn test_execute_basic_arithmetic() {
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0), // 2
        Instruction::PushConst(1), // 3
        Instruction::Add,          // 2 + 3 = 5
        Instruction::Dup,          // 5, 5
        Instruction::PushConst(2), // 4
        Instruction::Mul,          // 5 * 4 = 20
        Instruction::Pop,          // remove 20, resta 5
    ]);
    let constants = vec![2, 3, 4];
    let mut exec = Executor::new();
    exec.execute(&bytecode, &constants);
    // Após Pop, deve restar 5 na stack
    assert_eq!(exec.stack.values, vec![5]);
}

#[test]
fn test_execute_sub_mul_div() {
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0), // 10
        Instruction::PushConst(1), // 2
        Instruction::Sub,          // 10 - 2 = 8
        Instruction::PushConst(2), // 4
        Instruction::Mul,          // 8 * 4 = 32
        Instruction::PushConst(3), // 8
        Instruction::Div,          // 32 / 8 = 4
    ]);
    let constants = vec![10, 2, 4, 8];
    let mut exec = Executor::new();
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![4]);
}

#[test]
fn test_execute_pop_dup() {
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0), // 5
        Instruction::Dup,          // 5, 5
        Instruction::Pop,          // 5
    ]);
    let constants = vec![5];
    let mut exec = Executor::new();
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![5]);
}

#[test]
fn test_execute_load_store_local() {
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0), // 42
        Instruction::StoreLocal(2),
        Instruction::LoadLocal(2),
    ]);
    let constants = vec![42];
    let mut exec = Executor::new();
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![42]);
}

#[test]
fn test_stack_push_pop_frame() {
    let mut stack = v8_vm::stack::Stack::new();
    let frame1 = Frame::new();
    let frame2 = Frame::new();
    stack.push_frame(frame1);
    stack.push_frame(frame2);
    assert!(stack.pop_frame().is_some());
    assert!(stack.pop_frame().is_some());
    assert!(stack.pop_frame().is_none());
} 