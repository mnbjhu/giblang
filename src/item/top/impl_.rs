use crate::{
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::impl_::Impl,
};

impl AstItem for Impl {
    fn tokens(
        &self,
        state: &mut crate::check::state::CheckState,
        tokens: &mut Vec<crate::check::SemanticToken>,
    ) {
        self.generics.0.tokens(state, tokens);
        self.for_.0.tokens(state, tokens);
        self.trait_.0.tokens(state, tokens);
        for (func, _) in &self.body {
            func.tokens(state, tokens);
        }
    }

    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        self.generics.0.check(state);
        if self.for_.1.contains_offset(offset) {
            return self.for_.0.at_offset(state, offset);
        }
        if self.trait_.1.contains_offset(offset) {
            return self.trait_.0.at_offset(state, offset);
        }
        let for_ = self.for_.0.check(state);
        state.add_self_ty(for_, self.for_.1);
        for (func, span) in &self.body {
            if span.contains_offset(offset) {
                return func.at_offset(state, offset);
            }
        }
        self
    }
}
