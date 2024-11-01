use crate::{
    check::{state::CheckState, SemanticToken},
    parser::{expr::Expr, stmt::Stmt},
    util::Spanned,
};

use super::{
    common::{generics::brackets, type_::ContainsOffset},
    AstItem,
};

pub mod call;
pub mod code_block;
pub mod field;
pub mod ident;
pub mod if_else;
pub mod lit;
pub mod match_;
pub mod match_arm;
pub mod member_call;
pub mod op;
pub mod lambda;

impl AstItem for Expr {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        match self {
            Expr::Literal(l) => l.at_offset(state, offset),
            Expr::Ident(i) => i.at_offset(state, offset),
            Expr::Call(c) => c.at_offset(state, offset),
            Expr::MemberCall(m) => m.at_offset(state, offset),
            Expr::IfElse(i) => i.at_offset(state, offset),
            Expr::Match(m) => m.at_offset(state, offset),
            Expr::CodeBlock(c) => {
                for stmt in c {
                    if stmt.1.contains_offset(offset) {
                        return stmt.0.at_offset(state, offset);
                    }
                    stmt.check(state);
                }
                self
            }
            Expr::Tuple(exprs) => {
                for (expr, span) in exprs {
                    if span.contains_offset(offset) {
                        return expr.at_offset(state, offset);
                    }
                }
                self
            }
            Expr::Error => &Expr::Error,
            Expr::Op(op) => op.at_offset(state, offset),
            Expr::Field(field) => field.at_offset(state, offset),
            Expr::Lambda(l) => l.at_offset(state, offset),
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        match self {
            Expr::Literal(l) => l.tokens(state, tokens),
            Expr::Ident(i) => i.tokens(state, tokens),
            Expr::Call(c) => c.tokens(state, tokens),
            Expr::MemberCall(m) => m.tokens(state, tokens),
            Expr::IfElse(i) => i.tokens(state, tokens),
            Expr::Match(m) => m.tokens(state, tokens),
            Expr::CodeBlock(c) => {
                for (stmt, _) in c {
                    stmt.tokens(state, tokens);
                }
            }
            Expr::Tuple(exprs) => {
                for expr in exprs {
                    expr.0.tokens(state, tokens);
                }
            }
            Expr::Error => {}
            Expr::Op(op) => op.tokens(state, tokens),
            Expr::Field(field) => field.tokens(state, tokens),
            Expr::Lambda(lambda) => lambda.tokens(state, tokens),
        }
    }

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
