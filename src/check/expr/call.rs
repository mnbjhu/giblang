use crate::{
    check::{
        err::{missing_receiver::MissingReceiver, unexpected_args::UnexpectedArgs, CheckError},
        state::CheckState,
    },
    parser::expr::call::Call,
    project::Project,
    ty::{FuncTy, Ty},
    util::Span,
};

impl<'proj> Call {
    pub fn check(&self, project: &'proj Project, state: &mut CheckState<'proj>) -> Ty {
        let name_ty = self.name.0.check(project, state);
        if let Ty::Function(func_ty) = &name_ty {
            let FuncTy {
                args: expected_args,
                ret,
                receiver,
            } = &func_ty;
            if let Some(receiver) = receiver {
                state.error(CheckError::MissingReceiver(MissingReceiver {
                    span: self.name.1,
                    file: state.file_data.end,
                    expected: receiver.as_ref().clone(),
                }));
            }
            if expected_args.len() != self.args.len() {
                state.error(CheckError::UnexpectedArgs(UnexpectedArgs {
                    expected: expected_args.len(),
                    found: self.args.len(),
                    span: self.name.1,
                    file: state.file_data.end,
                    func: func_ty.clone(),
                }));
            }
            self.args
                .iter()
                .zip(expected_args)
                .for_each(|((arg, span), expected)| {
                    arg.expect_instance_of(expected, project, state, *span);
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

#[cfg(test)]
mod tests {
    use crate::check::err::CheckError;
    use crate::check::expr::tests::parse_expr;
    use crate::check::expr::tests::parse_expr_with_expected;
    use crate::project::check_test_state;

    use crate::project::Project;
    use crate::ty::Ty;

    #[test]
    fn add() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let ty = parse_expr(&project, &mut state, "add(1, 2)");
        assert_eq!(ty.get_name(&state), "Int");
        assert_eq!(state.errors, vec![]);
    }

    #[test]
    fn add_with_wrong_arg_ty() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let ty = parse_expr(&project, &mut state, "add(1, 'a')");
        assert_eq!(ty.get_name(&state), "Int");
        assert_eq!(state.errors.len(), 1);
        if let CheckError::IsNotInstance(err) = &state.errors[0] {
            assert_eq!(err.expected.get_name(&state), "Int");
            assert_eq!(err.found.get_name(&state), "Char");
        } else {
            panic!("Expected IsNotInstance error")
        }
    }

    #[test]
    fn add_with_too_many_args() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let ty = parse_expr(&project, &mut state, "add(1, 2, 3)");
        assert_eq!(ty.get_name(&state), "Int");
        assert_eq!(state.errors.len(), 1);
        if let CheckError::UnexpectedArgs(err) = &state.errors[0] {
            assert_eq!(err.expected, 2);
            assert_eq!(err.found, 3);
        } else {
            panic!("Expected UnexpectedArgs error")
        }
    }

    #[test]
    fn add_with_too_few_args() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let ty = parse_expr(&project, &mut state, "add(1)");
        assert_eq!(ty.get_name(&state), "Int");
        assert_eq!(state.errors.len(), 1);
        if let CheckError::UnexpectedArgs(err) = &state.errors[0] {
            assert_eq!(err.expected, 2);
            assert_eq!(err.found, 1);
        } else {
            panic!("Expected UnexpectedArgs error")
        }
    }

    #[test]
    fn expected_receiver() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let ty = parse_expr(&project, &mut state, "factorial()");
        assert_eq!(ty.get_name(&state), "Int");
        assert_eq!(state.errors.len(), 1);
        if let CheckError::MissingReceiver(err) = &state.errors[0] {
            assert_eq!(err.expected, Ty::int());
        } else {
            panic!("Expected MissingReceiver error")
        }
    }

    #[test]
    fn expected_is_returned() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let expected = Ty::int();
        let ty = parse_expr_with_expected(&project, &mut state, &expected, "add(1, 2)");
        assert_eq!(ty.get_name(&state), "Int");
        assert_eq!(state.errors, vec![]);
    }

    #[test]
    fn expected_is_not_returned() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let expected = Ty::string();
        let ty = parse_expr_with_expected(&project, &mut state, &expected, "add(1, 2)");
        assert_eq!(ty.get_name(&state), "Int");
        assert_eq!(state.errors.len(), 1);
        if let CheckError::IsNotInstance(err) = &state.errors[0] {
            assert_eq!(err.expected.get_name(&state), "String");
            assert_eq!(err.found.get_name(&state), "Int");
        } else {
            panic!("Expected IsNotInstance error")
        }
    }

    #[test]
    fn args_imply_type_vars() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let ty = parse_expr(&project, &mut state, "ident(\"Hello\")");
        state.resolve_type_vars();
        assert_eq!(ty.get_name(&state), "String");
        assert_eq!(state.errors, vec![]);
    }
}
