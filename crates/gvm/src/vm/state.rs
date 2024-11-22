use std::collections::HashMap;

use broom::Heap;

use crate::{
    format::{func::FuncDef, instr::ByteCode, literal::Literal, table::VTable},
    vm::text::DebugText as _,
};

use super::{heap::HeapItem, scope::Scope, stack::StackItem};

pub struct ProgramState<'code> {
    pub funcs: &'code HashMap<u32, FuncDef>,
    pub heap: Heap<HeapItem>,
    pub scopes: Vec<Scope<'code>>,
    pub vtables: HashMap<u64, HashMap<u32, u32>>, // type_id -> (trait_func_id -> impl_func_id)
    pub file_names: HashMap<u32, String>,
}

impl<'code> ProgramState<'code> {
    pub fn new(
        funcs: &'code HashMap<u32, FuncDef>,
        vtables: HashMap<u64, VTable>,
        file_names: HashMap<u32, String>,
    ) -> Self {
        Self {
            heap: Heap::default(),
            scopes: vec![],
            vtables,
            file_names,
            funcs,
        }
    }

    pub fn scope(&self) -> &Scope<'code> {
        self.scopes.last().expect("Call stack underflow")
    }

    pub fn scope_mut(&mut self) -> &mut Scope<'code> {
        self.scopes.last_mut().expect("Call stack underflow")
    }

    pub fn run_debug(&mut self) {
        let main = self.funcs.get(&0).expect("No main function");
        self.scopes.push(Scope::from_code(&main.body, 0));
        while !self.scopes.is_empty() {
            let instr = self.next_instr();
            println!(
                "{instr:?} : {}:{}",
                self.stack_trace(),
                self.scope()
                    .stack
                    .iter()
                    .map(|it| it.get_text(self))
                    .collect::<Vec<_>>()
                    .join("|"),
            );
            self.execute(instr);
        }
    }

    pub fn run(&mut self) {
        let main = self.funcs.get(&0).expect("No main function");
        self.scopes.push(Scope::from_code(&main.body, 0));
        while !self.scopes.is_empty() {
            let instr = self.next_instr();
            self.execute(instr);
        }
    }

    pub fn pop(&mut self) -> StackItem {
        if let Some(found) = self.scope_mut().stack.pop() {
            found
        } else {
            panic!("Stack underflow: {}", self.stack_trace())
        }
    }

    pub fn peak(&self) -> &StackItem {
        if let Some(found) = self.scope().stack.last() {
            found
        } else {
            panic!("Stack underflow: {}", self.stack_trace())
        }
    }

    pub fn stack_trace(&self) -> String {
        self.scopes
            .iter()
            .map(|scope| {
                let func = &self.funcs[&scope.id];
                let file_name = &self.file_names[&func.file];
                let marker = func
                    .marks
                    .iter()
                    .find_map(|(offset, pos)| {
                        if *offset >= scope.index {
                            Some(pos)
                        } else {
                            None
                        }
                    })
                    .copied()
                    .unwrap_or(func.pos);
                format!("{}:{}:{} ({})", file_name, marker.0, marker.1, func.name)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn new_local(&mut self, id: u32, refr: StackItem) {
        self.scope_mut().locals.insert(id, refr);
    }

    pub fn set_local(&mut self, id: u32, refr: StackItem) {
        self.scope_mut().locals.insert(id, refr);
    }

    pub fn get_local(&mut self, id: u32) -> StackItem {
        self.scope().locals[&id]
    }

    pub fn get_param(&self, id: u32) -> StackItem {
        self.scope().args[id as usize]
    }

    pub fn push(&mut self, refr: StackItem) {
        self.scope_mut().stack.push(refr);
    }

    pub fn next_instr(&mut self) -> &'code ByteCode {
        self.scope_mut().next_instr()
    }

    pub fn get_trait_impl(&self, func_id: u32, type_id: u64) -> Option<u32> {
        self.vtables.get(&type_id)?.get(&func_id).copied()
    }

    pub fn create(&mut self, literal: &Literal) -> StackItem {
        match literal {
            Literal::Int(num) => StackItem::Int(*num),
            Literal::Float(num) => StackItem::Float(*num),
            Literal::Bool(val) => StackItem::Bool(*val),
            Literal::Char(val) => StackItem::Char(*val),
            Literal::String(data) => {
                let refr = self.heap.insert(HeapItem::String(data.clone()));
                StackItem::Heap(refr.into())
            }
        }
    }
}
