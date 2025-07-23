// Register system for ECMAScript VM

use crate::runtime::Value;

#[derive(Debug, Clone)]
pub struct Registers {
    pub locals: Vec<Value>,
    pub temps: Vec<Value>,
    pub max_locals: usize,
    pub max_temps: usize,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            temps: Vec::new(),
            max_locals: 1000,
            max_temps: 100,
        }
    }
    
    pub fn with_capacity(local_capacity: usize, temp_capacity: usize) -> Self {
        Self {
            locals: Vec::with_capacity(local_capacity),
            temps: Vec::with_capacity(temp_capacity),
            max_locals: local_capacity,
            max_temps: temp_capacity,
        }
    }
    
    pub fn get_local(&self, index: usize) -> Option<&Value> {
        self.locals.get(index)
    }
    
    pub fn set_local(&mut self, index: usize, value: Value) -> Result<(), String> {
        if index >= self.max_locals {
            return Err(format!("Local register index {} exceeds maximum {}", index, self.max_locals));
        }
        
        if index >= self.locals.len() {
            // Extend the locals vector with undefined values
            while self.locals.len() <= index {
                self.locals.push(Value::Undefined);
            }
        }
        
        self.locals[index] = value;
        Ok(())
    }
    
    pub fn get_temp(&self, index: usize) -> Option<&Value> {
        self.temps.get(index)
    }
    
    pub fn set_temp(&mut self, index: usize, value: Value) -> Result<(), String> {
        if index >= self.max_temps {
            return Err(format!("Temporary register index {} exceeds maximum {}", index, self.max_temps));
        }
        
        if index >= self.temps.len() {
            // Extend the temps vector with undefined values
            while self.temps.len() <= index {
                self.temps.push(Value::Undefined);
            }
        }
        
        self.temps[index] = value;
        Ok(())
    }
    
    pub fn clear_locals(&mut self) {
        self.locals.clear();
    }
    
    pub fn clear_temps(&mut self) {
        self.temps.clear();
    }
    
    pub fn clear_all(&mut self) {
        self.clear_locals();
        self.clear_temps();
    }
    
    pub fn get_local_count(&self) -> usize {
        self.locals.len()
    }
    
    pub fn get_temp_count(&self) -> usize {
        self.temps.len()
    }
    
    pub fn get_max_locals(&self) -> usize {
        self.max_locals
    }
    
    pub fn get_max_temps(&self) -> usize {
        self.max_temps
    }
    
    pub fn set_max_locals(&mut self, max: usize) {
        self.max_locals = max;
    }
    
    pub fn set_max_temps(&mut self, max: usize) {
        self.max_temps = max;
    }
    
    pub fn reserve_locals(&mut self, additional: usize) {
        self.locals.reserve(additional);
    }
    
    pub fn reserve_temps(&mut self, additional: usize) {
        self.temps.reserve(additional);
    }
    
    pub fn shrink_locals_to_fit(&mut self) {
        self.locals.shrink_to_fit();
    }
    
    pub fn shrink_temps_to_fit(&mut self) {
        self.temps.shrink_to_fit();
    }
    
    pub fn get_local_capacity(&self) -> usize {
        self.locals.capacity()
    }
    
    pub fn get_temp_capacity(&self) -> usize {
        self.temps.capacity()
    }
    
    pub fn locals_as_slice(&self) -> &[Value] {
        &self.locals
    }
    
    pub fn temps_as_slice(&self) -> &[Value] {
        &self.temps
    }
    
    pub fn locals_as_slice_mut(&mut self) -> &mut [Value] {
        &mut self.locals
    }
    
    pub fn temps_as_slice_mut(&mut self) -> &mut [Value] {
        &mut self.temps
    }
    
    pub fn copy_local_to_temp(&mut self, local_index: usize, temp_index: usize) -> Result<(), String> {
        let value = self.get_local(local_index)
            .ok_or(format!("Local register {} not found", local_index))?
            .clone();
        self.set_temp(temp_index, value)
    }
    
    pub fn copy_temp_to_local(&mut self, temp_index: usize, local_index: usize) -> Result<(), String> {
        let value = self.get_temp(temp_index)
            .ok_or(format!("Temporary register {} not found", temp_index))?
            .clone();
        self.set_local(local_index, value)
    }
    
    pub fn swap_locals(&mut self, index1: usize, index2: usize) -> Result<(), String> {
        if index1 >= self.locals.len() || index2 >= self.locals.len() {
            return Err("Invalid local register index for swap".to_string());
        }
        self.locals.swap(index1, index2);
        Ok(())
    }
    
    pub fn swap_temps(&mut self, index1: usize, index2: usize) -> Result<(), String> {
        if index1 >= self.temps.len() || index2 >= self.temps.len() {
            return Err("Invalid temporary register index for swap".to_string());
        }
        self.temps.swap(index1, index2);
        Ok(())
    }
    
    pub fn move_local(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from >= self.locals.len() {
            return Err(format!("Source local register {} not found", from));
        }
        let value = self.locals[from].clone();
        self.set_local(to, value)
    }
    
    pub fn move_temp(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from >= self.temps.len() {
            return Err(format!("Source temporary register {} not found", from));
        }
        let value = self.temps[from].clone();
        self.set_temp(to, value)
    }
    
    pub fn is_local_defined(&self, index: usize) -> bool {
        if let Some(value) = self.get_local(index) {
            !value.is_undefined()
        } else {
            false
        }
    }
    
    pub fn is_temp_defined(&self, index: usize) -> bool {
        if let Some(value) = self.get_temp(index) {
            !value.is_undefined()
        } else {
            false
        }
    }
    
    pub fn get_defined_local_count(&self) -> usize {
        self.locals.iter().filter(|v| !v.is_undefined()).count()
    }
    
    pub fn get_defined_temp_count(&self) -> usize {
        self.temps.iter().filter(|v| !v.is_undefined()).count()
    }
    
    pub fn get_stats(&self) -> RegisterStats {
        RegisterStats {
            local_count: self.get_local_count(),
            temp_count: self.get_temp_count(),
            defined_local_count: self.get_defined_local_count(),
            defined_temp_count: self.get_defined_temp_count(),
            local_capacity: self.get_local_capacity(),
            temp_capacity: self.get_temp_capacity(),
            max_locals: self.get_max_locals(),
            max_temps: self.get_max_temps(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterStats {
    pub local_count: usize,
    pub temp_count: usize,
    pub defined_local_count: usize,
    pub defined_temp_count: usize,
    pub local_capacity: usize,
    pub temp_capacity: usize,
    pub max_locals: usize,
    pub max_temps: usize,
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Registers {{ locals: {}, temps: {} }}", 
            self.get_local_count(), self.get_temp_count())
    }
}

impl std::ops::Index<usize> for Registers {
    type Output = Value;
    
    fn index(&self, index: usize) -> &Self::Output {
        self.get_local(index).expect("Local register index out of bounds")
    }
}

impl std::ops::IndexMut<usize> for Registers {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.locals.len() {
            // Extend the locals vector with undefined values
            while self.locals.len() <= index {
                self.locals.push(Value::Undefined);
            }
        }
        &mut self.locals[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registers_creation() {
        let registers = Registers::new();
        assert_eq!(registers.get_local_count(), 0);
        assert_eq!(registers.get_temp_count(), 0);
    }
    
    #[test]
    fn test_registers_with_capacity() {
        let registers = Registers::with_capacity(10, 5);
        assert_eq!(registers.get_max_locals(), 10);
        assert_eq!(registers.get_max_temps(), 5);
    }
    
    #[test]
    fn test_set_get_local() {
        let mut registers = Registers::new();
        
        registers.set_local(0, Value::Number(42.0)).unwrap();
        assert_eq!(registers.get_local(0).unwrap(), &Value::Number(42.0));
        assert_eq!(registers.get_local_count(), 1);
    }
    
    #[test]
    fn test_set_get_temp() {
        let mut registers = Registers::new();
        
        registers.set_temp(0, Value::String("hello".to_string())).unwrap();
        assert_eq!(registers.get_temp(0).unwrap(), &Value::String("hello".to_string()));
        assert_eq!(registers.get_temp_count(), 1);
    }
    
    #[test]
    fn test_auto_extend_locals() {
        let mut registers = Registers::new();
        
        registers.set_local(5, Value::Boolean(true)).unwrap();
        assert_eq!(registers.get_local_count(), 6);
        assert_eq!(registers.get_local(5).unwrap(), &Value::Boolean(true));
        
        // Check that intermediate registers are undefined
        for i in 0..5 {
            assert_eq!(registers.get_local(i).unwrap(), &Value::Undefined);
        }
    }
    
    #[test]
    fn test_auto_extend_temps() {
        let mut registers = Registers::new();
        
        registers.set_temp(3, Value::Null).unwrap();
        assert_eq!(registers.get_temp_count(), 4);
        assert_eq!(registers.get_temp(3).unwrap(), &Value::Null);
        
        // Check that intermediate registers are undefined
        for i in 0..3 {
            assert_eq!(registers.get_temp(i).unwrap(), &Value::Undefined);
        }
    }
    
    #[test]
    fn test_clear_locals() {
        let mut registers = Registers::new();
        
        registers.set_local(0, Value::Number(1.0)).unwrap();
        registers.set_local(1, Value::Number(2.0)).unwrap();
        
        registers.clear_locals();
        assert_eq!(registers.get_local_count(), 0);
    }
    
    #[test]
    fn test_clear_temps() {
        let mut registers = Registers::new();
        
        registers.set_temp(0, Value::String("a".to_string())).unwrap();
        registers.set_temp(1, Value::String("b".to_string())).unwrap();
        
        registers.clear_temps();
        assert_eq!(registers.get_temp_count(), 0);
    }
    
    #[test]
    fn test_clear_all() {
        let mut registers = Registers::new();
        
        registers.set_local(0, Value::Number(1.0)).unwrap();
        registers.set_temp(0, Value::String("a".to_string())).unwrap();
        
        registers.clear_all();
        assert_eq!(registers.get_local_count(), 0);
        assert_eq!(registers.get_temp_count(), 0);
    }
    
    #[test]
    fn test_copy_local_to_temp() {
        let mut registers = Registers::new();
        
        registers.set_local(0, Value::Number(42.0)).unwrap();
        registers.copy_local_to_temp(0, 1).unwrap();
        
        assert_eq!(registers.get_temp(1).unwrap(), &Value::Number(42.0));
    }
    
    #[test]
    fn test_copy_temp_to_local() {
        let mut registers = Registers::new();
        
        registers.set_temp(0, Value::String("hello".to_string())).unwrap();
        registers.copy_temp_to_local(0, 1).unwrap();
        
        assert_eq!(registers.get_local(1).unwrap(), &Value::String("hello".to_string()));
    }
    
    #[test]
    fn test_swap_locals() {
        let mut registers = Registers::new();
        
        registers.set_local(0, Value::Number(1.0)).unwrap();
        registers.set_local(1, Value::Number(2.0)).unwrap();
        
        registers.swap_locals(0, 1).unwrap();
        
        assert_eq!(registers.get_local(0).unwrap(), &Value::Number(2.0));
        assert_eq!(registers.get_local(1).unwrap(), &Value::Number(1.0));
    }
    
    #[test]
    fn test_swap_temps() {
        let mut registers = Registers::new();
        
        registers.set_temp(0, Value::Boolean(true)).unwrap();
        registers.set_temp(1, Value::Boolean(false)).unwrap();
        
        registers.swap_temps(0, 1).unwrap();
        
        assert_eq!(registers.get_temp(0).unwrap(), &Value::Boolean(false));
        assert_eq!(registers.get_temp(1).unwrap(), &Value::Boolean(true));
    }
    
    #[test]
    fn test_is_defined() {
        let mut registers = Registers::new();
        
        registers.set_local(0, Value::Number(42.0)).unwrap();
        registers.set_local(1, Value::Undefined).unwrap();
        
        assert!(registers.is_local_defined(0));
        assert!(!registers.is_local_defined(1));
        assert!(!registers.is_local_defined(2));
    }
    
    #[test]
    fn test_get_defined_count() {
        let mut registers = Registers::new();
        
        registers.set_local(0, Value::Number(1.0)).unwrap();
        registers.set_local(1, Value::Undefined).unwrap();
        registers.set_local(2, Value::String("hello".to_string())).unwrap();
        
        assert_eq!(registers.get_defined_local_count(), 2);
    }
    
    #[test]
    fn test_index_operator() {
        let mut registers = Registers::new();
        
        registers.set_local(0, Value::Number(42.0)).unwrap();
        assert_eq!(registers[0], Value::Number(42.0));
        
        registers[1] = Value::String("hello".to_string());
        assert_eq!(registers.get_local(1).unwrap(), &Value::String("hello".to_string()));
    }
    
    #[test]
    fn test_register_stats() {
        let mut registers = Registers::new();
        
        registers.set_local(0, Value::Number(1.0)).unwrap();
        registers.set_local(1, Value::Undefined).unwrap();
        registers.set_temp(0, Value::String("hello".to_string())).unwrap();
        
        let stats = registers.get_stats();
        assert_eq!(stats.local_count, 2);
        assert_eq!(stats.temp_count, 1);
        assert_eq!(stats.defined_local_count, 1);
        assert_eq!(stats.defined_temp_count, 1);
    }
} 