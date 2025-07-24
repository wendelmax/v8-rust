//! Stack for the V8-Rust VM

use crate::frame::Frame;

pub struct Stack {
    pub values: Vec<i64>, // Futuramente: tipo genérico para valores JS
    pub frames: Vec<Frame>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { values: Vec::new(), frames: Vec::new() }
    }
    pub fn push(&mut self, value: i64) {
        self.values.push(value);
    }
    pub fn pop(&mut self) -> Option<i64> {
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
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.pop(), Some(20));
        stack.push(30);
        stack.push(40);
        if let Some(top) = stack.values.last().copied() {
            stack.push(top); // dup
        }
        assert_eq!(stack.pop(), Some(40));
        assert_eq!(stack.pop(), Some(40));
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }
} 