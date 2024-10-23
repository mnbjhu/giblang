use std::collections::HashMap;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::struct_field::StructField,
    ty::Ty,
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
            kind: TokenKind::Property,
        });
        self.ty.0.tokens(state, tokens);
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        _: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> Option<String> {
        let ty = self.ty.0.check(state);
        Some(format!(
            "{}: {}",
            self.name.0,
            ty.get_name(state, Some(type_vars))
        ))
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text(&self.name.0)
            .append(":")
            .append(allocator.space())
            .append(self.ty.0.pretty(allocator))
    }
}
