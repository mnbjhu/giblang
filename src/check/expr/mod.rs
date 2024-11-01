use std::ops::ControlFlow;

use crate::{
    item::AstItem,
    parser::expr::Expr,
    ty::Ty,
    util::{Span, Spanned},
};

use self::{
    ident::{check_ident, check_ident_is},
    tuple::{check_tuple, check_tuple_is},
};

use super::{state::CheckState, Check, ControlIter};

pub mod call;
pub mod code_block;
pub mod field;
pub mod ident;
pub mod lambda;
pub mod lit;
pub mod match_;
pub mod match_arm;
pub mod member;
pub mod op;
pub mod tuple;

impl<'ast, 'db, Iter: ControlIter<'ast>> Check<'ast, 'db, Iter, Ty<'db>> for Spanned<Expr> {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        args: (),
    ) -> ControlFlow<&'ast dyn AstItem, Ty<'db>> {
        match self {
            Expr::Literal(lit) => lit.to_ty(state.db),
            Expr::Ident(ident) => check_ident(state, ident),
            Expr::CodeBlock(block) => block.check(state),
            // TODO: Actually think about generics
            Expr::Call(call) => call.check(state),
            Expr::Match(match_) => match_.check(state),
            Expr::Tuple(values) => check_tuple(values, state),
            // TODO: Handle if else expr types
            Expr::IfElse(_) => todo!(),
            Expr::MemberCall(member) => member.check(state),
            Expr::Op(op) => op.check(state),
            Expr::Field(field) => field.check(state),
            Expr::Lambda(lambda) => lambda.check(state),
            Expr::Error => Ty::Unknown,
        }
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        args: (),
    ) -> ControlFlow<&'ast dyn AstItem, Ty<'db>> {
        match &self.0 {
            Expr::Literal(lit) => lit.expect_instance_of(expected, state, span),
            Expr::Ident(ident) => check_ident_is(state, ident, expected),
            Expr::CodeBlock(block) => block.expect(state, control, (), expected),
            Expr::Call(call) => call.expected_instance_of(expected, state, span),
            Expr::Match(match_) => match_.is_instance_of(expected, state),
            Expr::Tuple(v) => check_tuple_is(state, expected, v, span),
            Expr::IfElse(_) => todo!(),
            Expr::Op(op) => op.expected_instance_of(expected, state, span),
            Expr::MemberCall(member) => member.expected_instance_of(expected, state, span),
            Expr::Field(field) => field.expected_instance_of(expected, state, span),
            Expr::Lambda(lambda) => lambda.expected_instance_of(expected, state, span),
            Expr::Error => {}
        }
    }
}
