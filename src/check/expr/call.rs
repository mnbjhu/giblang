use crate::{
    check::state::CheckState, parser::expr::call::Call, project::Project, ty::Ty, util::Span,
};

impl<'proj> Call {
    pub fn check(&self, project: &'proj Project, state: &mut CheckState<'proj>) -> Ty {
        let name_ty = self.name.0.check(project, state);
        if let Ty::Function {
            args: expected_args,
            ret,
            receiver,
        } = &name_ty
        {
            if let Some(receiver) = receiver {
                state.simple_error(
                    &format!("Expected a receiver of type {}", receiver.get_name(state)),
                    self.name.1,
                );
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
                    let actual = arg.expect_instance_of(expected, project, state, *span);
                    expected.expect_is_instance_of(&actual, state, false, *span);
                });
            ret.as_ref().clone()
        } else if let Ty::Unknown = name_ty {
            Ty::Unknown
        } else {
            state.simple_error(
                &format!(
                    "Expected a function but found '{}'",
                    name_ty.get_name(state)
                ),
                self.name.1,
            );
            Ty::Unknown
        }
    }

    pub fn expected_instance_of(
        &self,
        expected: &Ty,
        project: &'proj Project,
        state: &mut CheckState<'proj>,
        span: Span,
    ) -> Ty {
        let actual = self.check(project, state);
        actual.expect_is_instance_of(expected, state, false, span);
        actual
    }
}
