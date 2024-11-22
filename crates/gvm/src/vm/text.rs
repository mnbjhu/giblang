use super::{heap::HeapItem, stack::StackItem, state::ProgramState};

pub trait DebugText {
    fn get_text(&self, state: &ProgramState) -> String;
}

impl DebugText for StackItem {
    fn get_text(&self, state: &ProgramState) -> String {
        match self {
            StackItem::Int(i) => i.to_string(),
            StackItem::Float(f) => f.to_string(),
            StackItem::Char(c) => c.to_string(),
            StackItem::Bool(b) => b.to_string(),
            StackItem::Heap(handle) => {
                let item = state.heap.get(handle).unwrap();
                item.get_text(state)
            }
        }
    }
}

impl DebugText for HeapItem {
    fn get_text(&self, state: &ProgramState) -> String {
        match self {
            HeapItem::Object(it, items) => {
                let mut text = format!("Object({})", it);
                for item in items {
                    text.push_str(&item.get_text(state));
                    text.push_str(", ");
                }
                text
            }
            HeapItem::String(text) => {
                format!("\"{}\"", text)
            }
            HeapItem::Dyn(id, item) => {
                format!("Dyn({}, {})", id, item.get_text(state))
            }
        }
    }
}
