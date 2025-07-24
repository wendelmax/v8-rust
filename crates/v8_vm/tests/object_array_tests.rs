use v8_vm::executor::Executor;
use v8_vm::bytecode::Bytecode;
use v8_vm::instructions::Instruction;
use v8_vm::value::Value;

#[test]
fn test_new_object_and_set_get_property() {
    let mut exec = Executor::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::NewObject, // empilha objeto
            Instruction::Dup, // duplica objeto para manter referência
            Instruction::PushConst(0), // empilha chave "foo"
            Instruction::PushConst(1), // empilha valor 123
            Instruction::SetProperty, // obj["foo"] = 123
            Instruction::PushConst(0), // empilha chave "foo"
            Instruction::GetProperty, // obj["foo"]
        ],
    };
    let constants = vec![Value::String("foo".to_string()), Value::Number(123.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values.last(), Some(&Value::Number(123.0)));
}

#[test]
fn test_new_array_and_push_get_element() {
    use v8_vm::heap::HeapEntry;
    let mut exec = Executor::new();
    // Cria array e adiciona dois elementos
    let arr_handle = exec.heap.alloc_array();
    exec.heap.push_array_element(arr_handle, Value::Number(10.0));
    exec.heap.push_array_element(arr_handle, Value::String("abc".to_string()));
    // Verifica elementos
    if let Some(HeapEntry::Array(arr)) = exec.heap.get(arr_handle) {
        assert_eq!(arr[0], Value::Number(10.0));
        assert_eq!(arr[1], Value::String("abc".to_string()));
    } else {
        panic!("Array não encontrado no heap");
    }
} 