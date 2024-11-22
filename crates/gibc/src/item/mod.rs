use std::fmt::Debug;

use pretty::{DocAllocator, DocBuilder};

use crate::{
    parser::{top::Top, Ast},
    util::Spanned,
};

pub mod common;
pub mod definitions;
pub mod expr;
pub mod stmt;
#[allow(deprecated)]
pub mod top;

pub trait AstItem: Debug {
    fn item_name(&self) -> &'static str;
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone;
}

impl<T: AstItem> AstItem for Spanned<T> {
    fn item_name(&self) -> &'static str {
        self.0.item_name()
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.0.pretty(allocator)
    }
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
