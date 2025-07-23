//! Garbage Collector implementation

use std::collections::HashMap;

/// Main garbage collector interface
pub struct Collector {
    heap_size: usize,
    objects: HashMap<usize, bool>, // object_id -> is_marked
}

impl Collector {
    pub fn new() -> Self {
        Self {
            heap_size: 0,
            objects: HashMap::new(),
        }
    }

    pub fn collect(&mut self) {
        // TODO: Implement mark and sweep
    }

    pub fn allocate(&mut self, size: usize) -> usize {
        // TODO: Implement allocation
        self.heap_size += size;
        self.heap_size
    }
}

impl Default for Collector {
    fn default() -> Self {
        Self::new()
    }
} 