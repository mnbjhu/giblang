use std::collections::HashMap;

use crate::db::input::Db;

pub struct BuildState<'db> {
    pub vars: Vec<HashMap<String, u32>>,
    pub params: HashMap<String, u32>,
    pub var_count: u32,
    pub db: &'db dyn Db,
}

impl<'db> BuildState<'db> {
    pub fn new(db: &'db dyn Db) -> Self {
        BuildState {
            vars: vec![],
            params: HashMap::new(),
            var_count: 0,
            db,
        }
    }

    pub fn add_var(&mut self, name: String) -> u32 {
        let var = self.var_count;
        self.var_count += 1;
        self.vars.last_mut().unwrap().insert(name, var);
        var
    }

    pub fn get_var(&self, name: &str) -> Option<u32> {
        self.vars
            .iter()
            .rev()
            .find_map(|vars| vars.get(name).copied())
    }

    pub fn get_param(&self, name: &str) -> Option<u32> {
        self.params.get(name).copied()
    }

    pub fn clear(&mut self) {
        self.vars.clear();
        self.params.clear();
        self.var_count = 0;
        self.enter_scope();
    }

    pub fn enter_scope(&mut self) {
        self.vars.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.vars.pop();
    }

    pub fn add_param(&mut self, name: String, id: u32) {
        self.params.insert(name, id);
    }
}
