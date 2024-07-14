use crate::{check::CheckState, fs::project::Project, parser::top::struct_::Struct};

impl Struct {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        self.generics.0.check(project, state, true);
        self.body.check(project, state)
    }
}
