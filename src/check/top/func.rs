use crate::{check::CheckState, fs::project::Project, parser::top::func::Func};

impl Func {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        self.generics.check(project, state, true);
    }
}
