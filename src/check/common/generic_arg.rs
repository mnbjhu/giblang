use crate::{
    check::CheckState,
    parser::common::generic_arg::GenericArg,
    project::Project,
    ty::{Generic, Ty},
};

impl GenericArg {
    pub fn check(&self, project: &Project, state: &mut CheckState, print_errors: bool) -> Ty {
        let super_ = if let Some((super_, _)) = &self.super_ {
            super_.check(project, state, print_errors)
        } else {
            Ty::Any
        };

        Ty::Generic(Generic {
            name: self.name.0.clone(),
            variance: self.variance,
            super_: Box::new(super_),
        })
    }
}
