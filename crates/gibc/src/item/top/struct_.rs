use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::state::CheckState, item::AstItem, parser::top::struct_::Struct,
    range::span_to_range_str, util::Span,
};

impl AstItem for Struct {
    fn item_name(&self) -> &'static str {
        "struct"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text("struct")
            .append(allocator.space())
            .append(self.name.0.clone())
            .append(self.generics.0.pretty(allocator))
            .append(self.body.0.pretty(allocator))
    }
}

impl Struct {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> DocumentSymbol {
        let txt = state.file_data.text(state.db);
        let range = span_to_range_str(span.into(), txt);
        let selection_range = span_to_range_str(self.name.1.into(), txt);
        let _ = self.generics.0.check(state);
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
