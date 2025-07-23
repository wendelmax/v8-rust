//! Garbage Collection for V8-Rust JavaScript engine
//! 
//! This crate provides memory management and garbage collection
//! for the JavaScript engine.

// Placeholder modules - to be implemented
pub mod collector;
pub mod heap;
pub mod mark_sweep;
pub mod object_tracker;

// Re-export main types
pub use collector::Collector;
pub use heap::Heap;
pub use mark_sweep::MarkSweepCollector;
pub use object_tracker::ObjectTracker; 