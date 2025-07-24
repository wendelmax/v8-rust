use v8_vm::value::Value;
use v8_vm::heap::Heap;

#[test]
fn test_function_allocation() {
    let mut heap = Heap::new();
    let func_handle = heap.alloc_function(2, 3); // 2 args, 3 locals
    assert_eq!(func_handle, 0);
    
    if let Some((arg_count, local_count, closure_vars)) = heap.get_function_info(func_handle) {
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
    let func_handle = heap.alloc_function(1, 2);
    
    heap.set_closure_var(func_handle, "x".to_string(), Value::Number(42.0));
    heap.set_closure_var(func_handle, "y".to_string(), Value::String("hello".to_string()));
    
    if let Some((_, _, closure_vars)) = heap.get_function_info(func_handle) {
        assert_eq!(closure_vars.get("x"), Some(&Value::Number(42.0)));
        assert_eq!(closure_vars.get("y"), Some(&Value::String("hello".to_string())));
        assert_eq!(closure_vars.len(), 2);
    } else {
        panic!("Função não encontrada no heap");
    }
}

#[test]
fn test_function_value_creation() {
    let func_value = Value::Function(0); // handle 0
    assert!(!func_value.is_primitive());
    assert_eq!(func_value.to_string(), "[function]");
    assert!(func_value.to_boolean()); // funções são truthy
} 