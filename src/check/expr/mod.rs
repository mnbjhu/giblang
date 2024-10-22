use crate::{parser::expr::Expr, ty::Ty, util::Span};

use self::{
    code_block::{check_code_block, check_code_block_is},
    ident::{check_ident, check_ident_is},
    tuple::{check_tuple, check_tuple_is},
};

use super::state::CheckState;

pub mod call;
pub mod code_block;
pub mod field;
pub mod ident;
pub mod lit;
pub mod match_;
pub mod match_arm;
pub mod member;
pub mod op;
pub mod tuple;

impl<'db> Expr {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        match self {
            Expr::Literal(lit) => lit.to_ty(state.db),
            Expr::Ident(ident) => check_ident(state, ident),
            Expr::CodeBlock(block) => check_code_block(state, block),
            // TODO: Actually think about generics
            Expr::Call(call) => call.check(state),
            Expr::Match(match_) => match_.check(state),
            Expr::Tuple(values) => check_tuple(values, state),
            // TODO: Handle if else expr types
            Expr::IfElse(_) => todo!(),
            Expr::MemberCall(member) => member.check(state),
            Expr::Op(op) => op.check(state),
            Expr::Field(field) => field.check(state),
            Expr::Error => Ty::Unknown,
        }
    }

    pub fn expect_instance_of(&self, expected: &Ty<'db>, state: &mut CheckState<'db>, span: Span) {
        match self {
            Expr::Literal(lit) => lit.expect_instance_of(expected, state, span),
            Expr::Ident(ident) => check_ident_is(state, ident, expected),
            Expr::CodeBlock(block) => check_code_block_is(state, expected, block, span),
            Expr::Call(call) => call.expected_instance_of(expected, state, span),
            Expr::Match(match_) => match_.is_instance_of(expected, state),
            Expr::Tuple(v) => check_tuple_is(state, expected, v, span),
            Expr::IfElse(_) => todo!(),
            Expr::Op(op) => op.expected_instance_of(expected, state, span),
            Expr::MemberCall(member) => member.expected_instance_of(expected, state, span),
            Expr::Field(field) => field.expected_instance_of(expected, state, span),
            Expr::Error => {}
        }
    }
}
