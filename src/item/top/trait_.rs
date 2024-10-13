use crate::{
    check::state::CheckState,
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::trait_::Trait,
};

impl AstItem for Trait {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        for (func, span) in &self.body {
            if span.contains_offset(offset) {
                return func.at_offset(state, offset);
            }
        }
        self
    }

    fn tokens(
        &self,
        state: &mut crate::check::state::CheckState,
        tokens: &mut Vec<crate::check::SemanticToken>,
    ) {
        self.generics.0.tokens(state, tokens);
        for arg in &self.body {
            arg.0.tokens(state, tokens);
        }
        for func in &self.body {
            func.0.tokens(state, tokens);
        }
    }
}
