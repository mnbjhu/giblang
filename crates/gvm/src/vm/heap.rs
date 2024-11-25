use broom::trace::Trace;

use super::stack::StackItem;

#[derive(PartialEq, Debug, Clone)]
pub enum HeapItem {
    Object(u32, Vec<StackItem>),
    String(String),
    Dyn(u64, StackItem),
}

impl Trace<Self> for HeapItem {
    fn trace(&self, tracer: &mut broom::prelude::Tracer<Self>) {
        match self {
            HeapItem::Object(_, items) => {
                for item in items {
                    item.trace(tracer);
                }
            }
            HeapItem::Dyn(_, item) => item.trace(tracer),
            HeapItem::String(_) => {}
        }
    }
}
