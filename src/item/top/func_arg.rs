use std::collections::HashMap;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::arg::FunctionArg,
    ty::Ty,
};

impl AstItem for FunctionArg {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
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

    fn hover(&self, state: &mut CheckState, _: usize, _: &HashMap<u32, Ty<'_>>) -> Option<String> {
        let ty = self.ty.0.check(state);
        Some(format!("{}: {}", self.name.0, ty.get_name(state)))
    }
}
