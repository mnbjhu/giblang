use async_lsp::lsp_types::DocumentSymbol;

use crate::{
    check::{state::CheckState},
    parser::top::Top,
    util::Span,
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
            Top::Struct(s) => s.0.pretty(allocator),
            Top::Func(f) => f.0.pretty(allocator),
            Top::Enum(e) => e.0.pretty(allocator),
            Top::Trait(t) => t.0.pretty(allocator),
            Top::Impl(i) => i.0.pretty(allocator),
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
            Top::Func(f) => Some(f.0.document_symbol(state, span)),
            Top::Struct(s) => Some(s.0.document_symbol(state, span)),
            Top::Enum(e) => Some(e.0.document_symbol(state, span)),
            Top::Trait(t) => Some(t.0.document_symbol(state, span)),
            Top::Impl(i) => Some(i.0.document_symbol(state, span)),
            Top::Use(u) => {
                let _ = state.import(u);
                None
            }
        };
        state.exit_scope();
        found
    }
}
