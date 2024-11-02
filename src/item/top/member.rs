use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::top::enum_member::EnumMember,
    range::span_to_range_str,
    ty::Ty,
    util::Span,
};

impl AstItem for EnumMember {
    fn tokens(&self, _: &mut CheckState, tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Member,
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
            .text(self.name.0.clone())
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
