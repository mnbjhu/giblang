use crate::{check::state::CheckState, parser::common::generic_args::GenericArgs, ty::Generic};

impl GenericArgs {
    pub fn resolve(&self, state: &mut CheckState<'_>) -> Vec<Generic> {
        self.0.iter().map(|(g, _)| g.resolve(state)).collect()
    }
}
