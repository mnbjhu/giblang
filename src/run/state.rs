use std::{collections::HashMap, usize};

use broom::{Handle, Heap};

use crate::lexer::literal::Literal;

use super::{bytecode::ByteCode, scope::Scope, Object};

pub struct ProgramState<'code> {
    pub heap: Heap<Object>,
    pub scopes: Vec<Scope<'code>>,
}

pub struct FuncDef {
    pub args: u32,
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
    pub fn new(code: &'code [ByteCode], id: u32) -> Self {
        Self {
            heap: Heap::default(),
            scopes: vec![Scope::from_code(code, id)],
        }
    }

    pub fn scope(&self) -> &Scope<'code> {
        self.scopes.last().expect("Call stack underflow")
    }

    pub fn scope_mut(&mut self) -> &mut Scope<'code> {
        self.scopes.last_mut().expect("Call stack underflow")
    }

    pub fn run(&mut self, funcs: &'code HashMap<u32, FuncDef>) {
        while !self.scopes.is_empty() {
            let instr = self.next_instr();
            // println!("Executing: {instr:?} {}", self.stack_trace());
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{lexer::literal::Literal, run::bytecode::ByteCode};
    use ByteCode::*;

    use super::{FuncDef, ProgramState};

    #[test]
    fn test_basic() {
        let code = vec![Push(Literal::String("Hello".to_string())), Panic];
        let mut prog = ProgramState::new(&code, 0);
        prog.run(&HashMap::new());
    }

    #[test]
    fn test_func() {
        let body = vec![Push(Literal::String("Hello Func".to_string())), Panic];
        let hello = FuncDef { args: 0, body };
        let code = vec![Push(Literal::String("Hello".to_string())), Print, Call(0)];
        let mut funcs = HashMap::new();
        funcs.insert(0, hello);
        let mut prog = ProgramState::new(&code, 0);
        prog.run(&funcs);
    }
}
