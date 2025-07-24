use v8_vm::value::Value;
use v8_vm::heap::Heap;

#[test]
fn test_value_conversions() {
    let n = Value::Number(42.5);
    let s = Value::String("123.4".to_string());
    let b = Value::Boolean(true);
    let null = Value::Null;
    let undef = Value::Undefined;

    assert_eq!(n.to_number(), 42.5);
    assert_eq!(s.to_number(), 123.4);
    assert_eq!(b.to_number(), 1.0);
    assert_eq!(null.to_number(), 0.0);
    assert!(undef.to_number().is_nan());

    assert_eq!(n.to_string(), "42.5");
    assert_eq!(b.to_string(), "true");
    assert_eq!(null.to_string(), "null");
    assert_eq!(undef.to_string(), "undefined");

    assert!(n.to_boolean());
    assert!(b.to_boolean());
    assert!(!Value::Number(0.0).to_boolean());
    assert!(!Value::String("".to_string()).to_boolean());
    assert!(!null.to_boolean());
    assert!(!undef.to_boolean());
}

#[test]
fn test_array_set_get_by_index() {
    let mut heap = Heap::new();
    let arr_handle = heap.alloc_array();
    heap.set_array_element(arr_handle, 0, Value::Number(10.0));
    heap.set_array_element(arr_handle, 2, Value::String("abc".to_string()));
    assert_eq!(heap.get_array_element(arr_handle, 0), Some(&Value::Number(10.0)));
    assert_eq!(heap.get_array_element(arr_handle, 1), Some(&Value::Undefined));
    assert_eq!(heap.get_array_element(arr_handle, 2), Some(&Value::String("abc".to_string())));
}

#[test]
fn test_object_remove_and_has_property() {
    let mut heap = Heap::new();
    let obj_handle = heap.alloc_object();
    heap.set_object_property(obj_handle, "foo".to_string(), Value::Number(1.0));
    heap.set_object_property(obj_handle, "bar".to_string(), Value::Number(2.0));
    assert!(heap.has_object_property(obj_handle, "foo"));
    heap.remove_object_property(obj_handle, "foo");
    assert!(!heap.has_object_property(obj_handle, "foo"));
    assert!(heap.has_object_property(obj_handle, "bar"));
} 