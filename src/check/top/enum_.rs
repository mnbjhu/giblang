use crate::{check::CheckState, fs::project::Project, parser::top::enum_::Enum};

impl Enum {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        self.generics.0.check(project, state, true);
    }
}
