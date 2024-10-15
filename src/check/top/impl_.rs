use crate::{check::state::CheckState, parser::top::impl_::Impl};

impl<'db> Impl {
    pub fn check(&'db self, state: &mut CheckState<'_, 'db>) {
        self.generics.0.check(state);
        let for_ = self.for_.0.check(state);
        self.trait_.0.check(state);
        state.add_self_ty(for_, self.for_.1);
        for func in &self.body {
            state.enter_scope();
            func.0.check(state);
            state.exit_scope();
        }
    }
}
