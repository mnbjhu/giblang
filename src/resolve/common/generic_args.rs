use crate::{parser::common::generic_args::GenericArgs, resolve::state::ResolveState, ty::Generic};

impl GenericArgs {
    pub fn resolve(&self, state: &mut ResolveState<'_>) -> Vec<Generic> {
        self.0.iter().map(|(g, _)| g.resolve(state)).collect()
    }
}
