use std::{collections::HashMap, fmt::Debug};

use async_lsp::lsp_types::CompletionItem;
use common::type_::ContainsOffset;
use salsa::Database;

use crate::{
    check::{state::CheckState, SemanticToken},
    parser::FileData,
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

impl<'db> FileData<'db> {
    pub fn at_offset<'ty, 'me, 'state>(
        &'me self,
        db: &'db dyn Database,
        state: &'state mut CheckState<'ty, 'db>,
        offset: usize,
    ) -> Option<&'me dyn AstItem>
    where
        Self: Sized,
    {
        for import in self.imports(db) {
            state.import(import);
        }
        for item in self.tops(db) {
            let span = item.span(db);
            if span.contains_offset(offset) {
                return Some(item.data(db).at_offset(state, offset));
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
        for import in self.imports(db) {
            import.tokens(state, &mut tokens);
            state.import(import);
        }
        for item in self.tops(db) {
            item.data(db).tokens(state, &mut tokens);
        }
        tokens
    }
}
