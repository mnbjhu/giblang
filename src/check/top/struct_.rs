use crate::{check::state::CheckState, parser::top::struct_::Struct};

impl Struct {
    pub fn check(&self, state: &mut CheckState) {
        self.generics.0.check(state);
        self.body.check(state);
    }
}
