use crate::{check::CheckState, parser::common::generic_args::GenericArgs, ty::Ty};

impl GenericArgs {
    pub fn check(&self, state: &mut CheckState) -> Vec<Ty> {
        let mut args = vec![];
        for (arg, _) in &self.0 {
            args.push(arg.check(state));
        }
        args
    }
}
