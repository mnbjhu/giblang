use crate::{check::CheckState, fs::project::Project, parser::top::trait_::Trait};

impl Trait {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        self.generics.check(project, state, true);
    }
}
