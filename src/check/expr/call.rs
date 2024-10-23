use crate::{
    check::{
        err::{missing_receiver::MissingReceiver, unexpected_args::UnexpectedArgs, CheckError},
        state::CheckState,
    },
    parser::expr::call::Call,
    ty::{FuncTy, Ty},
    util::Span,
};

impl<'db> Call {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        let name_ty = self.name.0.check(state);
        if let Ty::Unknown = name_ty {
            for arg in &self.args {
                arg.0.check(state);
            }
            return Ty::Unknown;
        }
        let func_ty = name_ty.try_get_func_ty(state, self.name.1);
        if let Some(func_ty) = &func_ty {
            let FuncTy {
                args: expected_args,
                ret,
                receiver,
            } = &func_ty;
            if let Some(receiver) = receiver {
                state.error(CheckError::MissingReceiver(MissingReceiver {
                    span: self.name.1,
                    file: state.file_data,
                    expected: receiver.get_name(state, None),
                }));
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
            self.args
                .iter()
                .zip(expected_args)
                .for_each(|((arg, span), expected)| {
                    arg.expect_instance_of(expected, state, *span);
                });
            ret.as_ref().clone()
        } else if let Ty::Unknown = name_ty {
            Ty::Unknown
        } else {
            state.simple_error(
                &format!(
                    "Expected a function but found '{}'",
                    name_ty.get_name(state, None)
                ),
                self.name.1,
            );
            Ty::Unknown
        }
    }

    pub fn expected_instance_of(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) {
        let ret = self.check(state);
        ret.expect_is_instance_of(expected, state, false, span);
    }
}

