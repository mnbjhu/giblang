use crate::{check::CheckState, fs::project::Project, parser::common::generic_args::GenericArgs};

impl GenericArgs {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        print_errors: bool,
    ) {
        for (arg, _) in &self.0 {
            arg.check(project, state, print_errors)
        }
    }
}
