use crate::{check::state::CheckState, parser::top::arg::FunctionArg, ty::Ty};

impl FunctionArg {
    pub fn check<'db>(&self, state: &mut CheckState<'db>) -> (String, Ty<'db>) {
        let ty = self.ty.0.check(state);
        state.insert_variable(self.name.0.clone(), ty.clone(), true, self.name.1);
        (self.name.0.clone(), ty)
    }
}
