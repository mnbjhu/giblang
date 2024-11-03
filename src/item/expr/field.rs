use std::{collections::HashMap, ops::ControlFlow};

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{state::CheckState, Check, SemanticToken, TokenKind},
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
        let ty = self.check(state, &mut (), Span::splat(offset), ());
        if let ControlFlow::Continue(ty) = ty {
            return Some(ty.get_name(state, Some(type_vars)));
        }
        panic!("Unexpected ControlFlow::Break in Field::hover");
    }

    fn completions(
        &self,
        state: &mut CheckState,
        _: usize,
        type_vars: &HashMap<u32, Ty<'_>>,
        _: &Ty<'_>,
    ) -> Vec<CompletionItem> {
        let ControlFlow::Continue(rec) = self.struct_.0.check(state, &mut (), self.struct_.1, ())
        else {
            panic!("Unexpected ControlFlow::Break in Field::completions");
        };
        let mut completions = Vec::new();
        for (name, func_ty) in rec.member_funcs(state, self.name.1) {
            completions.push(CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(func_ty.get_name(state, Some(type_vars))),
                ..Default::default()
            });
        }
        for (name, ty) in rec.fields(state) {
            completions.push(CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(ty.get_name(state, Some(type_vars))),
                ..Default::default()
            });
        }
        completions
    }
}
