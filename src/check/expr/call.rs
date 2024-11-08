use std::ops::ControlFlow;

use crate::{
    check::{
        err::{missing_receiver::MissingReceiver, unexpected_args::UnexpectedArgs, CheckError},
        state::CheckState,
        Check, ControlIter, Dir,
    },
    item::AstItem,
    parser::expr::call::Call,
    ty::{FuncTy, Ty},
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Call {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span)?;
        let name_ty = self.name.0.check(state, control, self.name.1, ())?;
        if let Ty::Unknown = name_ty {
            for arg in &self.args {
                arg.0.check(state, control, arg.1, ())?;
            }
            control.act(self, state, Dir::Exit(Ty::Unknown), span)?;
            return ControlFlow::Continue(Ty::Unknown);
        }
        let func_ty = name_ty.try_get_func_ty(state, self.name.1);
        if let Some(func_ty) = &func_ty {
            let FuncTy {
                args: expected_args,
                ret,
                receiver,
            } = &func_ty;
            if let Some(receiver) = receiver {
                if let Some(self_ty) = state.get_variable("self") {
                    self_ty
                        .ty
                        .expect_is_instance_of(receiver, state, self.name.1);
                } else {
                    state.error(CheckError::MissingReceiver(MissingReceiver {
                        span: self.name.1,
                        file: state.file_data,
                        expected: receiver.get_name(state, None),
                    }));
                }
            }
            if expected_args.len() != self.args.len() {
                state.error(CheckError::UnexpectedArgs(UnexpectedArgs {
                    expected: expected_args.len(),
                    found: self.args.len(),
                    span: self.name.1,
                    file: state.file_data,
                    func: func_ty.get_name(state, None),
                }));
            }
            for ((arg, span), expected) in self.args.iter().zip(expected_args) {
                arg.expect(state, control, expected, *span, ())?;
            }
            let ty = ret.as_ref().clone();
            control.act(self, state, Dir::Exit(ty.clone()), span)?;
            ControlFlow::Continue(ty)
        } else if let Ty::Unknown = name_ty {
            control.act(self, state, Dir::Exit(Ty::Unknown), span)?;
            ControlFlow::Continue(Ty::Unknown)
        } else {
            state.simple_error(
                &format!(
                    "Expected a function but found '{}'",
                    name_ty.get_name(state, None)
                ),
                self.name.1,
            );
            control.act(self, state, Dir::Exit(Ty::Unknown), span)?;
            ControlFlow::Continue(Ty::Unknown)
        }
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        (): (),
    ) -> std::ops::ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let ret = self.check(state, control, span, ())?;
        ret.expect_is_instance_of(expected, state, span);
        ControlFlow::Continue(ret)
    }
}
