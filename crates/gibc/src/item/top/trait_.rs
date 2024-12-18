use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::state::CheckState, item::AstItem, parser::top::trait_::Trait, range::span_to_range_str,
    util::Span,
};

use super::impl_::pretty_trait_body;

impl AstItem for Trait {
    fn item_name(&self) -> &'static str {
        "trait"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text("trait")
            .append(allocator.space())
            .append(self.name.0.clone())
            .append(self.generics.0.pretty(allocator))
            .append(allocator.space())
            .append(pretty_trait_body(allocator, &self.body))
    }
}

impl Trait {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> DocumentSymbol {
        let txt = state.file_data.text(state.db);
        let range = span_to_range_str(span.into(), txt);
        let selection_range = span_to_range_str(self.name.1.into(), txt);
        let _ = self.generics.0.check(state);
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
