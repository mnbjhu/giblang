use std::{collections::HashMap, fmt::Debug};

use async_lsp::lsp_types::CompletionItem;
use pretty::{DocAllocator, DocBuilder};

use crate::{
    check::{build_state::BuildState, state::CheckState, Dir, SemanticToken},
    db::input::SourceFile,
    parser::{top::Top, Ast},
    ty::Ty,
    util::{Span, Spanned},
};

pub mod common;
pub mod definitions;
pub mod expr;
pub mod stmt;
#[allow(deprecated)]
pub mod top;

pub trait AstItem: Debug {
    fn item_name(&self) -> &'static str;
    fn tokens(&self, _state: &mut CheckState, _tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {}

    fn hover<'db>(
        &self,
        _state: &mut CheckState<'db>,
        _offset: usize,
        _type_vars: &HashMap<u32, Ty<'db>>,
        _ty: &Ty<'db>,
    ) -> Option<String> {
        None
    }

    fn goto_def(&self, _state: &mut CheckState<'_>, _offset: usize) -> Option<(SourceFile, Span)> {
        None
    }

    fn completions(
        &self,
        _state: &mut CheckState,
        _offset: usize,
        _: &HashMap<u32, Ty<'_>>,
        _ty: &Ty<'_>,
    ) -> Vec<CompletionItem> {
        vec![]
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone;

    fn build(&self, state: &mut CheckState<'_>, builder: &mut BuildState, dir: Dir) {}
}

impl<'db> Ast<'db> {}
pub fn pretty_format<'b, 'db, D, A>(
    ast: &'b [Spanned<Top>],
    allocator: &'b D,
) -> DocBuilder<'b, D, A>
where
    D: DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    let tops = ast.iter().map(|(item, _)| {
        if let Top::Use(_) = item {
            item.pretty(allocator)
        } else {
            allocator.hardline().append(item.pretty(allocator))
        }
    });
    allocator.intersperse(tops, allocator.hardline())
}
