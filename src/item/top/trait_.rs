use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind}, db::path::ModulePath, item::{common::type_::ContainsOffset, AstItem}, parser::top::trait_::Trait, range::span_to_range_str, ty::Ty, util::Span
};

use super::impl_::pretty_trait_body;

impl AstItem for Trait {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        let args = self.generics.0.check(state);
        let name = ModulePath::new(state.db, state.path.clone());
        let self_ty = Ty::Named { name, args };
        state.add_self_ty(&self_ty, self.name.1);
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
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
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
