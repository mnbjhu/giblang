use crate::{check::state::CheckState, parser::top::struct_::Struct};

impl Struct {
    pub fn build(&self, state: &mut CheckState) -> String {
        format!("type {} struct {}", self.name.0, self.body.build(state))
    }
}
