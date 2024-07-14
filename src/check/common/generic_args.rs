use crate::{
    check::{ty::Ty, CheckState},
    fs::project::Project,
    parser::common::generic_args::GenericArgs,
};

impl GenericArgs {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        print_errors: bool,
    ) -> Vec<Ty<'module>> {
        let mut args = vec![];
        for (arg, _) in &self.0 {
            args.push(arg.check(project, state, print_errors))
        }
        args
    }
}
