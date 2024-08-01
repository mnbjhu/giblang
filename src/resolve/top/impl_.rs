use std::collections::HashMap;

use crate::{check::state::CheckState, parser::top::impl_::Impl, project::ImplData};

use super::Decl;

impl Impl {
    pub fn resolve(&self, state: &mut CheckState, decls: &mut HashMap<u32, Decl>) -> ImplData {
        let generics = self.generics.resolve(state);
        let to = self.trait_.0.resolve(state);
        let from = self.for_.0.resolve(state);
        let mut functions = Vec::new();
        for func in &self.body {
            state.enter_scope();
            let id = func.0.id;
            let decl = func.0.resolve(state);
            decls.insert(id, decl);
            functions.push(id);
            state.exit_scope();
        }
        ImplData {
            generics,
            from,
            to,
            functions,
        }
    }
}
