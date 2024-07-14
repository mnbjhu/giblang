use crate::{check::CheckState, fs::project::Project, parser::stmt::let_::LetStatement};

impl LetStatement {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        let ty = if let Some(expected) = &self.ty {
            let expected = expected.0.check(project, state, true);
            let actual = self.value.0.check(project, state);
            if !actual.is_instance_of(&expected, project) {
                state.error(
                    &format!("Expected value to be of type '{expected}' but found '{actual}'",),
                    self.value.1,
                )
            }
            expected
        } else {
            self.value.0.check(project, state)
        };
        self.pattern.0.check(project, state, ty);
    }
}
