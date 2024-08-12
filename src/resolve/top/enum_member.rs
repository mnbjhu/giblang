use crate::{
    parser::top::enum_member::EnumMember, project::decl::Decl, resolve::state::ResolveState,
};

impl EnumMember {
    pub fn resolve(&self, state: &mut ResolveState) -> Decl {
        Decl::Member {
            name: self.name.clone(),
            body: self.body.resolve(state),
        }
    }
}
