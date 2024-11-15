use std::collections::HashMap;

use crate::run::{bytecode::ByteCode, state::FuncDef};

#[derive(Default)]
pub struct BuildState {
    pub funcs: Vec<(u32, FuncDef)>,
    pub vars: HashMap<String, u32>,
    pub var_count: u32,
}

impl BuildState {
    pub fn add(&mut self, code: ByteCode) {
        self.funcs.last_mut().unwrap().1.body.push(code);
    }

    pub fn add_func(&mut self, id: u32, func: FuncDef) {
        self.var_count = 0;
        self.vars.clear();
        self.funcs.push((id, func));
    }

    pub fn add_var(&mut self, name: String) -> u32 {
        let var = self.var_count;
        self.var_count += 1;
        self.vars.insert(name, var);
        self.add(ByteCode::NewLocal);
        var
    }

    pub fn get_var(&self, name: &str) -> Option<u32> {
        self.vars.get(name).copied()
    }
}
