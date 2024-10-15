use std::collections::HashMap;

use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::enum_::Enum,
    range::span_to_range_str,
    ty::Ty,
    util::Span,
};

impl AstItem for Enum {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        for member in &self.members {
            if member.1.contains_offset(offset) {
                return member.0.at_offset(state, offset);
            }
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

impl Enum {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> DocumentSymbol {
        let txt = state.file_data.text(state.db);
        let range = span_to_range_str(span.into(), txt);
        let selection_range = span_to_range_str(self.name.1.into(), txt);
        self.generics.0.check(state);
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
