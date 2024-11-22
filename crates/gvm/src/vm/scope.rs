use std::collections::HashMap;

use crate::format::instr::ByteCode;

use super::stack::StackItem;

pub struct Scope<'code> {
    pub args: Vec<StackItem>,
    pub locals: HashMap<u32, StackItem>,
    pub stack: Vec<StackItem>,
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
