use std::{collections::HashMap, fmt::Debug};

use async_lsp::lsp_types::CompletionItem;
use common::type_::ContainsOffset;
use salsa::Database;

use crate::{
    check::{state::CheckState, SemanticToken},
    parser::{top::Top, Ast},
    ty::Ty,
};

pub mod common;
pub mod expr;
pub mod stmt;
pub mod top;

pub trait AstItem: Debug {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        todo!()
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {}

    fn hover<'db>(
        &self,
        state: &mut CheckState<'_, 'db>,
        offset: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> Option<String> {
        None
    }

    fn completions(&self, state: &mut CheckState, offset: usize) -> Vec<CompletionItem> {
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
