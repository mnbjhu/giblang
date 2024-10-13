use chumsky::container::Container;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::struct_::Struct,
};

impl AstItem for Struct {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        if self.body.1.contains_offset(offset) {
            return self.body.0.at_offset(state, offset);
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Struct,
        });
        self.generics.0.tokens(state, tokens);
        self.body.0.tokens(state, tokens);
    }
}
