use crate::{
    check::CheckState,
    parser::{common::variance::Variance, top::trait_::Trait},
    ty::{Generic, Ty},
    util::Span,
};

impl<'proj> Trait {
    pub fn check(&'proj self, state: &mut CheckState<'proj>) {
        let args = self.generics.check(state);
        state.add_self_ty(
            Ty::Named {
                name: self.id,
                args,
            },
            self.name.1,
        );
        for func in &self.body {
            state.enter_scope();
            func.0.check(state);
            state.exit_scope();
        }
    }
}

impl CheckState<'_> {
    pub fn add_self_ty(&mut self, super_: Ty, span: Span) {
        self.insert_generic(
            "Self".to_string(),
            Generic {
                name: ("Self".to_string(), span),
                variance: Variance::Invariant,
                super_: Box::new(super_),
            },
        );
    }
}
