use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use async_lsp::lsp_types::Position;
use chumsky::container::Container;
use rustc_hash::FxHasher;
use salsa::plumbing::AsId;

use crate::{
    db::{
        decl::Project,
        input::{Db, SourceFile},
    },
    range::offset_to_position_str,
    ty::Ty,
    util::Span,
};

pub struct BuildState<'db> {
    pub vars: Vec<HashMap<String, u32>>,
    pub params: HashMap<String, u32>,
    pub var_count: u32,
    pub db: &'db dyn Db,
    pub vtables: HashMap<u64, VTable>,
    pub vtable_map: HashMap<Ty<'db>, u64>,
    pub project: Project<'db>,
    pub hasher: FxHasher,
    pub file: SourceFile,
    pub marks: Vec<(usize, (u16, u16))>,
    pub block_scopes: Vec<usize>,
}

pub type VTable = HashMap<u32, u32>;

impl<'db> BuildState<'db> {
    pub fn new(db: &'db dyn Db, project: Project<'db>, file: SourceFile) -> Self {
        BuildState {
            vars: vec![],
            params: HashMap::new(),
            var_count: 0,
            db,
            vtables: HashMap::new(),
            vtable_map: HashMap::new(),
            project,
            hasher: FxHasher::default(),
            file,
            marks: Vec::new(),
            block_scopes: Vec::new(),
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
        self.block_scopes.push(0);
    }

    pub fn exit_scope(&mut self) {
        self.vars.pop();
        self.block_scopes.pop();
    }

    pub fn add_param(&mut self, name: String, id: u32) {
        self.params.insert(name, id);
    }

    pub fn get_vtable(&mut self, ty: &Ty<'db>) -> u64 {
        if let Some(existing) = self.vtable_map.get(ty) {
            return *existing;
        }
        let funcs = ty
            .get_trait_func_decls(self)
            .iter()
            .map(|(trait_, impl_)| (trait_.as_id().as_u32(), impl_.as_id().as_u32()))
            .collect::<HashMap<_, _>>();
        ty.hash(&mut self.hasher);
        let hash = self.hasher.finish();
        self.vtable_map.insert(ty.clone(), hash);
        self.vtables.insert(hash, funcs);
        hash
    }

    pub fn get_pos(&self, span: Span) -> (u16, u16) {
        let text = self.file.text(self.db);
        let Position { line, character } = offset_to_position_str(span.start, text);
        (line as u16, character as u16)
    }

    pub fn inc_index(&mut self, diff: usize) {
        *self.block_scopes.last_mut().unwrap() += diff;
    }
}
