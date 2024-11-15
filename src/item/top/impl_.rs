use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::state::CheckState,
    item::AstItem,
    parser::top::impl_::Impl,
    range::span_to_range_str,
    util::{Span, Spanned},
};

impl AstItem for Impl {
    fn item_name(&self) -> &'static str {
        "impl"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let trait_ = if let Some(trait_) = &self.trait_ {
            trait_
                .0
                .pretty(allocator)
                .append(allocator.space())
                .append("for")
                .append(allocator.space())
        } else {
            allocator.nil()
        };
        allocator
            .text("impl")
            .append(self.generics.0.pretty(allocator))
            .append(allocator.space())
            .append(trait_)
            .append(self.for_.0.pretty(allocator))
            .append(allocator.space())
            .append(pretty_trait_body(allocator, &self.body))
    }
}

pub fn pretty_trait_body<'b, D, A, T>(
    allocator: &'b D,
    items: &'b [Spanned<T>],
) -> pretty::DocBuilder<'b, D, A>
where
    D: pretty::DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
    T: AstItem,
{
    let separator = allocator.hardline();
    allocator
        .text("{")
        .append(
            allocator
                .hardline()
                .append(
                    allocator
                        .intersperse(items.iter().map(|(x, _)| x.pretty(allocator)), separator),
                )
                .nest(4),
        )
        .append(allocator.hardline())
        .append("}")
        .group()
}

impl Impl {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> DocumentSymbol {
        let txt = state.file_data.text(state.db);
        let range = span_to_range_str(span.into(), txt);
        let selection_range = span_to_range_str(self.for_.1.into(), txt);
        let mut symbols = Vec::new();
        let _ = self.generics.0.check(state);
        let name = if let Some(trait_) = self.trait_.as_ref() {
            let trait_ = trait_.0.check(state);
            let for_ = self.for_.0.check(state);
            let trait_ = trait_.ty.get_name(state, None);
            let for_ = for_.ty.get_name(state, None);
            format!("impl {trait_} for {for_}")
        } else {
            let for_ = self.for_.0.check(state);
            let for_ = for_.ty.get_name(state, None);
            format!("impl for {for_}")
        };
        for (func, span) in &self.body {
            symbols.push(func.document_symbol(state, *span));
        }
        DocumentSymbol {
            name,
            detail: None,
            kind: SymbolKind::CLASS,
            range,
            selection_range,
            children: Some(symbols),
            tags: None,
            deprecated: None,
        }
    }
}
