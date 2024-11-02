use crate::{
    parser::{expr::Expr, stmt::Stmt},
    util::Spanned,
};

use super::{
    common::generics::brackets,
    AstItem,
};

pub mod block;
pub mod call;
pub mod code_block;
pub mod field;
pub mod ident;
pub mod if_else;
pub mod lambda;
pub mod lit;
pub mod match_;
pub mod match_arm;
pub mod member_call;
pub mod op;

impl AstItem for Expr {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Expr::Literal(l) => l.pretty(allocator),
            Expr::Ident(i) => i.pretty(allocator),
            Expr::Call(c) => c.pretty(allocator),
            Expr::MemberCall(m) => m.pretty(allocator),
            Expr::IfElse(i) => i.pretty(allocator),
            Expr::Match(m) => m.pretty(allocator),
            Expr::CodeBlock(c) => {
                // TODO: Make codeblock ast item
                pretty_codeblock(allocator, c)
            }
            Expr::Tuple(exprs) => brackets(allocator, "(", ")", exprs),
            Expr::Error => panic!(),
            Expr::Op(op) => op.pretty(allocator),
            Expr::Field(field) => field.pretty(allocator),
            Expr::Lambda(lambda) => lambda.pretty(allocator),
        }
    }
}

pub fn pretty_codeblock<'b, D, A>(
    allocator: &'b D,
    code_block: &'b [Spanned<Stmt>],
) -> pretty::DocBuilder<'b, D, A>
where
    D: pretty::DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    if code_block.is_empty() {
        return allocator.text("{}");
    }
    let sep = allocator.hardline();
    let stmts = code_block.iter().map(|(stmt, _)| stmt.pretty(allocator));
    allocator
        .text("{")
        .append(
            allocator
                .hardline()
                .append(allocator.intersperse(stmts, sep))
                .nest(4)
                .group(),
        )
        .append(allocator.hardline())
        .append("}")
}
