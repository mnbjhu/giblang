use crate::{check::state::CheckState, parser::top::arg::FunctionArg, ty::Ty};

impl FunctionArg {
    pub fn resolve(&self, state: &mut CheckState) -> (String, Ty) {
        (self.name.0.clone(), self.ty.0.resolve(state))
    }
}
