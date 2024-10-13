use chumsky::container::Container;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::struct_field::StructField,
};

impl AstItem for StructField {
    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.ty.1.contains_offset(offset) {
            return self.ty.0.at_offset(state, offset);
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Param,
        });
        self.ty.0.tokens(state, tokens);
    }
}
