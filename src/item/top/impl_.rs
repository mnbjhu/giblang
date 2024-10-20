use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::state::CheckState,
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::impl_::Impl,
    range::span_to_range_str,
    util::{Span, Spanned},
};

impl AstItem for Impl {
    fn tokens(
        &self,
        state: &mut crate::check::state::CheckState,
        tokens: &mut Vec<crate::check::SemanticToken>,
    ) {
        self.generics.0.tokens(state, tokens);
        self.for_.0.tokens(state, tokens);
        self.trait_.0.tokens(state, tokens);
        for (func, _) in &self.body {
            func.tokens(state, tokens);
        }
    }

    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        self.generics.0.check(state);
        if self.for_.1.contains_offset(offset) {
            return self.for_.0.at_offset(state, offset);
        }
        let for_ = self.for_.0.check(state);
        state.add_self_ty(for_, self.for_.1);
        if self.trait_.1.contains_offset(offset) {
            return self.trait_.0.at_offset(state, offset);
        }
        self.trait_.0.check(state);
        for (func, span) in &self.body {
            if span.contains_offset(offset) {
                return func.at_offset(state, offset);
            }
            func.check(state);
        }
        self
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text("impl")
            .append(self.generics.0.pretty(allocator))
            .append(allocator.space())
            .append(self.trait_.0.pretty(allocator))
            .append(allocator.space())
            .append("for")
            .append(allocator.space())
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
        self.generics.0.check(state);
        let trait_ = self.trait_.0.check(state).get_name(state);
        let for_ = self.for_.0.check(state).get_name(state);
        let name = format!("impl {trait_} for {for_}");
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
