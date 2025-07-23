//! Object tracking for garbage collection

use std::collections::HashMap;

/// Tracks objects for garbage collection
pub struct ObjectTracker {
    objects: HashMap<usize, ObjectInfo>,
    next_id: usize,
}

#[derive(Debug, Clone)]
struct ObjectInfo {
    size: usize,
    references: Vec<usize>,
}

impl ObjectTracker {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn track_object(&mut self, size: usize) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        self.objects.insert(id, ObjectInfo {
            size,
            references: Vec::new(),
        });
        
        id
    }

    pub fn add_reference(&mut self, object_id: usize, reference_id: usize) {
        if let Some(info) = self.objects.get_mut(&object_id) {
            info.references.push(reference_id);
        }
    }

    pub fn get_references(&self, object_id: usize) -> Option<&[usize]> {
        self.objects.get(&object_id).map(|info| info.references.as_slice())
    }
}

impl Default for ObjectTracker {
    fn default() -> Self {
        Self::new()
    }
} 