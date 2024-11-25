use crate::{parser::top::arg::FunctionArg, resolve::state::ResolveState, ty::Ty};

impl FunctionArg {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> (String, Ty<'db>) {
        (self.name.0.clone(), self.ty.0.resolve(state))
    }
}
