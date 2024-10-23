use std::collections::HashMap;

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::expr::field::Field,
    ty::Ty,
};

impl AstItem for Field {
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

    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if offset < self.name.1.start {
            self.struct_.0.at_offset(state, offset)
        } else {
            self
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        self.struct_.0.tokens(state, tokens);
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Property,
        });
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        _: usize,
        type_vars: &std::collections::HashMap<u32, crate::ty::Ty<'db>>,
    ) -> Option<String> {
        let ty = self.check(state);
        Some(format!(
            "{}: {}",
            self.name.0,
            ty.get_name_with_types(state, type_vars)
        ))
    }

    fn completions(
        &self,
        state: &mut CheckState,
        _: usize,
        _: &HashMap<u32, Ty<'_>>,
    ) -> Vec<CompletionItem> {
        let rec = self.struct_.0.check(state);
        let mut completions = Vec::new();
        for (name, func_ty) in rec.member_funcs(state) {
            completions.push(CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(func_ty.get_name(state)),
                ..Default::default()
            });
        }
        for (name, ty) in rec.fields(state) {
            completions.push(CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(ty.get_name(state)),
                ..Default::default()
            });
        }
        completions
    }
}
