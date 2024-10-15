use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::struct_::Struct,
    range::span_to_range_str,
    util::Span,
};

impl AstItem for Struct {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        self.generics.0.check(state);
        if self.body.1.contains_offset(offset) {
            return self.body.0.at_offset(state, offset);
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Struct,
        });
        self.generics.0.tokens(state, tokens);
        self.body.0.tokens(state, tokens);
    }
}

impl Struct {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> DocumentSymbol {
        let txt = state.file_data.text(state.db);
        let range = span_to_range_str(span.into(), txt);
        let selection_range = span_to_range_str(self.name.1.into(), txt);
        self.generics.0.check(state);
        DocumentSymbol {
            name: self.name.0.clone(),
            detail: Some("struct".to_string()),
            kind: SymbolKind::STRUCT,
            range,
            selection_range,
            children: Some(self.body.0.document_symbols(state)),
            tags: None,
            deprecated: None,
        }
    }
}
