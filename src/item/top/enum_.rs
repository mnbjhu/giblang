use std::collections::HashMap;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::top::enum_::Enum,
    ty::Ty,
};

impl AstItem for Enum {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.start <= offset && offset <= self.generics.1.end {
            return self.generics.0.at_offset(state, offset);
        }
        self
    }

    fn hover(&self, _: &mut CheckState, _: usize, _: &HashMap<u32, Ty<'_>>) -> Option<String> {
        Some(format!("Enum {}", self.name.0))
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Enum,
        });
        self.generics.0.tokens(state, tokens);
        for member in &self.members {
            member.0.tokens(state, tokens);
        }
    }
}
