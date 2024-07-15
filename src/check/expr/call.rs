use crate::{
    check::{state::CheckState, ty::Ty},
    fs::project::Project,
    parser::expr::call::Call,
};

impl Call {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) -> Ty<'module> {
        let name_ty = self.name.0.check(project, state);
        // TODO: Think about receivers
        if let Ty::Function {
            args: expected_args,
            ret,
            ..
        } = &name_ty
        {
            let arg_tys = self
                .args
                .iter()
                .map(|arg| (arg.0.check(project, state), arg.1))
                .collect::<Vec<_>>();
            if expected_args.len() != self.args.len() {
                state.error(
                    &format!(
                        "Expected {} arguments but found {}",
                        expected_args.len(),
                        self.args.len()
                    ),
                    self.name.1,
                );
            }

            arg_tys
                .iter()
                .zip(expected_args)
                .for_each(|((arg, span), expected)| {
                    if !arg.is_instance_of(expected, project) {
                        state.error(
                            &format!(
                                "Expected argument to be of type '{}' but found '{}'",
                                expected, arg
                            ),
                            *span,
                        );
                    }
                });
            ret.as_ref().clone()
        } else if let Ty::Unknown = name_ty {
            Ty::Unknown
        } else {
            state.error(
                &format!("Expected a function but found '{name_ty:?}'"),
                self.name.1,
            );
            Ty::Unknown
        }
    }
}
