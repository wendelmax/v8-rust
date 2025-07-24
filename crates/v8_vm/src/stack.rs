//! Stack for the V8-Rust VM

use crate::frame::Frame;
use crate::value::Value;

#[derive(Debug, Default)]
pub struct Stack {
    pub values: Vec<Value>,
    pub frames: Vec<Frame>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            values: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_pop_dup() {
        let mut stack = Stack::new();
        stack.push(Value::Number(10.0));
        stack.push(Value::Number(20.0));
        assert_eq!(stack.pop(), Some(Value::Number(20.0)));
        stack.push(Value::Number(30.0));
        stack.push(Value::Number(40.0));
        if let Some(top) = stack.values.last().cloned() {
            stack.push(top);
        }
        assert_eq!(stack.pop(), Some(Value::Number(40.0)));
        assert_eq!(stack.pop(), Some(Value::Number(40.0)));
        assert_eq!(stack.pop(), Some(Value::Number(30.0)));
        assert_eq!(stack.pop(), Some(Value::Number(10.0)));
    }
} 