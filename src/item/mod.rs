use std::{collections::HashMap, fmt::Debug};

use async_lsp::lsp_types::CompletionItem;
use common::type_::ContainsOffset;
use salsa::Database;

use crate::{
    check::{state::CheckState, SemanticToken},
    db::input::SourceFile,
    parser::{top::Top, Ast},
    ty::Ty,
    util::Span,
};

pub mod common;
pub mod definitions;
pub mod expr;
pub mod stmt;
#[allow(deprecated)]
pub mod top;

pub trait AstItem: Debug {
    fn at_offset<'me>(&'me self, _state: &mut CheckState, _offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        todo!()
    }

    fn tokens(&self, _state: &mut CheckState, _tokens: &mut Vec<SemanticToken>) {}

    fn hover<'db>(
        &self,
        _state: &mut CheckState<'_, 'db>,
        _offset: usize,
        _type_vars: &HashMap<u32, Ty<'db>>,
    ) -> Option<String> {
        None
    }

    fn goto_def(
        &self,
        _state: &mut CheckState<'_, '_>,
        _offset: usize,
    ) -> Option<(SourceFile, Span)> {
        None
    }

    fn completions(&self, _state: &mut CheckState, _offset: usize) -> Vec<CompletionItem> {
        vec![]
    }
}

impl<'db> Ast<'db> {
    pub fn at_offset<'ty, 'me, 'state>(
        &'me self,
        db: &'db dyn Database,
        state: &'state mut CheckState<'ty, 'db>,
        offset: usize,
    ) -> Option<&'me dyn AstItem>
    where
        Self: Sized,
    {
        for (item, span) in self.tops(db) {
            if span.contains_offset(offset) {
                return Some(item.at_offset(state, offset));
            }
            if let Top::Use(u) = item {
                state.import(u);
            }
        }
        None
    }

    pub fn semantic_tokens<'ty, 'me, 'state>(
        &'me self,
        db: &'db dyn Database,
        state: &'state mut CheckState<'ty, 'db>,
    ) -> Vec<SemanticToken>
    where
        Self: Sized,
    {
        let mut tokens = vec![];
        for item in self.tops(db) {
            item.0.tokens(state, &mut tokens);
        }
        tokens
    }
}
