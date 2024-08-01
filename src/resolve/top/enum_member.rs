use crate::{check::state::CheckState, parser::top::enum_member::EnumMember};

use super::Decl;

impl EnumMember {
    pub fn resolve(&self, state: &mut CheckState) -> Decl {
        Decl::Member {
            name: self.name.clone(),
            body: self.body.resolve(state),
        }
    }
}
