use std::collections::HashMap;

use async_lsp::lsp_types::{DocumentSymbol, SymbolKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{
        common::{generics::brackets, type_::ContainsOffset},
        expr::pretty_codeblock,
        AstItem,
    },
    parser::top::func::Func,
    range::span_to_range_str,
    ty::Ty,
    util::Span,
};

impl AstItem for Func {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        self.generics.0.check(state);
        for arg in &self.args {
            if arg.1.contains_offset(offset) {
                return arg.0.at_offset(state, offset);
            }
            arg.0.check(state);
        }
        if let Some(rec) = &self.receiver {
            if rec.1.contains_offset(offset) {
                return rec.0.at_offset(state, offset);
            }
        }
        if let Some(ret) = &self.ret {
            if ret.1.contains_offset(offset) {
                return ret.0.at_offset(state, offset);
            }
        }
        if let Some(body) = &self.body {
            for stmt in body {
                if stmt.1.contains_offset(offset) {
                    return stmt.0.at_offset(state, offset);
                }
                stmt.0.check(state);
            }
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Func,
        });
        self.generics.0.tokens(state, tokens);
        self.generics.0.check(state);
        for arg in &self.args {
            arg.0.tokens(state, tokens);
            arg.0.check(state);
        }
        if let Some(rec) = &self.receiver {
            rec.0.tokens(state, tokens);
        }
        if let Some(ret) = &self.ret {
            ret.0.tokens(state, tokens);
        }
        if let Some(body) = &self.body {
            for stmt in body {
                stmt.0.tokens(state, tokens);
                stmt.0.check(state);
            }
        }
    }

    fn hover(&self, _: &mut CheckState, _: usize, _: &HashMap<u32, Ty<'_>>) -> Option<String> {
        Some(format!("Function {}", self.name.0))
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let rec = match &self.receiver {
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
            .append(rec)
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
        self.generics.0.check(state);
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
