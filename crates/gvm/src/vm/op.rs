use super::{
    heap::HeapItem,
    stack::{Slice, StackItem},
    state::ProgramState,
};

impl<'tmp> StackItem {
    pub fn add(&self, other: &StackItem, state: &mut ProgramState) -> StackItem {
        match (self, other) {
            (StackItem::Int(a), StackItem::Int(b)) => return StackItem::Int(a + b),
            (StackItem::Float(a), StackItem::Float(b)) => return StackItem::Float(a + b),
            _ => {}
        }
        if let Some((id, vec)) = self.vec_data(state) {
            if let Some((_, other)) = other.vec_data(state) {
                let mut new = vec.to_vec();
                new.extend_from_slice(other);
                let data = HeapItem::Object(id, new);
                let refr = state.heap.insert(data);
                return StackItem::Heap(refr.into());
            } else {
                panic!("Cannot add non-vec to vec \n{}", state.stack_trace());
            }
        }
        if let Some(text) = self.string_data(state) {
            if let Some(other) = other.string_data(state) {
                let new = format!("{}{}", text, other);
                let data = HeapItem::String(new);
                let refr = state.heap.insert(data);
                return StackItem::Heap(refr.into());
            } else {
                panic!("Cannot add non-vec to vec \n{}", state.stack_trace());
            }
        }
        panic!(
            "Cannot add {:?} and {:?}\n{}",
            self,
            other,
            state.stack_trace()
        )
    }

    pub fn do_clone(&self, state: &mut ProgramState) -> Self {
        match self {
            StackItem::Slice(slice) => slice.do_clone(state),
            _ => *self,
        }
    }

    pub fn string_data(&self, state: &'tmp ProgramState) -> Option<&'tmp str> {
        match self {
            StackItem::Slice(Slice { refr, start, end }) => match state.heap.get(refr).unwrap() {
                HeapItem::Object(_, _) => None,
                HeapItem::String(text) => {
                    let start = *start as usize;
                    let end = *end as usize;
                    Some(&text[start..end])
                }
                HeapItem::Dyn(_, inner) => inner.string_data(state),
            },
            StackItem::Heap(d) => match state.heap.get(d).unwrap() {
                HeapItem::Object(_, _) => None,
                HeapItem::String(text) => Some(text),
                HeapItem::Dyn(_, inner) => inner.string_data(state),
            },
            _ => None,
        }
    }
    pub fn vec_data(&self, state: &'tmp ProgramState) -> Option<(u32, &'tmp [StackItem])> {
        match self {
            StackItem::Slice(Slice { start, end, refr }) => match state.heap.get(refr).unwrap() {
                HeapItem::Object(id, items) => {
                    let start = *start as usize;
                    let end = *end as usize;
                    Some((*id, &items[start..end]))
                }
                HeapItem::String(_) => None,
                HeapItem::Dyn(_, inner) => inner.vec_data(state),
            },
            StackItem::Heap(refr) => match state.heap.get(refr).unwrap() {
                HeapItem::Object(id, data) => Some((*id, data)),
                HeapItem::String(_) => None,
                HeapItem::Dyn(_, inner) => inner.vec_data(state),
            },
            _ => None,
        }
    }

    pub fn is_equal(&self, other: &StackItem, state: &ProgramState) -> bool {
        match (self, other) {
            (StackItem::Int(a), StackItem::Int(b)) => a == b,
            (StackItem::Float(a), StackItem::Float(b)) => a == b,
            (StackItem::Char(a), StackItem::Char(b)) => a == b,
            (StackItem::Bool(a), StackItem::Bool(b)) => a == b,
            _ => {
                if let Some((id, vec)) = self.vec_data(state) {
                    if let Some((other_id, other)) = other.vec_data(state) {
                        return vec == other && id == other_id;
                    }
                } else if let Some(text) = self.string_data(state) {
                    if let Some(other) = other.string_data(state) {
                        return text == other;
                    }
                }
                panic!(
                    "Cannot compare {:?} and {:?}\n{}",
                    self,
                    other,
                    state.stack_trace()
                )
            }
        }
    }
}

impl Slice {
    pub fn do_clone(&self, state: &mut ProgramState) -> StackItem {
        match state.heap.get(self.refr).unwrap() {
            HeapItem::Object(id, data) => {
                let start = self.start as usize;
                let end = self.end as usize;
                let data = HeapItem::Object(*id, data[start..end].to_vec());
                let refr = state.heap.insert(data);
                StackItem::Heap(refr.into())
            }
            HeapItem::String(text) => {
                let start = self.start as usize;
                let end = self.end as usize;
                let data = HeapItem::String(text[start..end].to_string());
                let refr = state.heap.insert(data);
                StackItem::Heap(refr.into())
            }
            _ => unreachable!(),
        }
    }
}
