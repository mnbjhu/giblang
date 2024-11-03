use std::collections::HashMap;

use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::generics::brackets, expr::pretty_codeblock, AstItem},
    parser::top::func::Func,
    range::span_to_range_str,
    ty::Ty,
    util::Span,
};

impl AstItem for Func {
    fn item_name(&self) -> &'static str {
        "func"
    }
    fn tokens(&self, _: &mut CheckState, tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Func,
        });
    }

    fn hover(
        &self,
        _: &mut CheckState,
        _: usize,
        _: &HashMap<u32, Ty<'_>>,
        _: &Ty<'_>,
    ) -> Option<String> {
        Some(format!("Function {}", self.name.0))
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let receiver = match &self.receiver {
            Some(rec) => rec.0.pretty(allocator).append(allocator.text(".")),
            None => allocator.nil(),
        };
        let ret = match &self.ret {
            Some(ret) => allocator
                .text(":")
                .append(allocator.space())
                .append(ret.0.pretty(allocator)),
            None => allocator.nil(),
        };

        let body = match &self.body {
            Some(body) => allocator.space().append(pretty_codeblock(allocator, body)),
            None => allocator.nil(),
        };
        allocator
            .text("fn")
            .append(allocator.space())
            .append(receiver)
            .append(self.name.0.clone())
            .append(self.generics.0.pretty(allocator))
            .append(brackets(allocator, "(", ")", &self.args))
            .append(ret)
            .append(body)
    }
}
impl Func {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> DocumentSymbol {
        let txt = state.file_data.text(state.db);
        let range = span_to_range_str(span.into(), txt);
        let selection_range = span_to_range_str(self.name.1.into(), txt);
        DocumentSymbol {
            name: self.name.0.clone(),
            detail: Some("function".to_string()),
            kind: SymbolKind::FUNCTION,
            range,
            selection_range,
            children: None,
            tags: None,
            deprecated: None,
        }
    }
}
