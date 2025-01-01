use super::{
    heap::HeapItem,
    stack::{Slice, StackItem},
    state::ProgramState,
};

impl HeapItem {
    pub fn get(&self, index: u32) -> Option<StackItem> {
        match self {
            HeapItem::Object(_, items) => items.get(index as usize).cloned(),
            HeapItem::String(string) => {
                let index = index as usize;
                if index < string.len() {
                    Some(StackItem::Char(string.chars().nth(index).unwrap()))
                } else {
                    None
                }
            }
            HeapItem::Dyn(_, item) => panic!("Cannot index into dynamic type {:?}", item),
        }
    }

    pub fn set(&mut self, index: u32, item: StackItem) {
        match self {
            HeapItem::Object(_, items) => {
                let index = index as usize;
                if index < items.len() {
                    items[index] = item;
                } else {
                    panic!("Index out of bounds");
                }
            }
            HeapItem::String(string) => {
                let index = index as usize;
                if index < string.len() {
                    if let StackItem::Char(ch) = item {
                        let mut chars = string.chars().collect::<Vec<_>>();
                        chars[index] = ch;
                        *string = chars.into_iter().collect();
                    } else {
                        panic!("Cannot set non-char item in string");
                    }
                } else {
                    panic!("Index out of bounds");
                }
            }
            HeapItem::Dyn(_, item) => panic!("Cannot index into dynamic type {:?}", item),
        }
    }

    pub fn push(&mut self, item: StackItem) {
        match self {
            HeapItem::Object(_, items) => items.push(item),
            HeapItem::String(string) => {
                if let StackItem::Char(ch) = item {
                    string.push(ch);
                } else {
                    panic!("Cannot push non-char item into string");
                }
            }
            HeapItem::Dyn(_, item) => panic!("Cannot push into dynamic type {:?}", item),
        }
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        match self {
            HeapItem::Object(_, items) => items.pop(),
            HeapItem::String(string) => string.pop().map(StackItem::Char),
            HeapItem::Dyn(_, item) => panic!("Cannot pop from dynamic type {:?}", item),
        }
    }

    pub fn peak(&self) -> Option<StackItem> {
        match self {
            HeapItem::Object(_, items) => items.last().cloned(),
            HeapItem::String(string) => string.chars().last().map(StackItem::Char),
            HeapItem::Dyn(_, item) => panic!("Cannot peak into dynamic type {:?}", item),
        }
    }

    pub fn insert(&mut self, index: u32, item: StackItem) {
        match self {
            HeapItem::Object(_, items) => {
                let index = index as usize;
                if index < items.len() {
                    items.insert(index, item);
                } else {
                    panic!("Index out of bounds");
                }
            }
            HeapItem::String(string) => {
                let index = index as usize;
                if index < string.len() {
                    if let StackItem::Char(ch) = item {
                        let mut chars = string.chars().collect::<Vec<_>>();
                        chars.insert(index, ch);
                        *string = chars.into_iter().collect();
                    } else {
                        panic!("Cannot insert non-char item in string");
                    }
                } else {
                    panic!("Index out of bounds");
                }
            }
            HeapItem::Dyn(_, item) => panic!("Cannot insert into dynamic type {:?}", item),
        }
    }

    pub fn remove(&mut self, index: u32) -> Option<StackItem> {
        match self {
            HeapItem::Object(_, items) => {
                let index = index as usize;
                if index < items.len() {
                    Some(items.remove(index))
                } else {
                    None
                }
            }
            HeapItem::String(string) => {
                let index = index as usize;
                if index < string.len() {
                    let ch = string.chars().nth(index).unwrap();
                    let mut chars = string.chars().collect::<Vec<_>>();
                    chars.remove(index);
                    *string = chars.into_iter().collect();
                    Some(StackItem::Char(ch))
                } else {
                    None
                }
            }
            HeapItem::Dyn(_, item) => panic!("Cannot remove from dynamic type {:?}", item),
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        match self {
            HeapItem::Object(_, items) => items.len() as u32,
            HeapItem::String(string) => string.len() as u32,
            HeapItem::Dyn(_, item) => panic!("Cannot get length of dynamic type {:?}", item),
        }
    }
}

impl Slice {
    pub fn get(&self, index: u32, state: &ProgramState) -> Option<StackItem> {
        let index = self.start + index;
        state.heap.get(self.refr)?.get(index)
    }

    pub fn peak(&self, state: &ProgramState) -> Option<StackItem> {
        let len = state.heap.get(self.refr).unwrap().len();
        self.get(len - 1, state)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        self.end - self.start
    }
}

impl StackItem {
    pub fn get(&self, index: u32, state: &ProgramState) -> Option<StackItem> {
        match self {
            StackItem::Heap(handle) => state.heap.get(*handle)?.get(index),
            StackItem::Slice(slice) => slice.get(index, state),
            _ => panic!("Cannot index into non-heap, non-slice item {:?}", self),
        }
    }

    pub fn set(&mut self, index: u32, item: StackItem, state: &mut ProgramState) {
        match self {
            StackItem::Heap(handle) => {
                let heap = state.heap.get_mut(*handle).unwrap();
                heap.set(index, item);
            }
            _ => panic!("Cannot index into non-heap item {:?}", self),
        }
    }

    pub fn push(&mut self, item: StackItem, state: &mut ProgramState) {
        match self {
            StackItem::Heap(handle) => {
                let heap = state.heap.get_mut(*handle).unwrap();
                heap.push(item);
            }
            _ => panic!("Cannot push into non-heap item {:?}", self),
        }
    }

    pub fn pop(&mut self, state: &mut ProgramState) -> Option<StackItem> {
        match self {
            StackItem::Heap(handle) => {
                let heap = state.heap.get_mut(*handle).unwrap();
                heap.pop()
            }
            _ => panic!("Cannot pop from non-heap item {:?}", self),
        }
    }

    pub fn peak(&self, state: &ProgramState) -> Option<StackItem> {
        match self {
            StackItem::Heap(handle) => {
                let heap = state.heap.get(*handle).unwrap();
                heap.peak()
            }
            StackItem::Slice(slice) => slice.peak(state),
            _ => panic!("Cannot peak into non-heap item {:?}", self),
        }
    }

    pub fn insert(&mut self, index: u32, item: StackItem, state: &mut ProgramState) {
        match self {
            StackItem::Heap(handle) => {
                let heap = state.heap.get_mut(*handle).unwrap();
                heap.insert(index, item);
            }
            _ => panic!("Cannot insert into non-heap item {:?}", self),
        }
    }

    pub fn remove(&mut self, index: u32, state: &mut ProgramState) -> Option<StackItem> {
        match self {
            StackItem::Heap(handle) => {
                let heap = state.heap.get_mut(*handle).unwrap();
                heap.remove(index)
            }
            _ => panic!("Cannot remove from non-heap item {:?}", self),
        }
    }

    pub fn len(&self, state: &ProgramState) -> u32 {
        match self {
            StackItem::Heap(handle) => {
                let heap = state.heap.get(*handle).unwrap();
                heap.len()
            }
            StackItem::Slice(slice) => slice.len(),
            _ => panic!("Cannot get length of non-heap item {:?}", self),
        }
    }

    pub fn slice(&self, start: u32, end: u32, state: &ProgramState) -> StackItem {
        match self {
            StackItem::Heap(handle) => {
                let heap = state.heap.get(*handle).unwrap();
                if matches!(heap, HeapItem::Dyn(_, _)) {
                    panic!("Cannot slice dynamic type {:?}", heap);
                } else {
                    StackItem::Slice(Slice {
                        start,
                        end,
                        refr: *handle,
                    })
                }
            }
            StackItem::Slice(slice) => StackItem::Slice(Slice {
                start: slice.start + start,
                end: slice.start + end,
                refr: slice.refr,
            }),
            _ => panic!("Cannot slice non-heap item {:?}", self),
        }
    }
}
