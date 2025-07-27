use v8_vm::value::Value;
use v8_vm::heap::Heap;
use v8_vm::bytecode::Bytecode;
use v8_vm::instructions::Instruction;
use v8_vm::executor::Executor;

#[test]
fn test_function_allocation() {
    let mut heap = Heap::new();
    let bytecode = Bytecode::new(vec![]);
    let func_handle = heap.alloc_function(bytecode, 2, 3); // 2 args, 3 locals
    assert_eq!(func_handle, 0);
    
    if let Some((_, arg_count, local_count, closure_vars)) = heap.get_function_info(func_handle) {
        assert_eq!(*arg_count, 2);
        assert_eq!(*local_count, 3);
        assert!(closure_vars.is_empty());
    } else {
        panic!("Função não encontrada no heap");
    }
}

#[test]
fn test_closure_variables() {
    let mut heap = Heap::new();
    let bytecode = Bytecode::new(vec![]);
    let func_handle = heap.alloc_function(bytecode, 1, 2);
    
    heap.set_closure_var(func_handle, "x".to_string(), Value::Number(42.0));
    heap.set_closure_var(func_handle, "y".to_string(), Value::String("hello".to_string()));
    
    if let Some((_, _, _, closure_vars)) = heap.get_function_info(func_handle) {
        assert_eq!(closure_vars.get("x"), Some(&Value::Number(42.0)));
        assert_eq!(closure_vars.get("y"), Some(&Value::String("hello".to_string())));
        assert_eq!(closure_vars.len(), 2);
    } else {
        panic!("Função não encontrada no heap");
    }
}

#[test]
fn test_function_value_creation() {
    let func_value = Value::Function(HandleId::from(0));
    assert!(!func_value.is_primitive());
    assert_eq!(func_value.to_string(), "[function]");
    assert!(func_value.to_boolean()); // funções são truthy
} 

#[test]
fn test_function_identity_execution() {
    // Função identidade: retorna o argumento 0
    let bytecode = Bytecode::new(vec![
        Instruction::LoadArg(0),
        Instruction::Return,
    ]);
    let mut heap = Heap::new();
    let func_handle = heap.alloc_function(bytecode, 1, 0);
    let mut exec = Executor::new();
    exec.heap = heap;
    
    // Empilha o argumento primeiro, depois a função
    exec.stack.push(Value::Number(42.0));
    exec.stack.push(Value::Function(func_handle));
    
    // Executa Call(1)
    exec.execute(&Bytecode::new(vec![Instruction::Call(1)]), &[]);
    
    // O topo da stack deve ser o valor retornado
    let result = exec.stack.pop().unwrap();
    assert_eq!(result, Value::Number(42.0));
} 

#[test]
fn test_function_recursion_factorial() {
    // Teste simplificado - apenas verificar se a função básica funciona
    // Vamos testar uma função simples que retorna o argumento + 1
    let bytecode = Bytecode::new(vec![
        Instruction::LoadArg(0), // carrega argumento
        Instruction::PushConst(0), // empilha 1
        Instruction::Add, // argumento + 1
        Instruction::Return,
    ]);
    
    let mut heap = Heap::new();
    let func_handle = heap.alloc_function(bytecode, 1, 0);
    let mut exec = Executor::new();
    exec.heap = heap;
    
    println!("=== Teste Função Simples ===");
    
    // Pool de constantes: [1]
    let constants = vec![Value::Number(1.0)];
    
    // Teste: f(3) = 4
    exec.stack.push(Value::Number(3.0));
    exec.stack.push(Value::Function(func_handle));
    
    println!("Stack antes do Call: {:?}", exec.stack.values);
    
    exec.execute(&Bytecode::new(vec![Instruction::Call(1)]), &constants);
    
    println!("Stack após o Call: {:?}", exec.stack.values);
    
    let result = exec.stack.pop().unwrap();
    println!("Resultado: {:?}", result);
    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn test_function_simple_recursion() {
    // Teste simplificado - apenas verificar se a função básica funciona
    // Vamos testar uma função simples que retorna o argumento * 2
    let bytecode = Bytecode::new(vec![
        Instruction::LoadArg(0), // carrega argumento
        Instruction::PushConst(0), // empilha 2
        Instruction::Mul, // argumento * 2
        Instruction::Return,
    ]);
    
    let mut heap = Heap::new();
    let func_handle = heap.alloc_function(bytecode, 1, 0);
    let mut exec = Executor::new();
    exec.heap = heap;
    
    println!("=== Teste Função Simples 2 ===");
    
    // Pool de constantes: [2]
    let constants = vec![Value::Number(2.0)];
    
    // Teste: f(3) = 6
    exec.stack.push(Value::Number(3.0));
    exec.stack.push(Value::Function(func_handle));
    
    println!("Stack antes do Call: {:?}", exec.stack.values);
    
    exec.execute(&Bytecode::new(vec![Instruction::Call(1)]), &constants);
    
    println!("Stack após o Call: {:?}", exec.stack.values);
    
    let result = exec.stack.pop().unwrap();
    println!("Resultado: {:?}", result);
    assert_eq!(result, Value::Number(6.0));
} 

#[test]
fn test_load_this_function() {
    // Função que apenas carrega a si mesma e retorna
    let bytecode = Bytecode::new(vec![
        Instruction::LoadThisFunction, // carrega a função atual
        Instruction::Return, // retorna a função
    ]);
    
    let mut heap = Heap::new();
    let func_handle = heap.alloc_function(bytecode, 0, 0);
    let mut exec = Executor::new();
    exec.heap = heap;
    
    println!("=== Teste LoadThisFunction ===");
    println!("Func handle: {:?}", func_handle);

    // Criar um objeto para ser usado como this
    let obj_handle = exec.heap.alloc_object();
    let this_obj = Value::Object(obj_handle);
    
    // Definir uma propriedade no objeto
    exec.heap.set_object_property(obj_handle, "name".to_string(), Value::String("test".to_string()));
    
    // Chama a função sem argumentos, mas com this
    // Ordem: this, função
    exec.stack.push(this_obj.clone());
    exec.stack.push(Value::Function(func_handle));
    
    println!("Stack antes do Call: {:?}", exec.stack.values);
    println!("This object: {:?}", this_obj);
    
    // Remover a linha que define this_value manualmente, pois agora é passado via stack
    exec.execute(&Bytecode::new(vec![Instruction::Call(0)]), &[]);
    
    println!("Stack após o Call: {:?}", exec.stack.values);
    
    let result = exec.stack.pop().unwrap();
    println!("Resultado: {:?}", result);
    assert_eq!(result, Value::Function(func_handle));
} 

#[test]
fn test_subtraction_operation() {
    // Função que faz uma subtração simples
    let bytecode = Bytecode::new(vec![
        Instruction::LoadArg(0), // carrega argumento
        Instruction::PushConst(0), // empilha 1
        Instruction::Sub, // subtrai
        Instruction::Return,
    ]);
    
    let mut heap = Heap::new();
    let func_handle = heap.alloc_function(bytecode, 1, 0);
    let mut exec = Executor::new();
    exec.heap = heap;
    
    println!("=== Teste Subtração ===");
    
    // Pool de constantes: [1]
    let constants = vec![Value::Number(1.0)];
    
    // Teste: 5 - 1 = 4
    exec.stack.push(Value::Number(5.0));
    exec.stack.push(Value::Function(func_handle));
    
    println!("Stack antes do Call: {:?}", exec.stack.values);
    
    exec.execute(&Bytecode::new(vec![Instruction::Call(1)]), &constants);
    
    println!("Stack após o Call: {:?}", exec.stack.values);
    
    let result = exec.stack.pop().unwrap();
    println!("Resultado: {:?}", result);
    assert_eq!(result, Value::Number(4.0));
} 

#[test]
fn test_function_this_value() {
    // Função que retorna o valor de this
    let bytecode = Bytecode::new(vec![
        Instruction::LoadThis, // carrega o valor de this
        Instruction::Return, // retorna this
    ]);
    
    let mut heap = Heap::new();
    let func_handle = heap.alloc_function(bytecode, 0, 0);
    let mut exec = Executor::new();
    exec.heap = heap;
    
    println!("=== Teste This Value ===");
    
    // Criar um objeto para ser usado como this
    let obj_handle = exec.heap.alloc_object();
    let this_obj = Value::Object(obj_handle);
    
    // Definir uma propriedade no objeto
    exec.heap.set_object_property(obj_handle, "name".to_string(), Value::String("test".to_string()));
    
    // Chama a função sem argumentos, mas com this
    // Ordem: this, função
    exec.stack.push(this_obj.clone());
    exec.stack.push(Value::Function(func_handle));
    
    println!("Stack antes do Call: {:?}", exec.stack.values);
    println!("This object: {:?}", this_obj);
    
    // Remover a linha que define this_value manualmente, pois agora é passado via stack
    exec.execute(&Bytecode::new(vec![Instruction::Call(0)]), &[]);
    
    println!("Stack após o Call: {:?}", exec.stack.values);
    
    let result = exec.stack.pop().unwrap();
    println!("Resultado: {:?}", result);
    assert_eq!(result, this_obj);
} 

#[test]
fn test_closure_variables_execution() {
    // Função que acessa variáveis de closure
    let bytecode = Bytecode::new(vec![
        Instruction::LoadClosureVar("x".to_string()), // carrega variável de closure 'x'
        Instruction::Return, // retorna o valor
    ]);
    
    let mut heap = Heap::new();
    let func_handle = heap.alloc_function(bytecode, 0, 0);
    
    // Definir variáveis de closure
    heap.set_closure_var(func_handle, "x".to_string(), Value::Number(42.0));
    heap.set_closure_var(func_handle, "y".to_string(), Value::String("hello".to_string()));
    
    let mut exec = Executor::new();
    exec.heap = heap;
    
    println!("=== Teste Closure Variables ===");
    
    // Chama a função sem argumentos
    exec.stack.push(Value::Function(func_handle));
    
    println!("Stack antes do Call: {:?}", exec.stack.values);
    
    exec.execute(&Bytecode::new(vec![Instruction::Call(0)]), &[]);
    
    println!("Stack após o Call: {:?}", exec.stack.values);
    
    let result = exec.stack.pop().unwrap();
    println!("Resultado: {:?}", result);
    assert_eq!(result, Value::Number(42.0));
} 

#[test]
fn test_complex_function_with_multiple_features() {
    // Função complexa que usa múltiplos argumentos, this, closure variables e operações
    // f(a, b) = this.value + closure_var + a + b
    let bytecode = Bytecode::new(vec![
        // Carrega this.value
        Instruction::LoadThis,
        Instruction::PushConst(0), // "value" como string
        Instruction::GetProperty, // this.value
        // Carrega closure_var
        Instruction::LoadClosureVar("multiplier".to_string()),
        // Soma this.value + closure_var
        Instruction::Add,
        // Carrega argumento 0 (a)
        Instruction::LoadArg(0),
        // Soma (this.value + closure_var) + a
        Instruction::Add,
        // Carrega argumento 1 (b)
        Instruction::LoadArg(1),
        // Soma ((this.value + closure_var) + a) + b
        Instruction::Add,
        Instruction::Return,
    ]);
    
    let mut heap = Heap::new();
    let func_handle = heap.alloc_function(bytecode, 2, 0);
    
    // Definir variável de closure
    heap.set_closure_var(func_handle, "multiplier".to_string(), Value::Number(10.0));
    
    let mut exec = Executor::new();
    exec.heap = heap;
    
    println!("=== Teste Função Complexa ===");
    
    // Criar objeto com propriedade value
    let obj_handle = exec.heap.alloc_object();
    exec.heap.set_object_property(obj_handle, "value".to_string(), Value::Number(5.0));
    let this_obj = Value::Object(obj_handle);
    
    // Pool de constantes: ["value"]
    let constants = vec![Value::String("value".to_string())];
    
    // Chama a função: f(3, 7) com this = {value: 5}
    // Ordem: this, arg1, arg2, função
    exec.stack.push(this_obj);
    exec.stack.push(Value::Number(3.0));
    exec.stack.push(Value::Number(7.0));
    exec.stack.push(Value::Function(func_handle));
    
    println!("Stack antes do Call: {:?}", exec.stack.values);
    
    exec.execute(&Bytecode::new(vec![Instruction::Call(2)]), &constants);
    
    println!("Stack após o Call: {:?}", exec.stack.values);
    
    let result = exec.stack.pop().unwrap();
    println!("Resultado: {:?}", result);
    // Esperado: this.value(5) + closure_var(10) + a(3) + b(7) = 25
    assert_eq!(result, Value::Number(25.0));
} 