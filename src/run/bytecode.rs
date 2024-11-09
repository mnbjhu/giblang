use chumsky::container::Container;

use crate::{lexer::literal::Literal, run::DebugText};

use super::{
    scope::Scope,
    state::{FuncDef, ProgramState},
    Object,
};

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
}

impl<'code> ProgramState<'code> {
    pub fn execute(&mut self, code: &'code ByteCode, funcs: &'code [FuncDef]) {
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
                let func = &funcs[*id as usize];
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
                };
                self.scopes.push(scope);
            }
            ByteCode::Return => {
                self.scopes.pop();
            },
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
        };
    }
}
