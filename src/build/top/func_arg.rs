use crate::{check::state::CheckState, parser::top::arg::FunctionArg};

impl FunctionArg {
    pub fn build(&self, state: &mut CheckState) -> String {
        format!("{}: {}", self.name.0, self.ty.0.build(state))
    }
}
