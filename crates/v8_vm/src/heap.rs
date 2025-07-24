//! Heap for the V8-Rust VM

use std::collections::HashMap;
use crate::value::Value;
use crate::bytecode::Bytecode;

#[derive(Debug, Clone)]
pub enum HeapEntry {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Function {
        bytecode: Bytecode, // Bytecode real da função
        arg_count: usize,
        local_count: usize,
        closure_vars: HashMap<String, Value>,
    },
    String(String),
}

#[derive(Debug, Default)]
pub struct Heap {
    entries: Vec<HeapEntry>,
}

impl Heap {
    pub fn new() -> Self {
        Heap { entries: Vec::new() }
    }
    pub fn alloc_object(&mut self) -> usize {
        let idx = self.entries.len();
        self.entries.push(HeapEntry::Object(HashMap::new()));
        idx
    }
    pub fn alloc_array(&mut self) -> usize {
        let idx = self.entries.len();
        self.entries.push(HeapEntry::Array(Vec::new()));
        idx
    }
    pub fn alloc_function(&mut self, bytecode: Bytecode, arg_count: usize, local_count: usize) -> usize {
        let idx = self.entries.len();
        self.entries.push(HeapEntry::Function {
            bytecode,
            arg_count,
            local_count,
            closure_vars: HashMap::new(),
        });
        idx
    }
    pub fn get_function_info(&self, handle: usize) -> Option<(&Bytecode, &usize, &usize, &HashMap<String, Value>)> {
        if let Some(HeapEntry::Function { bytecode, arg_count, local_count, closure_vars }) = self.entries.get(handle) {
            Some((bytecode, arg_count, local_count, closure_vars))
        } else {
            None
        }
    }
    pub fn set_closure_var(&mut self, handle: usize, name: String, value: Value) {
        if let Some(HeapEntry::Function { closure_vars, .. }) = self.entries.get_mut(handle) {
            closure_vars.insert(name, value);
        }
    }
    pub fn get(&self, handle: usize) -> Option<&HeapEntry> {
        self.entries.get(handle)
    }
    pub fn get_mut(&mut self, handle: usize) -> Option<&mut HeapEntry> {
        self.entries.get_mut(handle)
    }
    pub fn set_object_property(&mut self, handle: usize, key: String, value: Value) {
        if let Some(HeapEntry::Object(obj)) = self.entries.get_mut(handle) {
            obj.insert(key, value);
        }
    }
    pub fn get_object_property(&self, handle: usize, key: &str) -> Option<&Value> {
        if let Some(HeapEntry::Object(obj)) = self.entries.get(handle) {
            obj.get(key)
        } else {
            None
        }
    }
    pub fn push_array_element(&mut self, handle: usize, value: Value) {
        if let Some(HeapEntry::Array(arr)) = self.entries.get_mut(handle) {
            arr.push(value);
        }
    }
    pub fn get_array_element(&self, handle: usize, idx: usize) -> Option<&Value> {
        if let Some(HeapEntry::Array(arr)) = self.entries.get(handle) {
            arr.get(idx)
        } else {
            None
        }
    }
    pub fn set_array_element(&mut self, handle: usize, idx: usize, value: Value) {
        if let Some(HeapEntry::Array(arr)) = self.entries.get_mut(handle) {
            if idx < arr.len() {
                arr[idx] = value;
            } else {
                // Expande o array se necessário
                arr.resize(idx + 1, Value::Undefined);
                arr[idx] = value;
            }
        }
    }
    pub fn remove_object_property(&mut self, handle: usize, key: &str) {
        if let Some(HeapEntry::Object(obj)) = self.entries.get_mut(handle) {
            obj.remove(key);
        }
    }
    pub fn has_object_property(&self, handle: usize, key: &str) -> bool {
        if let Some(HeapEntry::Object(obj)) = self.entries.get(handle) {
            obj.contains_key(key)
        } else {
            false
        }
    }
} 