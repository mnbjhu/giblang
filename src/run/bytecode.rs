use std::{collections::HashMap, fmt::Display};

use chumsky::container::Container;

use crate::{lexer::literal::Literal, run::DebugText};

use super::{
    scope::Scope,
    state::{FuncDef, ProgramState},
    Object,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ByteCode {
    Push(Literal),
    Pop,
    Print,
    Panic,
    Construct { id: u32, len: u32 },
    Call(u32),
    Return,
    NewLocal,
    GetLocal(u32),
    SetLocal(u32),
    Param(u32),
    Goto(u32),
    Add,
    Mul,
    Sub,
    Or,
    And,
    Not,
    Eq,
}

#[allow(clippy::too_many_lines)]
impl<'code> ProgramState<'code> {
    pub fn execute(&mut self, code: &'code ByteCode, funcs: &'code HashMap<u32, FuncDef>) {
        match code {
            ByteCode::Push(lit) => {
                let refr = self.heap.insert(lit.clone().into());
                self.push(refr.into());
            }
            ByteCode::Pop => {
                self.pop();
            }
            ByteCode::Print => {
                print!("{}", self.pop().get_text(self));
            }
            ByteCode::Panic => {
                panic!("{}", self.pop().get_text(self));
            }
            ByteCode::Call(id) => {
                let func = &funcs[id];
                let mut args = Vec::new();
                for _ in 0..func.args {
                    args.push(self.pop());
                }
                let scope = Scope {
                    args,
                    locals: Vec::new(),
                    stack: Vec::new(),
                    code: &func.body,
                    index: 0,
                    id: *id,
                };
                self.scopes.push(scope);
            }
            ByteCode::Return => {
                let ret = self.scope_mut().stack.pop();
                self.scopes.pop();
                if let Some(ret) = ret {
                    if !self.scopes.is_empty() {
                        self.push(ret);
                    }
                }
            }
            ByteCode::Construct { id, len } => {
                let mut args = Vec::new();
                for _ in 0..*len {
                    args.push(self.pop());
                }
                let obj = Object::Vec(*id, args);
                let refr = self.heap.insert(obj);
                self.push(refr.into());
            }
            ByteCode::NewLocal => {
                let refr = self.pop();
                self.new_local(refr);
            }
            ByteCode::GetLocal(id) => {
                let local = self.get_local(*id);
                self.push(local);
            }
            ByteCode::SetLocal(id) => {
                let refr = self.pop();
                self.set_local(*id, refr);
            }
            ByteCode::Goto(line) => {
                let cond = self.pop();
                if let Object::Bool(cond) = self.heap.get(cond).unwrap() {
                    if *cond {
                        self.scope_mut().index = *line as usize;
                    }
                } else {
                    panic!("Expected condition to be a boolean")
                }
            }
            ByteCode::Param(id) => {
                let refr = self.get_param(*id);
                self.push(refr);
            }
            ByteCode::Mul => {
                let b = self.pop();
                let a = self.pop();
                match (self.heap.get(a).unwrap(), self.heap.get(b).unwrap()) {
                    (Object::Int(a), Object::Int(b)) => {
                        let res = self.heap.insert(Object::Int(a * b));
                        self.push(res.into());
                    }
                    (Object::Float(a), Object::Float(b)) => {
                        let res = self.heap.insert(Object::Float(a * b));
                        self.push(res.into());
                    }
                    _ => {
                        panic!("Cannot 'mul' non-numbers")
                    }
                }
            }
            ByteCode::Add => {
                let b = self.pop();
                let a = self.pop();
                match (self.heap.get(a).unwrap(), self.heap.get(b).unwrap()) {
                    (Object::Int(a), Object::Int(b)) => {
                        let res = self.heap.insert(Object::Int(a + b));
                        self.push(res.into());
                    }
                    (Object::Float(a), Object::Float(b)) => {
                        let res = self.heap.insert(Object::Float(a + b));
                        self.push(res.into());
                    }
                    _ => {
                        panic!("Cannot 'add' non-numbers")
                    }
                }
            }
            ByteCode::Sub => {
                let b = self.pop();
                let a = self.pop();
                match (self.heap.get(a).unwrap(), self.heap.get(b).unwrap()) {
                    (Object::Int(a), Object::Int(b)) => {
                        let res = self.heap.insert(Object::Int(a - b));
                        self.push(res.into());
                    }
                    (Object::Float(a), Object::Float(b)) => {
                        let res = self.heap.insert(Object::Float(a - b));
                        self.push(res.into());
                    }
                    _ => {
                        panic!("Cannot 'sub' non-numbers")
                    }
                }
            }
            ByteCode::And => {
                let b = self.pop();
                let a = self.pop();
                match (self.heap.get(a).unwrap(), self.heap.get(b).unwrap()) {
                    (Object::Bool(a), Object::Bool(b)) => {
                        let res = self.heap.insert(Object::Bool(*a && *b));
                        self.push(res.into());
                    }
                    _ => {
                        panic!("Cannot 'and' non-bools")
                    }
                }
            }
            ByteCode::Or => {
                let b = self.pop();
                let a = self.pop();
                match (self.heap.get(a).unwrap(), self.heap.get(b).unwrap()) {
                    (Object::Bool(a), Object::Bool(b)) => {
                        let res = self.heap.insert(Object::Bool(*a || *b));
                        self.push(res.into());
                    }
                    _ => {
                        panic!("Cannot 'or' non-bools")
                    }
                }
            }
            ByteCode::Eq => {
                let b = self.pop();
                let a = self.pop();
                let a = self.heap.get(a).unwrap();
                let b = self.heap.get(b).unwrap();
                let res = self.heap.insert(Object::Bool(a == b));
                self.push(res.into());
            }
            ByteCode::Not => {
                let a = self.pop();
                match self.heap.get(a).unwrap() {
                    Object::Bool(a) => {
                        let res = self.heap.insert(Object::Bool(!a));
                        self.push(res.into());
                    }
                    _ => {
                        panic!("Cannot 'or' non-bools")
                    }
                }
            }
        };
    }
}

impl Display for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteCode::Push(lit) => write!(f, "push {lit}"),
            ByteCode::Pop => write!(f, "pop"),
            ByteCode::Print => write!(f, "print"),
            ByteCode::Panic => write!(f, "panic"),
            ByteCode::Construct { id, len } => write!(f, "construct {id}, {len}"),
            ByteCode::Call(id) => write!(f, "call {id}"),
            ByteCode::Return => write!(f, "return"),
            ByteCode::NewLocal => write!(f, "new"),
            ByteCode::GetLocal(id) => write!(f, "get {id}"),
            ByteCode::SetLocal(id) => write!(f, "set {id}"),
            ByteCode::Param(id) => write!(f, "param {id}"),
            ByteCode::Goto(line) => write!(f, "goto {line}"),
            ByteCode::Add => write!(f, "add"),
            ByteCode::Mul => write!(f, "mul"),
            ByteCode::Sub => write!(f, "sub"),
            ByteCode::Or => write!(f, "or"),
            ByteCode::And => write!(f, "and"),
            ByteCode::Not => write!(f, "not"),
            ByteCode::Eq => write!(f, "eq"),
        }
    }
}
