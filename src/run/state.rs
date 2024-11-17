use std::{collections::HashMap, usize};

use broom::{Handle, Heap};

use crate::{lexer::literal::Literal, run::DebugText as _};

use super::{bytecode::ByteCode, scope::Scope, Object};

pub struct ProgramState<'code> {
    pub heap: Heap<Object>,
    pub scopes: Vec<Scope<'code>>,
}

pub struct FuncDef {
    pub args: u32,
    pub offset: usize,
    pub body: Vec<ByteCode>,
}

impl From<Literal> for Object {
    fn from(val: Literal) -> Self {
        match val {
            Literal::Int(int) => Object::Int(int.parse().unwrap()),
            Literal::Float(float) => Object::Float(float.parse().unwrap()),
            Literal::String(s) => Object::String(s),
            Literal::Bool(b) => Object::Bool(b),
            Literal::Char(c) => Object::Char(c),
        }
    }
}

impl<'code> ProgramState<'code> {
    pub fn new() -> Self {
        Self {
            heap: Heap::default(),
            scopes: vec![],
        }
    }

    pub fn scope(&self) -> &Scope<'code> {
        self.scopes.last().expect("Call stack underflow")
    }

    pub fn scope_mut(&mut self) -> &mut Scope<'code> {
        self.scopes.last_mut().expect("Call stack underflow")
    }

    pub fn run(&mut self, funcs: &'code HashMap<u32, FuncDef>) {
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

    pub fn pop(&mut self) -> Handle<Object> {
        if let Some(found) = self.scope_mut().stack.pop() {
            found
        } else {
            panic!("Stack underflow: {}", self.stack_trace())
        }
    }

    pub fn peak(&self) -> Handle<Object> {
        if let Some(found) = self.scope().stack.last() {
            *found
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

    pub fn new_local(&mut self, refr: Handle<Object>) {
        self.scope_mut().locals.push(refr);
    }

    pub fn set_local(&mut self, id: u32, refr: Handle<Object>) {
        self.scope_mut().locals[id as usize] = refr;
    }

    pub fn get_local(&mut self, id: u32) -> Handle<Object> {
        self.scope().locals[id as usize]
    }

    pub fn get_param(&self, id: u32) -> Handle<Object> {
        self.scope().args[id as usize]
    }

    pub fn push(&mut self, refr: Handle<Object>) {
        self.scope_mut().stack.push(refr);
    }

    pub fn next_instr(&mut self) -> &'code ByteCode {
        self.scope_mut().next_instr()
    }
}
