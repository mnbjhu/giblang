use crate::{
    check::state::CheckState,
    parser::expr::member::MemberCall,
    ty::{FuncTy, Ty},
    util::Span,
};

use super::ident::check_ident;

impl MemberCall {
    pub fn check(&self, state: &mut CheckState<'_>) -> Ty {
        let ty = check_ident(state, &vec![self.name.clone()]);

        if let Ty::Function(FuncTy {
            args: expected_args,
            ret,
            receiver: Some(receiver),
        }) = &ty
        {
            self.rec
                .0
                .as_ref()
                .expect_instance_of(receiver, state, self.rec.1);

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
                    arg.expect_instance_of(expected, state, *span);
                });
            ret.as_ref().clone()
        } else {
            Ty::Unknown
        }
    }

    pub fn expected_instance_of(&self, expected: &Ty, state: &mut CheckState<'_>, span: Span) {
        let actual = self.check(state);
        actual.expect_is_instance_of(expected, state, false, span);
    }
}
