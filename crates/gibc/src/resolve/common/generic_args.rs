use crate::{parser::common::generic_args::GenericArgs, resolve::state::ResolveState, ty::Generic};

impl GenericArgs {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> Vec<Generic<'db>> {
        self.0.iter().map(|(g, _)| g.resolve(state)).collect()
    }
}
