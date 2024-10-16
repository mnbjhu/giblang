use std::collections::HashMap;

use crate::{
    check::state::CheckState,
    project::decl::{Decl, DeclKind},
    ty::Ty,
};

impl Decl<'_> {
    pub fn hover(&self, state: &mut CheckState, type_vars: &HashMap<u32, Ty>) -> String {
        let path_name = self.path(state.db).name(state.db).join("::");
        let kind = match self.kind(state.db) {
            DeclKind::Struct { .. } => "struct",
            DeclKind::Trait { .. } => "trait",
            DeclKind::Enum { .. } => "enum",
            DeclKind::Member { .. } => "member",
            DeclKind::Function { .. } => "function",
            DeclKind::Prim(_) => "primitive",
        };
        format!("{kind} {path_name}")
    }
}
