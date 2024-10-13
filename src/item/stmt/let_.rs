use crate::{
    check::{state::CheckState, SemanticToken},
    item::{common::type_::ContainsOffset as _, AstItem},
    parser::stmt::let_::LetStatement,
};

impl AstItem for LetStatement {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.value.1.contains_offset(offset) {
            return self.value.0.at_offset(state, offset);
        }
        self.check(state);
        if self.pattern.1.contains_offset(offset) {
            return self.pattern.0.at_offset(state, offset);
        }
        if let Some(ty) = &self.ty {
            if ty.1.contains_offset(offset) {
                return ty.0.at_offset(state, offset);
            }
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        self.pattern.0.tokens(state, tokens);
        if let Some(ty) = &self.ty {
            ty.0.tokens(state, tokens);
        }
        self.value.0.tokens(state, tokens);
    }
}
