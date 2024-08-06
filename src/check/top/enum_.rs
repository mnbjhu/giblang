use crate::{check::CheckState, parser::top::enum_::Enum, project::Project};

impl Enum {
    pub fn check(&self, project: &Project, state: &mut CheckState) {
        self.generics.0.check(project, state);
        for member in &self.members {
            member.0.body.check(project, state);
        }
    }
}
