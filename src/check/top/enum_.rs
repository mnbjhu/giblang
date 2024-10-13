use crate::{check::state::CheckState, parser::top::enum_::Enum};

impl Enum {
    pub fn check(&self, state: &mut CheckState) {
        self.generics.0.check(state);
        state.path.push(self.name.0.to_string());
        for member in &self.members {
            member.0.body.0.check(state);
        }
        state.path.pop();
    }
}
