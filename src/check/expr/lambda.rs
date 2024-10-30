use crate::{
    check::state::CheckState, parser::expr::lambda::{Lambda, LambdaParam}, ty::{FuncTy, Ty}, util::Span
};

use super::code_block::{check_code_block, check_code_block_is};

impl<'db> Lambda {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        state.enter_scope();
        let args = self.args.iter().map(|arg| arg.0.check(state)).collect();
        let ret = check_code_block(state, &self.body.0);
        state.exit_scope();
        Ty::Function(FuncTy { receiver: None, args, ret: Box::new(ret) })
    }

    pub fn expected_instance_of(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) {
        state.enter_scope();
        if let Ty::Function(expected) = expected {
            self.args.iter().zip(&expected.args).for_each(|((arg, _), expected)| {
                arg.expect_instance_of(expected, state, span);
            });
            if self.args.len() > expected.args.len() {
                for arg in self.args.iter().skip(expected.args.len()) {
                    arg.0.check(state);
                    state.simple_error("Unexpected argument", arg.1);
                }
            }
            if self.args.is_empty() && expected.args.len() == 1 {
                state.insert_variable("it".to_string(), expected.args[0].clone(), true, span);
            }
            if let Some(receiver) = &expected.receiver {
                state.add_self_param(receiver.as_ref().clone(), span);
            }
            check_code_block_is(state, &expected.ret, &self.body.0, span);
        } else {
            let ty = self.check(state);
            ty.expect_is_instance_of(expected, state, false, span);
        };
        state.exit_scope();
    }
}

impl<'db> LambdaParam {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        if let Some(ty) = &self.ty {
            let expected = ty.0.check(state);
            self.pattern.0.check(state, &expected);
            expected
        } else {
            let id = state.type_state.new_type_var(self.pattern.1, state.file_data);
            let type_var = Ty::TypeVar { id };
            self.pattern.0.check(state, &type_var);
            type_var

        }
    }

    pub fn expect_instance_of(&self, expected: &Ty<'db>, state: &mut CheckState<'db>, _: Span) -> Ty<'db> {
        if let Some(ty) = &self.ty {
            let explicit = ty.0.check(state);
            explicit.expect_is_instance_of(expected, state, true, ty.1);
            self.pattern.0.check(state, &explicit);
            explicit
        } else {
            self.pattern.0.check(state, expected);
            expected.clone()
        }
    }
}
