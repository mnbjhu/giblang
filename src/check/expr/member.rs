use crate::{
    check::state::CheckState, parser::expr::member::MemberCall, project::Project, ty::Ty,
    util::Span,
};

use super::ident::check_ident;

impl MemberCall {
    pub fn check<'module>(&self, project: &'module Project, state: &mut CheckState<'module>) -> Ty {
        let ty = check_ident(state, &vec![self.name.clone()], project);

        if let Ty::Function {
            args: expected_args,
            ret,
            receiver: Some(receiver),
        } = &ty
        {
            self.rec
                .0
                .as_ref()
                .expect_instance_of(receiver, project, state, self.rec.1);

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
                    arg.expect_instance_of(expected, project, state, *span);
                });
            ret.as_ref().clone()
        } else {
            Ty::Unknown
        }
    }

    pub fn expected_instance_of<'module>(
        &self,
        expected: &Ty,
        project: &'module Project,
        state: &mut CheckState<'module>,
        span: Span,
    ) -> Ty {
        let actual = self.check(project, state);
        actual.expect_is_instance_of(expected, state, false, span);
        actual
    }
}
