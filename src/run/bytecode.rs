use std::{collections::HashMap, fmt::Display};

use crate::{lexer::literal::Literal, run::DebugText};

use super::{
    scope::Scope,
    state::{FuncDef, ProgramState},
    Object, StackItem,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ByteCode {
    Push(Literal),
    Copy,
    Pop,
    Print,
    Panic,
    Construct { id: u32, len: u32 },
    Call(u32),
    Return,
    Index(u32),
    SetIndex(u32),
    NewLocal(u32),
    GetLocal(u32),
    SetLocal(u32),
    Param(u32),
    Goto(u32),
    Je(i32),
    Jne(i32),
    Jmp(i32),
    Id,
    Add,
    Mul,
    Sub,
    Or,
    And,
    Not,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    Match(u32),
    Clone,
}

#[allow(clippy::too_many_lines)]
impl<'code> ProgramState<'code> {
    pub fn execute(&mut self, code: &'code ByteCode, funcs: &'code HashMap<u32, FuncDef>) {
        match code {
            ByteCode::Push(lit) => {
                self.push(lit.clone().into());
            }
            ByteCode::Pop => {
                self.pop();
            }
            ByteCode::Id => {
                if let StackItem::Vec(id, _) = self.peak() {
                    let res = StackItem::Int(*id as i32);
                    self.push(res);
                } else {
                    panic!("Expected vec")
                }
            }
            ByteCode::Match(expected) => {
                if let StackItem::Vec(id, _) = self.pop() {
                    let res = StackItem::Bool(id == *expected);
                    self.push(res);
                } else {
                    panic!("Expected vec")
                }
            }
            ByteCode::Copy => {
                let refr = self.peak();
                self.push(refr.clone());
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
                    locals: HashMap::new(),
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
                let refr = self.heap.insert(Object(args));
                let res = StackItem::Vec(*id, refr.into());
                self.push(res);
            }
            ByteCode::NewLocal(id) => {
                let refr = self.pop();
                self.new_local(*id, refr);
            }
            ByteCode::GetLocal(id) => {
                let local = self.get_local(*id);
                self.push(local);
            }
            ByteCode::SetLocal(id) => {
                let refr = self.pop();
                self.set_local(*id, refr);
            }
            ByteCode::Je(diff) => {
                if let StackItem::Bool(cond) = self.pop() {
                    if cond {
                        self.scope_mut().index = (self.scope().index as i32 + diff - 1) as usize;
                    }
                } else {
                    panic!("Expected condition to be a boolean")
                }
            }
            ByteCode::Jne(diff) => {
                if let StackItem::Bool(cond) = self.pop() {
                    if !cond {
                        self.scope_mut().index = (self.scope().index as i32 + diff - 1) as usize;
                    }
                } else {
                    panic!("Expected condition to be a boolean")
                }
            }
            ByteCode::Jmp(diff) => {
                self.scope_mut().index = (self.scope().index as i32 + diff - 1) as usize;
            }
            ByteCode::Goto(line) => {
                if let StackItem::Bool(cond) = self.pop() {
                    if cond {
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
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Int(a * b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Float(a * b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'mul' non-numbers")
                    }
                }
            }
            ByteCode::Add => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Int(a + b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Float(a + b);
                        self.push(res);
                    }
                    (StackItem::String(a), StackItem::String(b)) => {
                        let res = StackItem::String(format!("{a}{b}"));
                        self.push(res);
                    }
                    _ => {
                        panic!("Can only add numbers or strings")
                    }
                }
            }
            ByteCode::Sub => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Int(a - b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Float(a - b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'sub' non-numbers")
                    }
                }
            }
            ByteCode::And => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Bool(a), StackItem::Bool(b)) => {
                        let res = StackItem::Bool(a && b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'and' non-bools")
                    }
                }
            }
            ByteCode::Or => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Bool(a), StackItem::Bool(b)) => {
                        let res = StackItem::Bool(a || b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot 'or' non-bools")
                    }
                }
            }
            ByteCode::Eq => {
                let b = self.pop();
                let a = self.pop();
                let res = StackItem::Bool(a == b);
                self.push(res);
            }
            ByteCode::Neq => {
                let b = self.pop();
                let a = self.pop();
                let res = StackItem::Bool(a != b);
                self.push(res);
            }
            ByteCode::Lt => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Bool(a < b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Bool(a < b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot '<' non-numbers")
                    }
                }
            }
            ByteCode::Gt => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Bool(a > b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Bool(a > b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot '>' non-numbers")
                    }
                }
            }
            ByteCode::Lte => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Bool(a <= b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Bool(a <= b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot '<=' non-numbers")
                    }
                }
            }
            ByteCode::Gte => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (StackItem::Int(a), StackItem::Int(b)) => {
                        let res = StackItem::Bool(a >= b);
                        self.push(res);
                    }
                    (StackItem::Float(a), StackItem::Float(b)) => {
                        let res = StackItem::Bool(a >= b);
                        self.push(res);
                    }
                    _ => {
                        panic!("Cannot '>=' non-numbers")
                    }
                }
            }
            ByteCode::Not => match self.pop() {
                StackItem::Bool(a) => {
                    let res = StackItem::Bool(!a);
                    self.push(res);
                }
                _ => {
                    panic!("Cannot 'or' non-bools")
                }
            },
            ByteCode::Index(index) => {
                let StackItem::Vec(_, refr) = self.pop() else {
                    panic!("Cannot index non-vec")
                };
                let data = self.heap.get(refr).unwrap();
                let res = &data.0[*index as usize];
                self.push(res.clone());
            }
            ByteCode::SetIndex(index) => {
                let value = self.pop();
                let vec = self.pop();
                let StackItem::Vec(_, refr) = &vec else {
                    panic!("Cannot index non-vec")
                };
                let data = self.heap.get_mut(*refr).unwrap();
                data.0[*index as usize] = value;
            }
            ByteCode::Clone => {
                let refr = self.pop();
                if let StackItem::Vec(id, refr) = refr {
                    let data = self.heap.get(refr).unwrap();
                    let refr = self.heap.insert(Object(data.0.clone()));
                    let res = StackItem::Vec(id, refr.into());
                    self.push(res);
                } else {
                    self.push(refr.clone());
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
            ByteCode::Index(index) => write!(f, "index {index}"),
            ByteCode::SetIndex(index) => write!(f, "set_index {index}"),
            ByteCode::Call(id) => write!(f, "call {id}"),
            ByteCode::Return => write!(f, "return"),
            ByteCode::NewLocal(id) => write!(f, "new {id}"),
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
            ByteCode::Copy => write!(f, "copy"),
            ByteCode::Je(diff) => write!(f, "je {diff}"),
            ByteCode::Jne(diff) => write!(f, "jne {diff}"),
            ByteCode::Jmp(diff) => write!(f, "jmp {diff}"),
            ByteCode::Id => write!(f, "id"),
            ByteCode::Match(id) => write!(f, "match {id}"),
            ByteCode::Lt => write!(f, "lt"),
            ByteCode::Gt => write!(f, "gt"),
            ByteCode::Lte => write!(f, "lte"),
            ByteCode::Gte => write!(f, "gte"),
            ByteCode::Neq => write!(f, "neq"),
            ByteCode::Clone => write!(f, "clone"),
        }
    }
}
