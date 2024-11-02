use std::collections::HashMap;

use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{
        common::generics::braces,
        AstItem,
    },
    parser::top::enum_::Enum,
    range::span_to_range_str,
    ty::Ty,
    util::Span,
};

impl AstItem for Enum {
    fn hover(&self, _: &mut CheckState, _: usize, _: &HashMap<u32, Ty<'_>>, _: &Ty<'_>) -> Option<String> {
        Some(format!("Enum {}", self.name.0))
    }
fn tokens(&self, _: &mut CheckState, tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Enum,
        });
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text("enum")
            .append(allocator.space())
            .append(self.name.0.clone())
            .append(allocator.space())
            .append(self.generics.0.pretty(allocator))
            .append(allocator.space())
            .append(braces(allocator, &self.members))
    }
}

impl Enum {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> DocumentSymbol {
        let txt = state.file_data.text(state.db);
        let range = span_to_range_str(span.into(), txt);
        let selection_range = span_to_range_str(self.name.1.into(), txt);
        DocumentSymbol {
            name: self.name.0.clone(),
            detail: Some("enum".to_string()),
            kind: SymbolKind::ENUM,
            range,
            selection_range,
            children: Some(
                self.members
                    .iter()
                    .map(|member| member.0.document_symbol(state, member.1))
                    .collect(),
            ),
            tags: None,
            deprecated: None,
        }
    }
}
