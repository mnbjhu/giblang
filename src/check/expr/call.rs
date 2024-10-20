use crate::{
    check::{
        err::{missing_receiver::MissingReceiver, unexpected_args::UnexpectedArgs, CheckError},
        state::CheckState,
    }, parser::expr::call::Call, project::decl::{DeclKind}, ty::{FuncTy, Ty}, util::Span
};

impl<'db> Call {
    pub fn check(&self, state: &mut CheckState<'_, 'db>) -> Ty<'db> {
        let name_ty = self.name.0.check(state);
        if let Ty::Unknown = name_ty {
            for arg in &self.args {
                arg.0.check(state);
            }
            return Ty::Unknown;
        }
        let func_ty = name_ty.try_get_func_ty(state);
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
                    expected: receiver.get_name(state),
                }));
            }
            if expected_args.len() != self.args.len() {
                state.error(CheckError::UnexpectedArgs(UnexpectedArgs {
                    expected: expected_args.len(),
                    found: self.args.len(),
                    span: self.name.1,
                    file: state.file_data,
                    func: func_ty.get_name(state),
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
                    name_ty.get_name(state)
                ),
                self.name.1,
            );
            Ty::Unknown
        }
    }



    pub fn expected_instance_of(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'_, 'db>,
        span: Span,
    ) {
        let name_ty = self.name.0.check(state);
        if let Ty::Function(func_ty) = &name_ty {
            let FuncTy {
                args: expected_args,
                ret,
                receiver,
            } = &func_ty;
            ret.expect_is_instance_of(expected, state, false, span);
            if let Some(receiver) = receiver {
                state.error(CheckError::MissingReceiver(MissingReceiver {
                    span: self.name.1,
                    file: state.file_data,
                    expected: receiver.get_name(state),
                }));
            }
            if expected_args.len() != self.args.len() {
                state.error(CheckError::UnexpectedArgs(UnexpectedArgs {
                    expected: expected_args.len(),
                    found: self.args.len(),
                    span: self.name.1,
                    file: state.file_data,
                    func: func_ty.get_name(state),
                }));
            }
            self.args
                .iter()
                .zip(expected_args)
                .for_each(|((arg, span), expected)| {
                    arg.expect_instance_of(expected, state, *span);
                });
        } else {
            state.simple_error(
                &format!(
                    "Expected a function but found '{}'",
                    name_ty.get_name(state)
                ),
                self.name.1,
            );
        }
    }
}

impl<'db> Ty<'db> {
    pub fn try_get_func_ty(&self, state: &mut CheckState<'_, 'db>) -> Option<FuncTy<'db>> {
        if let Ty::Function(func_ty) = self {
            Some(func_ty.clone())
        }  else if let Ty::Meta(ty) = self {
            if let Ty::Named { name, .. } = ty.as_ref() {
                let decl = state.project.get_decl(state.db, *name);
                if let Some(decl) = decl {
                    if let DeclKind::Struct { body, .. } = decl.kind(state.db) {
                        return body.get_constructor_ty(ty.as_ref().clone());
                    }
                }
            }
            None
        } else {
            None
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::check::err::CheckError;
//     use crate::check::expr::tests::parse_expr;
//     use crate::check::expr::tests::parse_expr_with_expected;
//     use crate::project::check_test_state;
//
//     use crate::project::Project;
//     use crate::ty::Ty;
//
//     #[test]
//     fn add() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_expr(&mut state, "add(1, 2)");
//         assert_eq!(ty.get_name(&state), "Int");
//         assert_eq!(state.errors, vec![]);
//     }
//
//     #[test]
//     fn add_with_wrong_arg_ty() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_expr(&mut state, "add(1, 'a')");
//         assert_eq!(ty.get_name(&state), "Int");
//         assert_eq!(state.errors.len(), 1);
//         if let CheckError::IsNotInstance(err) = &state.errors[0] {
//             assert_eq!(err.expected.get_name(&state), "Int");
//             assert_eq!(err.found.get_name(&state), "Char");
//         } else {
//             panic!("Expected IsNotInstance error")
//         }
//     }
//
//     #[test]
//     fn add_with_too_many_args() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_expr(&mut state, "add(1, 2, 3)");
//         assert_eq!(ty.get_name(&state), "Int");
//         assert_eq!(state.errors.len(), 1);
//         if let CheckError::UnexpectedArgs(err) = &state.errors[0] {
//             assert_eq!(err.expected, 2);
//             assert_eq!(err.found, 3);
//         } else {
//             panic!("Expected UnexpectedArgs error")
//         }
//     }
//
//     #[test]
//     fn add_with_too_few_args() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_expr(&mut state, "add(1)");
//         assert_eq!(ty.get_name(&state), "Int");
//         assert_eq!(state.errors.len(), 1);
//         if let CheckError::UnexpectedArgs(err) = &state.errors[0] {
//             assert_eq!(err.expected, 2);
//             assert_eq!(err.found, 1);
//         } else {
//             panic!("Expected UnexpectedArgs error")
//         }
//     }
//
//     #[test]
//     fn expected_receiver() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_expr(&mut state, "factorial()");
//         assert_eq!(ty.get_name(&state), "Int");
//         assert_eq!(state.errors.len(), 1);
//         if let CheckError::MissingReceiver(err) = &state.errors[0] {
//             assert_eq!(err.expected, Ty::int());
//         } else {
//             panic!("Expected MissingReceiver error")
//         }
//     }
//
//     #[test]
//     fn expected_is_returned() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let expected = Ty::int();
//         parse_expr_with_expected(&mut state, &expected, "add(1, 2)");
//         assert_eq!(state.errors, vec![]);
//     }
//
//     #[test]
//     fn expected_is_not_returned() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let expected = Ty::string();
//         parse_expr_with_expected(&mut state, &expected, "add(1, 2)");
//         assert_eq!(state.errors.len(), 1);
//         if let CheckError::IsNotInstance(err) = &state.errors[0] {
//             assert_eq!(err.expected.get_name(&state), "String");
//             assert_eq!(err.found.get_name(&state), "Int");
//         } else {
//             panic!("Expected IsNotInstance error")
//         }
//     }
//
//     #[test]
//     fn args_imply_type_vars() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_expr(&mut state, "ident(\"Hello\")");
//         state.resolve_type_vars();
//         assert_eq!(ty.get_name(&state), "String");
//         assert_eq!(state.errors, vec![]);
//     }
//
//     #[test]
//     fn args_imply_type_vars_with_wrong_arg_ty() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         parse_expr_with_expected(&mut state, &Ty::string(), "ident(1)");
//         state.resolve_type_vars();
//         assert_eq!(state.errors.len(), 1);
//         if let CheckError::IsNotInstance(err) = &state.errors[0] {
//             assert_eq!(err.expected.get_name(&state), "String");
//             assert_eq!(err.found.get_name(&state), "Int");
//             assert_eq!(err.span, (6..7).into());
//         } else {
//             panic!("Expected IsNotInstance error")
//         }
//     }
// }
