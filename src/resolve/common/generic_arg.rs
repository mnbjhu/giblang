use crate::{
    check::state::CheckState,
    parser::common::generic_arg::GenericArg,
    ty::{Generic, Ty},
};

impl GenericArg {
    pub fn resolve(&self, state: &mut CheckState<'_>) -> Generic {
        let res = Generic {
            name: self.name.0.to_string(),
            variance: self.variance,
            super_: Box::new(
                self.super_
                    .as_ref()
                    .map_or(Ty::Any, |(ty, _)| ty.resolve(state)),
            ),
        };
        state.insert_generic(self.name.0.to_string(), res.clone());
        res
    }
}
