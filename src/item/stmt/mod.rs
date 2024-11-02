use crate::parser::stmt::Stmt;

use super::AstItem;

pub mod let_;
impl AstItem for Stmt {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Stmt::Expr(e) => e.pretty(allocator),
            Stmt::Let(l) => l.pretty(allocator),
        }
    }
}
