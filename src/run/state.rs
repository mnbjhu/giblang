use std::{collections::HashMap, usize};

use broom::Heap;

use crate::{check::build_state::VTable, lexer::literal::Literal, run::DebugText as _};

use super::{bytecode::ByteCode, scope::Scope, Object, StackItem};

pub struct ProgramState<'code> {
    pub funcs: &'code HashMap<u32, FuncDef>,
    pub heap: Heap<Object>,
    pub scopes: Vec<Scope<'code>>,
    pub vtables: HashMap<u64, HashMap<u32, u32>>, // type_id -> (trait_func_id -> impl_func_id)
    pub file_names: HashMap<u32, String>,
}

pub struct FuncDef {
    pub name: String,
    pub args: u32,
    pub pos: (u16, u16),
    pub file: u32,
    pub body: Vec<ByteCode>,
    pub marks: Vec<(usize, ByteCodeSpan)>,
}

pub type ByteCodeSpan = (u16, u16);

impl From<Literal> for StackItem {
    fn from(val: Literal) -> Self {
        match val {
            Literal::Int(int) => StackItem::Int(int.parse().unwrap()),
            Literal::Float(float) => StackItem::Float(float.parse().unwrap()),
            Literal::String(s) => StackItem::String(s),
            Literal::Bool(b) => StackItem::Bool(b),
            Literal::Char(c) => StackItem::Char(c),
        }
    }
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

    pub fn peak<'stack>(&'stack self) -> &'stack StackItem {
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
        self.scope().locals[&id].clone()
    }

    pub fn get_param(&self, id: u32) -> StackItem {
        self.scope().args[id as usize].clone()
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
}
