use crate::{
    check::{CheckState, NamedExpr},
    fs::project::Project,
    parser::common::generic_arg::GenericArg,
    ty::Ty,
};

impl GenericArg {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        print_errors: bool,
    ) -> Ty<'module> {
        let super_ = if let Some((super_, _)) = &self.super_ {
            super_.check(project, state, print_errors)
        } else {
            Ty::Any
        };
        state.insert(
            self.name.0.clone(),
            NamedExpr::GenericArg {
                name: self.name.0.clone(),
                super_: super_.clone(),
                variance: self.variance,
            },
        );
        Ty::Generic {
            name: self.name.0.clone(),
            variance: self.variance,
            super_: Box::new(super_),
        }
    }
}
