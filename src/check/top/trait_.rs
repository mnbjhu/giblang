use crate::{
    check::state::CheckState,
    parser::{common::variance::Variance, top::trait_::Trait},
    ty::{Generic, Ty},
    util::Span,
};

impl<'db> Trait {
    pub fn check(&'db self, state: &mut CheckState<'db>) {
        let args = self.generics.0.check(state);
        let id = state.local_id(self.name.0.to_string());
        state.add_self_ty(&Ty::Named { name: id, args }, self.name.1);
        for func in &self.body {
            state.enter_scope();
            func.0.check(state, true);
            state.exit_scope();
        }
    }
}

impl<'db> CheckState<'db> {
    pub fn add_self_ty(&mut self, super_: &Ty<'db>, span: Span) {
        let generic = Generic {
            name: ("Self".to_string(), span),
            variance: Variance::Invariant,
            super_: Box::new(super_.clone()),
        };
        self.insert_generic("Self".to_string(), generic);
    }
    pub fn add_self_param(&mut self, super_: Ty<'db>, span: Span) {
        self.insert_variable("self".to_string(), super_, true, span);
    }
}
