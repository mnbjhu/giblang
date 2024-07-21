use std::collections::HashMap;

use crate::{check::state::CheckState, parser::top::enum_::Enum};

use super::Decl;

impl Enum {
    pub fn resolve(&self, state: &mut CheckState, decls: &mut HashMap<u32, Decl>) -> Decl {
        let generics = self.generics.0.resolve(state);
        let mut variants = vec![];
        for m in &self.members {
            let id = m.0.id;
            let decl = m.0.resolve(state);
            decls.insert(id, decl);
            variants.push(id)
        }
        Decl::Enum {
            name: self.name.clone(),
            generics,
            variants,
        }
    }
}
