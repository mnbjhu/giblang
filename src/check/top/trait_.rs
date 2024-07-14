use crate::{
    check::{ty::Ty, CheckState, NamedExpr},
    fs::project::Project,
    parser::{common::variance::Variance, top::trait_::Trait},
};

impl Trait {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        let args = self.generics.check(project, state, true);
        if let NamedExpr::Imported(trait_, _) = state.get_name(&self.name.0) {
            state.insert(
                "Self".to_string(),
                NamedExpr::GenericArg {
                    name: "Self".to_string(),
                    super_: Ty::Named { name: trait_, args },
                    variance: Variance::Invariant,
                },
            )
        }
        for func in &self.body {
            state.enter_scope();
            func.0.check(project, state);
            state.exit_scope();
        }
    }
}
