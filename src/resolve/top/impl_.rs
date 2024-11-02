use crate::{db::decl::impl_::ImplForDecl, parser::top::impl_::Impl, resolve::state::ResolveState};

impl Impl {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> ImplForDecl<'db> {
        let generics = self.generics.0.resolve(state);
        let to = self.trait_.as_ref().map(|trait_| trait_.0.resolve(state));
        let from = self.for_.0.resolve(state);
        state.add_self_ty(from.clone(), self.for_.1);
        let mut functions = Vec::new();
        for func in &self.body {
            state.enter_scope();
            let decl = func.0.resolve(state);
            functions.push(decl);
            state.exit_scope();
        }
        ImplForDecl::new(state.db, generics, from, to, functions)
    }
}
