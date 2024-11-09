use broom::Handle;

use super::{bytecode::ByteCode, state::FuncDef, Object};

pub struct Scope<'code> {
    pub args: Vec<Handle<Object>>,
    pub locals: Vec<Handle<Object>>,
    pub stack: Vec<Handle<Object>>,
    pub code: &'code [ByteCode],
    pub index: usize,
}

impl<'code> Scope<'code> {
    pub fn from_code(code: &'code [ByteCode]) -> Self {
        Self {
            args: Vec::new(),
            locals: Vec::new(),
            stack: Vec::new(),
            code,
            index: 0,
        }
    }

    pub fn next_instr(&mut self) -> &'code ByteCode {
        let code = &self.code[self.index];
        self.index += 1;
        code
    }
}
