use crate::{check::state::CheckState, parser::top::enum_::Enum};

impl Enum {
    pub fn check(&self, state: &mut CheckState) {
        self.generics.0.check(state);
        for member in &self.members {
            member.0.body.check(state);
        }
    }
}
