use std::{collections::HashMap, usize};

use broom::Heap;

use crate::{lexer::literal::Literal, run::DebugText as _};

use super::{bytecode::ByteCode, scope::Scope, Object, StackItem};

pub struct ProgramState<'code> {
    pub heap: Heap<Object>,
    pub scopes: Vec<Scope<'code>>,
    pub vtables: HashMap<u32, HashMap<u32, u32>>, // trait_func_id -> (type_id -> impl_func_id)
}

pub struct FuncDef {
    pub args: u32,
    pub offset: usize,
    pub body: Vec<ByteCode>,
}

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
    pub fn new() -> Self {
        Self {
            heap: Heap::default(),
            scopes: vec![],
            vtables: HashMap::new(),
        }
    }

    pub fn scope(&self) -> &Scope<'code> {
        self.scopes.last().expect("Call stack underflow")
    }

    pub fn scope_mut(&mut self) -> &mut Scope<'code> {
        self.scopes.last_mut().expect("Call stack underflow")
    }

    pub fn run_debug(&mut self, funcs: &'code HashMap<u32, FuncDef>) {
        let main = funcs.get(&0).expect("No main function");
        self.scopes.push(Scope::from_code(&main.body, 0));
        while !self.scopes.is_empty() {
            let instr = self.next_instr();
            println!(
                "{instr:?} : {:?} : {:?}",
                self.stack_trace(),
                self.scope()
                    .stack
                    .iter()
                    .map(|it| it.get_text(self))
                    .collect::<Vec<_>>()
                    .join("|"),
            );
            self.execute(instr, funcs);
        }
    }

    pub fn run(&mut self, funcs: &'code HashMap<u32, FuncDef>) {
        let main = funcs.get(&0).expect("No main function");
        self.scopes.push(Scope::from_code(&main.body, 0));
        while !self.scopes.is_empty() {
            let instr = self.next_instr();
            self.execute(instr, funcs);
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
            .map(|scope| format!("{}:{}", scope.id, scope.index - 1))
            .collect::<Vec<_>>()
            .join("/")
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

    pub fn get_trait_impl(&self, func_id: u32, trait_id: u32) -> Option<u32> {
        self.vtables.get(&func_id)?.get(&trait_id).copied()
    }
}
