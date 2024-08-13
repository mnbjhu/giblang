use crate::{parser::expr::Expr, ty::Ty, util::Span};

use self::{
    code_block::{check_code_block, check_code_block_is},
    ident::{check_ident, check_ident_is},
    tuple::{check_tuple, check_tuple_is},
};

use super::CheckState;

pub mod call;
pub mod code_block;
pub mod ident;
pub mod lit;
pub mod match_;
pub mod match_arm;
pub mod member;
pub mod tuple;

impl Expr {
    pub fn check(&self, state: &mut CheckState<'_>) -> Ty {
        match self {
            Expr::Literal(lit) => lit.into(),
            Expr::Ident(ident) => check_ident(state, ident),
            Expr::CodeBlock(block) => check_code_block(state, block),
            // TODO: Actually think about generics
            Expr::Call(call) => call.check(state),
            Expr::Match(match_) => match_.check(state),
            Expr::Tuple(values) => check_tuple(values, state),
            // TODO: Handle if else expr types
            Expr::IfElse(_) => todo!(),
            Expr::MemberCall(member) => member.check(state),
        }
    }

    pub fn expect_instance_of(&self, expected: &Ty, state: &mut CheckState<'_>, span: Span) {
        match self {
            Expr::Literal(lit) => lit.expect_instance_of(expected, state, span),
            Expr::Ident(ident) => check_ident_is(state, ident, expected),
            Expr::CodeBlock(block) => check_code_block_is(state, expected, block, span),
            Expr::Call(call) => call.expected_instance_of(expected, state, span),
            Expr::Match(match_) => match_.is_instance_of(expected, state),
            Expr::Tuple(v) => check_tuple_is(state, expected, v, span),
            Expr::IfElse(_) => todo!(),
            Expr::MemberCall(member) => member.expected_instance_of(expected, state, span),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::check::state::CheckState;
    use crate::lexer::parser::lexer;
    use crate::parser::expr::expr_parser;
    use crate::parser::stmt::stmt_parser;
    use crate::ty::Ty;
    use crate::util::Span;
    use chumsky::input::Input;
    use chumsky::Parser;

    pub fn parse_expr<'proj>(state: &mut CheckState<'proj>, expr: &str) -> Ty {
        let eoi = Span::splat(expr.len());
        let tokens = lexer().parse(expr).unwrap();
        let ty = expr_parser(stmt_parser())
            .parse(tokens.spanned(eoi))
            .unwrap();
        ty.check(state)
    }

    pub fn parse_expr_with_expected<'proj>(
        state: &mut CheckState<'proj>,
        expected: &Ty,
        expr: &str,
    ) {
        let eoi = Span::splat(expr.len());
        let tokens = lexer().parse(expr).unwrap();
        let expr = expr_parser(stmt_parser())
            .parse(tokens.spanned(eoi))
            .unwrap();
        expr.expect_instance_of(expected, state, Span::splat(0));
    }
}
