use crate::{
    db::{decl::DeclKind, path::ModulePath},
    parser::top::trait_::Trait,
    resolve::state::ResolveState,
    ty::{Named, Ty},
};

use super::Decl;

impl Trait {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> Decl<'db> {
        let generics = self.generics.0.resolve(state);
        state.add_self_ty(
            Ty::Named(Named {
                name: ModulePath::new(state.db, state.path.clone()),
                args: generics.iter().map(|g| Ty::Generic(g.clone())).collect(),
            }),
            self.name.1,
        );
        let name = self.name.clone();
        let mut body = Vec::new();
        for func in &self.body {
            state.enter_scope();
            state.path.push(func.0.name.0.clone());
            let decl = func.0.resolve(state);
            body.push(decl);
            state.path.pop();
            state.exit_scope();
        }
        let kind = DeclKind::Trait { generics, body };
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
