use crate::{check::state::CheckState, parser::top::arg::FunctionArg, project::Project};

impl FunctionArg {
    pub fn check(&self, project: &Project, state: &mut CheckState) {
        let ty = self.ty.0.check(project, state, true);
        state.insert_variable(self.name.0.clone(), ty)
    }
}
