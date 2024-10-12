use crate::{check::state::CheckState, parser::stmt::let_::LetStatement};

impl LetStatement {
    pub fn check(&self, state: &mut CheckState) {
        let ty = if let Some(expected) = &self.ty {
            let expected = expected.0.check(state);
            self.value
                .0
                .expect_instance_of(&expected, state, self.value.1);
            expected
        } else {
            self.value.0.check(state)
        };
        self.pattern.0.check(state, ty);
    }
}

// #[cfg(test)]
// mod tests {
//     use chumsky::{input::Input, Parser};
//
//     use crate::{
//         check::{err::CheckError, state::CheckState, ty::tests::parse_ty_with_state},
//         lexer::parser::lexer,
//         parser::{
//             expr::expr_parser,
//             stmt::{let_::let_parser, stmt_parser},
//         },
//         project::{check_test_state, Project},
//         ty::Ty,
//         util::Span,
//     };
//
//     fn check_let(text: &'static str, state: &mut CheckState) -> Vec<CheckError> {
//         let tokens = lexer().parse(text).unwrap();
//         let input = tokens.spanned(Span::splat(text.len()));
//         let parser = let_parser(expr_parser(stmt_parser()));
//         let let_ = parser.parse(input).unwrap();
//         state.enter_scope();
//         let_.check(state);
//         state.resolve_type_vars();
//         state.errors.clone()
//     }
//
//     #[test]
//     fn test_let() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let errors = check_let("let x = 5", &mut state);
//         assert!(errors.is_empty());
//         let ty = state
//             .get_variable("x")
//             .expect("Expected state to have variable x");
//         assert_eq!(*ty, Ty::int());
//     }
//
//     #[test]
//     fn test_let_ty() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let errors = check_let("let x: Abc = 5", &mut state);
//         assert_eq!(errors.len(), 1);
//         let error = errors.first().unwrap();
//         if let CheckError::Unresolved(unresolved) = error {
//             assert_eq!(unresolved.name.0, "Abc");
//         } else {
//             panic!("Expected Unresolved error");
//         }
//         let ty = state
//             .get_variable("x")
//             .expect("Expected state to have variable x");
//         assert_eq!(*ty, Ty::Unknown);
//     }
//
//     #[test]
//     fn imply_option() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         check_let("let x = Option::Some(5)", &mut state);
//         state.resolve_type_vars();
//         assert_eq!(state.errors, vec![]);
//
//         let ty = state
//             .get_variable("x")
//             .expect("Expected state to have variable x");
//
//         if let Ty::Named { name, args } = ty {
//             assert_eq!(project.get_decl(*name).name(), "Option");
//             assert_eq!(args.len(), 1);
//             if let Ty::TypeVar { id } = args[0] {
//                 let resolved = state.get_resolved_type_var(id);
//                 assert_eq!(resolved, Ty::int());
//             }
//         } else {
//             panic!("Expected Named ty");
//         }
//     }
//
//     #[test]
//     fn imply_option_with_wildcard() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         check_let("let x: Option[_] = Option::Some(5)", &mut state);
//         state.resolve_type_vars();
//         assert_eq!(state.errors, vec![]);
//         let ty = state
//             .get_variable("x")
//             .expect("Expected state to have variable x");
//         if let Ty::Named { name, args } = ty {
//             assert_eq!(project.get_decl(*name).name(), "Option");
//             assert_eq!(args.len(), 1);
//             if let Ty::TypeVar { id } = args[0] {
//                 let resolved = state.get_resolved_type_var(id);
//                 assert_eq!(resolved, Ty::int());
//             }
//         } else {
//             panic!("Expected Named ty");
//         }
//     }
//
//     #[test]
//     fn fails_to_imply_none() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         check_let("let x = Option::None", &mut state);
//         assert_eq!(state.type_state.vars.len(), 1);
//         assert_eq!(state.errors.len(), 1);
//
//         let ty = state
//             .get_variable("x")
//             .expect("Expected state to have variable x");
//
//         if let Ty::Named { name, args } = ty {
//             assert_eq!(project.get_decl(*name).name(), "Option");
//             assert_eq!(args.len(), 1);
//             if let Ty::TypeVar { id } = args[0] {
//                 let resolved = state.get_resolved_type_var(id);
//                 assert_eq!(resolved, Ty::Unknown);
//             }
//         } else {
//             panic!("Expected Named ty");
//         }
//     }
//
//     #[test]
//     fn test_unresolved_type_var() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let type_var = parse_ty_with_state(&mut state, "_");
//         assert_eq!(type_var, Ty::TypeVar { id: 0 });
//         state.resolve_type_vars();
//         assert_eq!(state.errors.len(), 1);
//         if let CheckError::UnboundTypeVar(unbound) = state.errors.first().unwrap() {
//             assert_eq!(unbound.name, "_");
//         } else {
//             panic!("Expected UnboundTypeVar error");
//         }
//     }
//
//     #[test]
//     fn test_imply_type() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let type_var = parse_ty_with_state(&mut state, "_");
//         let string_ty = parse_ty_with_state(&mut state, "String");
//         type_var.expect_is_instance_of(&string_ty, &mut state, false, Span::splat(0));
//         assert_eq!(type_var, Ty::TypeVar { id: 0 });
//         state.resolve_type_vars();
//         assert_eq!(state.errors.len(), 0);
//         assert_eq!(state.get_resolved_type_var(0), Ty::string());
//     }
//
//     #[test]
//     fn imply_int() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let errors = check_let("let x = 5", &mut state);
//         assert_eq!(errors, vec![]);
//         let ty = state
//             .get_variable("x")
//             .expect("Expected state to have variable x");
//         assert_eq!(*ty, Ty::int());
//     }
//
//     #[test]
//     fn imply_int_with_wildcard() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//
//         let errors = check_let("let x: _ = 5", &mut state);
//         assert_eq!(errors, vec![]);
//
//         let ty = state
//             .get_variable("x")
//             .expect("Expected state to have variable x");
//
//         if let Ty::TypeVar { id } = ty {
//             let resolved = state.get_resolved_type_var(*id);
//             assert_eq!(resolved, Ty::int());
//         }
//     }
// }
