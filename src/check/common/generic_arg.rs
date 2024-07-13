use crate::{
    check::{ty::Ty, CheckState, NamedExpr},
    fs::project::Project,
    parser::common::generic_arg::GenericArg,
};

impl GenericArg {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        print_errors: bool,
    ) {
        let super_ = if let Some((super_, _)) = &self.super_ {
            super_.check(project, state, print_errors)
        } else {
            Ty::Any
        };
        state.insert(
            self.name.0.clone(),
            NamedExpr::GenericArg {
                super_,
                variance: self.variance,
            },
        )
    }
}
