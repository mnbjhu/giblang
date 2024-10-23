use crate::{
    check::state::CheckState,
    parser::expr::member::MemberCall,
    ty::{FuncTy, Ty},
    util::Span,
};


impl<'db> MemberCall {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        let rec = self.rec.0.check(state);
        let Some(func_ty) = rec.get_member_func(&self.name, state) else {
            state.simple_error(
                &format!(
                    "No function {} found for type {}",
                    self.name.0,
                    rec.get_name(state, None)
                ),
                self.name.1,
            );
            return Ty::Unknown;
        };
        let FuncTy {
            args: expected_args,
            ret,
            receiver,
        } = func_ty;
        if let Some(rec) = receiver {
            self.rec.0.expect_instance_of(&rec, state, self.rec.1);
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

        self.args
            .iter()
            .zip(expected_args)
            .for_each(|((arg, span), expected)| {
                arg.expect_instance_of(&expected, state, *span);
            });
        ret.as_ref().clone()
    }

    pub fn expected_instance_of(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) {
        let actual = self.check(state);
        actual.expect_is_instance_of(expected, state, false, span);
    }
}

