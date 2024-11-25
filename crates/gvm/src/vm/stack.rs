use broom::{trace::Trace as _, Handle};

use super::heap::HeapItem;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum StackItem {
    Int(i32),
    Float(f32),
    Char(char),
    Bool(bool),
    Heap(Handle<HeapItem>),
}

impl StackItem {
    pub fn trace(&self, tracer: &mut broom::prelude::Tracer<HeapItem>) {
        if let StackItem::Heap(handle) = self {
            handle.trace(tracer);
        }
    }
}
