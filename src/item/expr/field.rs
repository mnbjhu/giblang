use std::{collections::HashMap, ops::ControlFlow};

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::expr::field::Field,
    ty::Ty,
    util::Span,
};

impl AstItem for Field {
    fn item_name(&self) -> &'static str {
        "field"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.struct_
            .0
            .pretty(allocator)
            .append(".")
            .append(&self.name.0)
    }

    fn tokens(&self, _: &mut CheckState, tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Property,
        });
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        offset: usize,
        type_vars: &std::collections::HashMap<u32, crate::ty::Ty<'db>>,
        _: &crate::ty::Ty<'_>,
    ) -> Option<String> {
        todo!();
        panic!("Unexpected ControlFlow::Break in Field::hover");
    }

    fn completions(
        &self,
        state: &mut CheckState,
        _: usize,
        type_vars: &HashMap<u32, Ty<'_>>,
        _: &Ty<'_>,
    ) -> Vec<CompletionItem> {
        todo!();
    }
}
