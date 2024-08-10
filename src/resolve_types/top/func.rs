use crate::{parser::top::func::Func, resolve_types::state::TypeResolveState};

impl Func {
    pub fn type_resolve(&self, state: &mut TypeResolveState) {
        self.resolve(&mut state.resolve_state);
        for (stmt, _) in &self.body.unwrap_or_default() {
            stmt.type_resolve(state);
        }
    }
}
