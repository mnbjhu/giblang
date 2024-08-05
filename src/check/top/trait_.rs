use crate::{
    check::CheckState,
    parser::{common::variance::Variance, top::trait_::Trait},
    project::Project,
    ty::{Generic, Ty},
};

impl<'proj> Trait {
    pub fn check(&'proj self, project: &'proj Project, state: &mut CheckState<'proj>) {
        let args = self.generics.check(project, state);
        state.add_self_ty(Ty::Named {
            name: self.id,
            args,
        });
        for func in &self.body {
            state.enter_scope();
            func.0.check(project, state);
            state.exit_scope();
        }
    }
}

impl CheckState<'_> {
    pub fn add_self_ty(&mut self, super_: Ty) {
        self.insert_generic(
            "Self".to_string(),
            Generic {
                name: "Self".to_string(),
                variance: Variance::Invariant,
                super_: Box::new(super_),
            },
        );
    }
}
