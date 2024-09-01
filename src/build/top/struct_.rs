use crate::{check::state::CheckState, parser::top::struct_::Struct};

impl Struct {
    pub fn build(&self, state: &mut CheckState) -> String {
        format!("type T{} struct {}", self.id, self.body.build(state))
    }
}
