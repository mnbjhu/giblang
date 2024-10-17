use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::enum_member::EnumMember,
    range::span_to_range_str,
    util::Span,
};

impl AstItem for EnumMember {
    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.body.1.contains_offset(offset) {
            self.body.0.at_offset(state, offset)
        } else {
            self
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Member,
        });
        self.body.0.tokens(state, tokens);
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text(self.name.0.clone())
            .append(allocator.space())
            .append(self.body.0.pretty(allocator))
    }
}
impl EnumMember {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> DocumentSymbol {
        let txt = state.file_data.text(state.db);
        let range = span_to_range_str(span.into(), txt);
        let selection_range = span_to_range_str(self.name.1.into(), txt);
        DocumentSymbol {
            name: self.name.0.clone(),
            detail: Some("member".to_string()),
            kind: SymbolKind::ENUM_MEMBER,
            range,
            selection_range,
            children: Some(self.body.0.document_symbols(state)),
            tags: None,
            deprecated: None,
        }
    }
}
