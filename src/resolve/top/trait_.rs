use std::collections::HashMap;

use crate::{check::state::CheckState, parser::top::trait_::Trait};

use super::Decl;

impl Trait {
    pub fn resolve(&self, state: &mut CheckState, decls: &mut HashMap<u32, Decl>) -> Decl {
        let generics = self.generics.resolve(state);
        let name = self.name.clone();
        let mut body = Vec::new();
        for func in &self.body {
            state.enter_scope();
            let id = func.0.id;
            let decl = func.0.resolve(state);
            decls.insert(id, decl);
            body.push(id);
            state.exit_scope();
        }
        Decl::Trait {
            name,
            generics,
            body,
        }
    }
}
