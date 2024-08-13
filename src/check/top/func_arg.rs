use crate::{check::state::CheckState, parser::top::arg::FunctionArg};

impl FunctionArg {
    pub fn check(&self, state: &mut CheckState) {
        let ty = self.ty.0.check(state);
        state.insert_variable(self.name.0.clone(), ty);
    }
}
