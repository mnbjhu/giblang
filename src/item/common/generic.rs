use std::collections::HashMap;

use crate::{
    check::{state::CheckState, TokenKind},
    item::AstItem,
    parser::common::generic_arg::GenericArg,
    ty::Ty,
};

impl AstItem for GenericArg {
    fn item_name(&self) -> &'static str {
        "generic_arg"
    }
    fn hover(
        &self,
        state: &mut CheckState,
        _: usize,
        _: &HashMap<u32, Ty<'_>>,
        _: &Ty<'_>,
    ) -> Option<String> {
        Some(state.get_generic(&self.name.0).unwrap().hover(state))
    }

    fn tokens(
        &self,
        _: &mut CheckState,
        tokens: &mut Vec<crate::check::SemanticToken>,
        _: &Ty<'_>,
    ) {
        tokens.push(crate::check::SemanticToken {
            span: self.name.1,
            kind: TokenKind::Generic,
        });
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        if let Some(super_) = &self.super_ {
            allocator
                .text(&self.name.0)
                .append(": ")
                .append(allocator.space())
                .append(super_.0.pretty(allocator))
        } else {
            allocator.text(&self.name.0)
        }
    }
}
