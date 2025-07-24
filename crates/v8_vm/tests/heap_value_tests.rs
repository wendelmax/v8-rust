use v8_vm::value::Value;
use v8_vm::heap::{Heap, HeapEntry};

#[test]
fn test_value_creation() {
    let num = Value::Number(42.0);
    let str_val = Value::String("hello".to_string());
    let bool_val = Value::Boolean(true);
    let null_val = Value::Null;
    let undefined_val = Value::Undefined;

    assert!(num.is_primitive());
    assert!(str_val.is_primitive());
    assert!(bool_val.is_primitive());
    assert!(null_val.is_primitive());
    assert!(undefined_val.is_primitive());
}

#[test]
fn test_value_conversion() {
    let num = Value::Number(42.0);
    let str_val = Value::String("hello".to_string());
    let bool_val = Value::Boolean(true);

    assert_eq!(num.as_number(), Some(42.0));
    assert_eq!(str_val.as_string(), Some("hello"));
    assert_eq!(bool_val.as_bool(), Some(true));
}

#[test]
fn test_heap_allocation() {
    let mut heap = Heap::new();
    
    let obj_handle = heap.alloc_object();
    let arr_handle = heap.alloc_array();
    
    assert_eq!(obj_handle, 0);
    assert_eq!(arr_handle, 1);
}

#[test]
fn test_heap_access() {
    let mut heap = Heap::new();
    let obj_handle = heap.alloc_object();
    
    if let Some(HeapEntry::Object(obj)) = heap.get_mut(obj_handle) {
        obj.insert("key".to_string(), Value::String("value".to_string()));
    }
    
    if let Some(HeapEntry::Object(obj)) = heap.get(obj_handle) {
        assert_eq!(obj.get("key"), Some(&Value::String("value".to_string())));
    }
}

#[test]
fn test_value_equality() {
    let num1 = Value::Number(42.0);
    let num2 = Value::Number(42.0);
    let num3 = Value::Number(43.0);
    
    assert_eq!(num1, num2);
    assert_ne!(num1, num3);
    
    let str1 = Value::String("hello".to_string());
    let str2 = Value::String("hello".to_string());
    let str3 = Value::String("world".to_string());
    
    assert_eq!(str1, str2);
    assert_ne!(str1, str3);
} 