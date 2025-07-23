// Stack system for ECMAScript VM

use crate::runtime::Value;

#[derive(Debug, Clone)]
pub struct Stack {
    pub values: Vec<Value>,
    pub max_size: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            max_size: 10000, // Default max stack size
        }
    }
    
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            values: Vec::new(),
            max_size,
        }
    }
    
    pub fn push(&mut self, value: Value) -> Result<(), String> {
        if self.values.len() >= self.max_size {
            return Err("Stack overflow".to_string());
        }
        self.values.push(value);
        Ok(())
    }
    
    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }
    
    pub fn peek(&self) -> Option<&Value> {
        self.values.last()
    }
    
    pub fn peek_mut(&mut self) -> Option<&mut Value> {
        self.values.last_mut()
    }
    
    pub fn peek_at(&self, index: usize) -> Option<&Value> {
        if index < self.values.len() {
            self.values.get(self.values.len() - 1 - index)
        } else {
            None
        }
    }
    
    pub fn peek_at_mut(&mut self, index: usize) -> Option<&mut Value> {
        if index < self.values.len() {
            let len = self.values.len();
            self.values.get_mut(len - 1 - index)
        } else {
            None
        }
    }
    
    pub fn swap(&mut self) -> Result<(), String> {
        if self.values.len() < 2 {
            return Err("Not enough values on stack for swap".to_string());
        }
        let len = self.values.len();
        self.values.swap(len - 1, len - 2);
        Ok(())
    }
    
    pub fn dup(&mut self) -> Result<(), String> {
        if self.values.is_empty() {
            return Err("Cannot duplicate empty stack".to_string());
        }
        let value = self.values.last().unwrap().clone();
        self.push(value)
    }
    
    pub fn dup_at(&mut self, index: usize) -> Result<(), String> {
        if let Some(value) = self.peek_at(index) {
            self.push(value.clone())
        } else {
            Err(format!("Invalid stack index: {}", index))
        }
    }
    
    pub fn drop(&mut self, count: usize) -> Result<(), String> {
        if count > self.values.len() {
            return Err(format!("Cannot drop {} values from stack of size {}", count, self.values.len()));
        }
        for _ in 0..count {
            self.values.pop();
        }
        Ok(())
    }
    
    pub fn clear(&mut self) {
        self.values.clear();
    }
    
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    
    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }
    
    pub fn reserve(&mut self, additional: usize) {
        self.values.reserve(additional);
    }
    
    pub fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
    }
    
    pub fn get_max_size(&self) -> usize {
        self.max_size
    }
    
    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
    }
    
    pub fn as_slice(&self) -> &[Value] {
        &self.values
    }
    
    pub fn as_slice_mut(&mut self) -> &mut [Value] {
        &mut self.values
    }
    
    pub fn iter(&self) -> std::slice::Iter<Value> {
        self.values.iter()
    }
    
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Value> {
        self.values.iter_mut()
    }
    
    pub fn into_iter(self) -> std::vec::IntoIter<Value> {
        self.values.into_iter()
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", value)?;
        }
        write!(f, "]")
    }
}

impl std::ops::Index<usize> for Stack {
    type Output = Value;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl std::ops::IndexMut<usize> for Stack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl IntoIterator for Stack {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a> IntoIterator for &'a Stack {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a> IntoIterator for &'a mut Stack {
    type Item = &'a mut Value;
    type IntoIter = std::slice::IterMut<'a, Value>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stack_creation() {
        let stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }
    
    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        
        stack.push(Value::Number(42.0)).unwrap();
        assert_eq!(stack.len(), 1);
        
        let value = stack.pop().unwrap();
        assert_eq!(value, Value::Number(42.0));
        assert!(stack.is_empty());
    }
    
    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        
        stack.push(Value::String("hello".to_string())).unwrap();
        assert_eq!(stack.peek().unwrap(), &Value::String("hello".to_string()));
        assert_eq!(stack.len(), 1);
    }
    
    #[test]
    fn test_stack_swap() {
        let mut stack = Stack::new();
        
        stack.push(Value::Number(1.0)).unwrap();
        stack.push(Value::Number(2.0)).unwrap();
        
        stack.swap().unwrap();
        
        assert_eq!(stack.pop().unwrap(), Value::Number(1.0));
        assert_eq!(stack.pop().unwrap(), Value::Number(2.0));
    }
    
    #[test]
    fn test_stack_dup() {
        let mut stack = Stack::new();
        
        stack.push(Value::Boolean(true)).unwrap();
        stack.dup().unwrap();
        
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop().unwrap(), Value::Boolean(true));
        assert_eq!(stack.pop().unwrap(), Value::Boolean(true));
    }
    
    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::with_max_size(2);
        
        stack.push(Value::Number(1.0)).unwrap();
        stack.push(Value::Number(2.0)).unwrap();
        
        assert!(stack.push(Value::Number(3.0)).is_err());
    }
    
    #[test]
    fn test_stack_drop() {
        let mut stack = Stack::new();
        
        stack.push(Value::Number(1.0)).unwrap();
        stack.push(Value::Number(2.0)).unwrap();
        stack.push(Value::Number(3.0)).unwrap();
        
        stack.drop(2).unwrap();
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.pop().unwrap(), Value::Number(1.0));
    }
    
    #[test]
    fn test_stack_clear() {
        let mut stack = Stack::new();
        
        stack.push(Value::Number(1.0)).unwrap();
        stack.push(Value::Number(2.0)).unwrap();
        
        stack.clear();
        assert!(stack.is_empty());
    }
    
    #[test]
    fn test_stack_iteration() {
        let mut stack = Stack::new();
        
        stack.push(Value::Number(1.0)).unwrap();
        stack.push(Value::Number(2.0)).unwrap();
        stack.push(Value::Number(3.0)).unwrap();
        
        let values: Vec<Value> = stack.iter().cloned().collect();
        assert_eq!(values, vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
    }
} 