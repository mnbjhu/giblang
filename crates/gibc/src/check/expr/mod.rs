use std::ops::ControlFlow;

use crate::{item::AstItem, parser::expr::Expr, ty::Ty, util::Span};

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

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Expr {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        match self {
            Expr::Literal(lit) => lit.check(state, control, span, ()),
            Expr::Ident(ident) => ident.check(state, control, span, ()),
            Expr::CodeBlock(block) => block.check(state, control, span, ()),
            // TODO: Actually think about generics
            Expr::Call(call) => call.check(state, control, span, ()),
            Expr::Match(match_) => match_.check(state, control, span, ()),
            Expr::Tuple(values) => values.check(state, control, span, ()),
            // TODO: Handle if else expr types
            Expr::IfElse(_) => todo!(),
            Expr::MemberCall(member) => member.check(state, control, span, ()),
            Expr::Op(op) => op.check(state, control, span, ()),
            Expr::Field(field) => field.check(state, control, span, ()),
            Expr::Lambda(lambda) => lambda.check(state, control, span, ()),
            Expr::Error => ControlFlow::Continue(Ty::Unknown),
        }
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        match &self {
            Expr::Literal(lit) => lit.expect(state, control, expected, span, ()),
            Expr::Ident(ident) => ident.expect(state, control, expected, span, ()),
            Expr::CodeBlock(block) => block.expect(state, control, expected, span, ()),
            Expr::Call(call) => call.expect(state, control, expected, span, ()),
            Expr::Match(match_) => match_.expect(state, control, expected, span, ()),
            Expr::Tuple(v) => v.expect(state, control, expected, span, ()),
            Expr::IfElse(_) => todo!(),
            Expr::Op(op) => op.expect(state, control, expected, span, ()),
            Expr::MemberCall(member) => member.expect(state, control, expected, span, ()),
            Expr::Field(field) => field.expect(state, control, expected, span, ()),
            Expr::Lambda(lambda) => lambda.expect(state, control, expected, span, ()),
            Expr::Error => ControlFlow::Continue(Ty::Unknown),
        }
    }
}
