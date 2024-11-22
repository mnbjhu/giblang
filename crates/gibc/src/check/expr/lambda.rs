use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir, TokenKind},
    item::AstItem,
    parser::expr::lambda::{Lambda, LambdaParam},
    ty::{FuncTy, Ty},
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Lambda {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        state.enter_scope();
        control.act(self, state, Dir::Enter, span)?;
        let mut args = vec![];
        for (arg, span) in &self.args {
            args.push(arg.check(state, control, *span, ())?);
        }
        let ret = self.body.0.check(state, control, self.body.1, ())?;
        state.exit_scope();
        let ty = Ty::Function(FuncTy {
            receiver: None,
            args,
            ret: Box::new(ret),
        });
        control.act(self, state, Dir::Exit(ty.clone()), span)?;
        ControlFlow::Continue(ty)
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        state.enter_scope();
        let ty = if let Ty::Function(expected) = expected {
            control.act(self, state, Dir::Enter, span)?;
            for ((arg, span), expected) in self.args.iter().zip(&expected.args) {
                arg.expect(state, control, expected, *span, ())?;
            }
            if self.args.len() > expected.args.len() {
                for arg in self.args.iter().skip(expected.args.len()) {
                    arg.0.check(state, control, arg.1, ())?;
                    state.simple_error("Unexpected argument", arg.1);
                }
            }
            if self.args.is_empty() && expected.args.len() == 1 {
                state.insert_variable(
                    "it".to_string(),
                    expected.args[0].clone(),
                    TokenKind::Var,
                    span,
                );
            }
            if let Some(receiver) = &expected.receiver {
                state.add_self_param(receiver.as_ref().clone(), span);
            }
            self.body
                .0
                .expect(state, control, &expected.ret, self.body.1, ())?;
            Ty::Function(expected.clone())
        } else {
            let ty = self.check(state, control, span, ())?;
            ty.expect_is_instance_of(expected, state, span);
            control.act(self, state, Dir::Exit(ty.clone()), span)?;
            ty
        };
        control.act(self, state, Dir::Exit(ty.clone()), span)?;
        state.exit_scope();
        ControlFlow::Continue(ty)
    }
}

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for LambdaParam {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span)?;
        let ty = if let Some(ty) = &self.ty {
            let expected = ty.0.check(state, control, ty.1, ())?;
            self.pattern
                .0
                .check(state, control, self.pattern.1, &expected)?;
            expected
        } else {
            let id = state
                .type_state
                .new_type_var(self.pattern.1, state.file_data);
            let type_var = Ty::TypeVar { id };
            self.pattern
                .0
                .check(state, control, self.pattern.1, &type_var)?;
            type_var
        };
        control.act(self, state, Dir::Exit(ty.clone()), span)?;
        ControlFlow::Continue(ty)
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span)?;
        let ty = if let Some(ty) = &self.ty {
            let explicit = ty.0.check(state, control, ty.1, ())?;
            explicit.expect_is_instance_of(expected, state, ty.1);
            self.pattern
                .0
                .check(state, control, self.pattern.1, &explicit)?;
            explicit
        } else {
            self.pattern
                .0
                .check(state, control, self.pattern.1, expected)?;
            expected.clone()
        };
        control.act(self, state, Dir::Exit(ty.clone()), span)?;
        ControlFlow::Continue(ty)
    }
}
