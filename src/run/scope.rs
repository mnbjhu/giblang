use std::collections::HashMap;

use broom::Handle;

use super::{bytecode::ByteCode, Object};

pub struct Scope<'code> {
    pub args: Vec<Handle<Object>>,
    pub locals: HashMap<u32, Handle<Object>>,
    pub stack: Vec<Handle<Object>>,
    pub code: &'code [ByteCode],
    pub index: usize,
    pub id: u32,
}

impl<'code> Scope<'code> {
    pub fn from_code(code: &'code [ByteCode], id: u32) -> Self {
        Self {
            args: Vec::new(),
            locals: HashMap::new(),
            stack: Vec::new(),
            code,
            index: 0,
            id,
        }
    }

    pub fn next_instr(&mut self) -> &'code ByteCode {
        let code = &self.code[self.index];
        self.index += 1;
        code
    }
}
