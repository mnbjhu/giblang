use crate::{
    db::modules::{Module, ModuleData, ModulePath},
    parser::top::trait_::Trait,
    project::decl::DeclKind,
    resolve::state::ResolveState,
    ty::Ty,
};

use super::Decl;

impl Trait {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> Decl<'db> {
        let generics = self.generics.resolve(state);
        let mut self_path = state.file_data.module_path(state.db).name(state.db).clone();
        self_path.push(self.name.0.to_string());
        state.add_self_ty(
            Ty::Named {
                name: ModulePath::new(state.db, self_path),
                args: generics.iter().map(|g| Ty::Generic(g.clone())).collect(),
            },
            self.name.1,
        );
        let name = self.name.clone();
        let mut body = Vec::new();
        for func in &self.body {
            state.enter_scope();
            let decl = func.0.resolve(state);
            body.push(Module::new(
                state.db,
                decl.name(state.db),
                ModuleData::Export(decl),
            ));
            state.exit_scope();
        }
        let kind = DeclKind::Trait { generics, body };
        Decl::new(state.db, name.0, name.1, kind)
    }
}
