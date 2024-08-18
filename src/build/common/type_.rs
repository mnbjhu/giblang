use crate::{check::state::CheckState, parser::common::type_::Type};

impl Type {
    pub fn build(&self, state: &mut CheckState) -> String {
        let ty = self.check(state);
        ty.build(state)
    }
}
