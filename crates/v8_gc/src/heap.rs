//! Heap management for garbage collection

/// Heap management interface
pub struct Heap {
    size: usize,
    max_size: usize,
}

impl Heap {
    pub fn new(max_size: usize) -> Self {
        Self {
            size: 0,
            max_size,
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if self.size + size <= self.max_size {
            let addr = self.size;
            self.size += size;
            Some(addr)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }
} 