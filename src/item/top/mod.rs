use async_lsp::lsp_types::DocumentSymbol;

use crate::{
    check::state::CheckState, parser::top::Top, util::Span
};

use super::AstItem;

pub mod enum_;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod member;
pub mod struct_;
pub mod struct_body;
pub mod struct_field;
pub mod trait_;
pub mod use_;

impl AstItem for Top {

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Top::Struct(s) => s.pretty(allocator),
            Top::Func(f) => f.pretty(allocator),
            Top::Enum(e) => e.pretty(allocator),
            Top::Trait(t) => t.pretty(allocator),
            Top::Impl(i) => i.pretty(allocator),
            Top::Use(u) => allocator
                .text("use")
                .append(allocator.space())
                .append(u.pretty(allocator)),
        }
    }
}

impl Top {
    pub fn document_symbol(&self, state: &mut CheckState, span: Span) -> Option<DocumentSymbol> {
        state.enter_scope();
        let found = match self {
            Top::Func(f) => Some(f.document_symbol(state, span)),
            Top::Struct(s) => Some(s.document_symbol(state, span)),
            Top::Enum(e) => Some(e.document_symbol(state, span)),
            Top::Trait(t) => Some(t.document_symbol(state, span)),
            Top::Impl(i) => Some(i.document_symbol(state, span)),
            Top::Use(u) => {
                let _ = state.import(u);
                None
            }
        };
        state.exit_scope();
        found
    }
}
