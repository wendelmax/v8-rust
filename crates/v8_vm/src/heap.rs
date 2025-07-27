//! Heap for the V8-Rust VM

use crate::bytecode::Bytecode;
use crate::value::Value;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct HandleId(usize);

impl From<usize> for HandleId {
    fn from(value: usize) -> Self {
        HandleId(value)
    }
}

impl From<&usize> for HandleId {
    fn from(value: &usize) -> Self {
        HandleId(*value)
    }
}

impl Deref for HandleId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<usize> for HandleId {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

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
    pub fn alloc_entry(&mut self, entry: HeapEntry) -> HandleId {
        let idx = self.entries.len();
        self.entries.push(entry);
        HandleId(idx)
    }
    pub fn alloc_object(&mut self) -> HandleId {
        self.alloc_entry(HeapEntry::Object(HashMap::new()))
    }
    pub fn alloc_array(&mut self) -> HandleId {
        self.alloc_entry(HeapEntry::Array(Vec::new()))
    }
    pub fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: usize,
        local_count: usize,
    ) -> HandleId {
        self.alloc_entry(HeapEntry::Function {
            bytecode,
            arg_count,
            local_count,
            closure_vars: HashMap::new(),
        })
    }
    pub fn get_function_info(
        &self,
        handle: HandleId,
    ) -> Option<(&Bytecode, &usize, &usize, &HashMap<String, Value>)> {
        if let Some(HeapEntry::Function {
            bytecode,
            arg_count,
            local_count,
            closure_vars,
        }) = self.get(handle)
        {
            Some((bytecode, arg_count, local_count, closure_vars))
        } else {
            None
        }
    }
    pub fn set_closure_var(&mut self, handle: HandleId, name: String, value: Value) {
        if let Some(HeapEntry::Function { closure_vars, .. }) = self.entries.get_mut(*handle) {
            closure_vars.insert(name, value);
        }
    }
    pub fn get(&self, handle: HandleId) -> Option<&HeapEntry> {
        self.entries.get(*handle)
    }
    pub fn get_mut(&mut self, handle: HandleId) -> Option<&mut HeapEntry> {
        self.entries.get_mut(*handle)
    }
    pub fn set_object_property(&mut self, handle: HandleId, key: String, value: Value) {
        if let Some(HeapEntry::Object(obj)) = self.get_mut(handle) {
            obj.insert(key, value);
        }
    }
    pub fn get_object_property(&self, handle: HandleId, key: &str) -> Option<&Value> {
        if let Some(HeapEntry::Object(obj)) = self.get(handle) {
            obj.get(key)
        } else {
            None
        }
    }
    pub fn push_array_element(&mut self, handle: HandleId, value: Value) {
        if let Some(HeapEntry::Array(arr)) = self.get_mut(handle) {
            arr.push(value);
        }
    }
    pub fn get_array_element(&self, handle: HandleId, idx: usize) -> Option<&Value> {
        if let Some(HeapEntry::Array(arr)) = self.get(handle) {
            arr.get(idx)
        } else {
            None
        }
    }
    pub fn set_array_element(&mut self, handle: HandleId, idx: usize, value: Value) {
        if let Some(HeapEntry::Array(arr)) = self.get_mut(handle) {
            if idx < arr.len() {
                arr[idx] = value;
            } else {
                // Expande o array se necessário
                arr.resize(idx + 1, Value::Undefined);
                arr[idx] = value;
            }
        }
    }
    pub fn remove_object_property(&mut self, handle: HandleId, key: &str) {
        if let Some(HeapEntry::Object(obj)) = self.get_mut(handle) {
            obj.remove(key);
        }
    }
    pub fn has_object_property(&self, handle: HandleId, key: &str) -> bool {
        if let Some(HeapEntry::Object(obj)) = self.get(handle) {
            obj.contains_key(key)
        } else {
            false
        }
    }
} 
