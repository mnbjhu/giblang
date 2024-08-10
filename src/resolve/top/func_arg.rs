use crate::{
    parser::top::arg::FunctionArg, resolve::state::ResolveState, ty::Ty,
};

impl FunctionArg {
    pub fn resolve(&self, state: &mut ResolveState) -> (String, Ty) {
        (self.name.0.clone(), self.ty.0.resolve(state))
    }
}
