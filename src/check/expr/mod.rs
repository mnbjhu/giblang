use crate::{parser::expr::Expr, project::Project, ty::Ty, util::Span};

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
    pub fn check<'proj>(&self, project: &'proj Project, state: &mut CheckState<'proj>) -> Ty {
        match self {
            Expr::Literal(lit) => lit.into(),
            Expr::Ident(ident) => check_ident(state, ident, project),
            Expr::CodeBlock(block) => check_code_block(state, block, project),
            // TODO: Actually think about generics
            Expr::Call(call) => call.check(project, state),
            Expr::Match(match_) => match_.check(project, state),
            Expr::Tuple(values) => check_tuple(values, project, state),
            // TODO: Handle if else expr types
            Expr::IfElse(_) => todo!(),
            Expr::MemberCall(member) => member.check(project, state),
        }
    }

    pub fn expect_instance_of<'proj>(
        &self,
        expected: &Ty,
        project: &'proj Project,
        state: &mut CheckState<'proj>,
        span: Span,
    ) -> Ty {
        match self {
            Expr::Literal(lit) => lit.expect_instance_of(expected, state, span),
            Expr::Ident(ident) => check_ident_is(state, ident, expected, project),
            Expr::CodeBlock(block) => check_code_block_is(state, expected, block, project),
            Expr::Call(call) => call.expected_instance_of(expected, project, state, span),
            Expr::Match(match_) => match_.is_instance_of(expected, project, state),
            Expr::Tuple(v) => check_tuple_is(state, expected, v, project, span),
            Expr::IfElse(_) => todo!(),
            Expr::MemberCall(member) => member.expected_instance_of(expected, project, state, span),
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
    use chumsky::input::Input;
    use chumsky::Parser;

    use crate::project::Project;
    use crate::util::Span;

    pub fn parse_expr<'proj>(
        project: &'proj Project,
        state: &mut CheckState<'proj>,
        expr: &str,
    ) -> Ty {
        let eoi = Span::splat(expr.len());
        let tokens = lexer().parse(expr).unwrap();
        let ty = expr_parser(stmt_parser())
            .parse(tokens.spanned(eoi))
            .unwrap();
        ty.check(project, state)
    }

    pub fn parse_expr_with_expected<'proj>(
        project: &'proj Project,
        state: &mut CheckState<'proj>,
        expected: &Ty,
        expr: &str,
    ) -> Ty {
        let eoi = Span::splat(expr.len());
        let tokens = lexer().parse(expr).unwrap();
        let expr = expr_parser(stmt_parser())
            .parse(tokens.spanned(eoi))
            .unwrap();
        expr.expect_instance_of(expected, project, state, Span::splat(0))
    }
}
