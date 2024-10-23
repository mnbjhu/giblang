use crate::{
    parser::top::enum_member::EnumMember,
    project::decl::{Decl, DeclKind},
    resolve::state::ResolveState,
};

impl EnumMember {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> Decl<'db> {
        let kind = DeclKind::Member {
            body: self.body.0.resolve(state),
        };
        let name = self.name.clone();
        Decl::new(
            state.db,
            name.0,
            name.1,
            kind,
            Some(state.file_data),
            state.module_path(),
        )
    }
}
