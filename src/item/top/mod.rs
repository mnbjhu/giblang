use async_lsp::lsp_types::DocumentSymbol;

use crate::{
    check::{state::CheckState, SemanticToken},
    db::input::{Db, SourceFile},
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
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        state.enter_scope();
        match self {
            Top::Func(f) => f.at_offset(state, offset),
            Top::Struct(s) => s.at_offset(state, offset),
            Top::Enum(e) => e.at_offset(state, offset),
            Top::Trait(t) => t.at_offset(state, offset),
            Top::Impl(i) => i.at_offset(state, offset),
            Top::Use(u) => u.at_offset(state, offset),
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        state.enter_scope();
        match self {
            Top::Func(f) => f.tokens(state, tokens),
            Top::Struct(s) => s.tokens(state, tokens),
            Top::Enum(e) => e.tokens(state, tokens),
            Top::Trait(t) => t.tokens(state, tokens),
            Top::Impl(i) => i.tokens(state, tokens),
            Top::Use(u) => {
                u.tokens(state, tokens);
                state.import(u);
            }
        }
        state.exit_scope();
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
                state.import(u);
                None
            }
        };
        state.exit_scope();
        found
    }
}
