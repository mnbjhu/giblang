use crate::{parser::stmt::let_::LetStatement, resolve_types::state::TypeResolveState};

impl LetStatement {
    pub fn type_resolve(&self, state: &mut TypeResolveState) {
        if let Some(expected) = self.ty.as_ref() {
            let ty = expected.0.type_resolve(&state);
            let id = state.instantiate(ty);
            state.expect_type_bound(id, ty);
        }
    }
}
