use crate::{
    db::decl::{Decl, DeclKind}, parser::top::enum_::Enum, resolve::state::ResolveState
};

impl Enum {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> Decl<'db> {
        let generics = self.generics.0.resolve(state);
        let mut variants = vec![];
        for m in &self.members {
            state.path.push(m.0.name.0.clone());
            let decl = m.0.resolve(state);
            variants.push(decl);
            state.path.pop();
        }
        let kind = DeclKind::Enum { generics, variants };
        Decl::new(
            state.db,
            self.name.0.clone(),
            self.name.1,
            kind,
            Some(state.file_data),
            state.module_path(),
        )
    }
}
