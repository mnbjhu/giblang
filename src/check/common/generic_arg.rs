use crate::{
    check::state::CheckState,
    parser::common::generic_arg::GenericArg,
    ty::{Generic, Ty},
};

impl GenericArg {
    pub fn check<'db>(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        let super_ = if let Some((super_, _)) = &self.super_ {
            super_.check(state)
        } else {
            Ty::Any
        };

        let generic = Generic {
            name: self.name.clone(),
            variance: self.variance,
            super_: Box::new(super_),
        };
        state.insert_generic(self.name.0.to_string(), generic.clone());
        Ty::Generic(generic)
    }
}
