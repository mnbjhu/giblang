use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::trait_::Trait,
    range::span_to_range_str,
    util::Span,
};

impl AstItem for Trait {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        for (func, span) in &self.body {
            if span.contains_offset(offset) {
                return func.at_offset(state, offset);
            }
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Trait,
        });
        self.generics.0.tokens(state, tokens);
        for func in &self.body {
            func.0.tokens(state, tokens);
        }
    }
}

impl Trait {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> DocumentSymbol {
        let txt = state.file_data.text(state.db);
        let range = span_to_range_str(span.into(), txt);
        let selection_range = span_to_range_str(self.name.1.into(), txt);
        self.generics.0.check(state);
        DocumentSymbol {
            name: self.name.0.clone(),
            detail: Some("trait".to_string()),
            kind: SymbolKind::INTERFACE,
            range,
            selection_range,
            children: Some(
                self.body
                    .iter()
                    .map(|(func, span)| func.document_symbol(state, *span))
                    .collect(),
            ),
            tags: None,
            deprecated: None,
        }
    }
}
