use crate::{check::state::CheckState, parser::common::generic_args::GenericArgs, ty::Ty};

impl<'db> GenericArgs {
    pub fn check(&self, state: &mut CheckState<'_, 'db>) -> Vec<Ty<'db>> {
        let mut args = vec![];
        for (arg, _) in &self.0 {
            args.push(arg.check(state));
        }
        args
    }
}
