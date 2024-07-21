use crate::{check::CheckState, parser::top::struct_::Struct, project::Project};

impl Struct {
    pub fn check(&self, project: &Project, state: &mut CheckState) {
        self.generics.0.check(project, state, true);
        self.body.check(project, state)
    }
}
