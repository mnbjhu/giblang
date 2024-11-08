use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::expr::member::MemberCall,
    ty::{FuncTy, Ty},
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for MemberCall {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span)?;
        let rec: Ty = self.rec.0.check(state, control, self.rec.1, ())?;
        let funcs = rec.get_member_func(&self.name, state);
        let Some(func_ty) = funcs else {
            state.simple_error(
                &format!(
                    "No function {} found for type {}",
                    self.name.0,
                    rec.get_name(state, None)
                ),
                self.name.1,
            );
            control.act(self, state, Dir::Exit(Ty::Unknown), span)?;
            return ControlFlow::Continue(Ty::Unknown);
        };
        let FuncTy {
            args: expected_args,
            ret,
            receiver,
        } = func_ty;
        if let Some(expected) = receiver {
            rec.expect_is_instance_of(&expected, state, self.rec.1);
        }
        if expected_args.len() != self.args.len() {
            state.simple_error(
                &format!(
                    "Expected {} arguments but found {}",
                    expected_args.len(),
                    self.args.len()
                ),
                self.name.1,
            );
        }

        for ((arg, span), expected) in self.args.iter().zip(expected_args) {
            arg.expect(state, control, &expected, *span, ())?;
        }
        let ty = ret.as_ref().clone();
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
        let ty = self.check(state, control, span, ())?;
        ty.expect_is_instance_of(expected, state, span);
        ControlFlow::Continue(ty)
    }
}
