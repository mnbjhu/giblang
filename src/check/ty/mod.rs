use salsa::Database;

use crate::{
    check::state::CheckState,
    parser::common::type_::Type,
    ty::{FuncTy, Generic, Ty},
    util::Span,
};
pub mod named;

impl Type {
    pub fn check(&self, state: &mut CheckState) -> Ty {
        match &self {
            Type::Named(named) => named.check(state),
            Type::Tuple(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(state));
                }
                Ty::Tuple(tys)
            }
            Type::Sum(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(state));
                }
                Ty::Sum(tys)
            }
            Type::Function {
                receiver,
                args,
                ret,
            } => Ty::Function(FuncTy {
                receiver: receiver
                    .as_ref()
                    .map(|receiver| Box::new(receiver.as_ref().0.check(state))),
                args: args.iter().map(|r| r.0.check(state)).collect(),
                ret: Box::new(ret.0.check(state)),
            }),
            Type::Wildcard(s) => {
                let id = state.type_state.new_type_var(*s);
                Ty::TypeVar { id }
            }
        }
    }

    pub fn expect_is_bound_by<'db>(
        &self,
        db: &'db dyn Database,
        bound: &Generic<'db>,
        state: &mut CheckState<'_, 'db>,
        span: Span,
    ) -> Ty {
        let ty = self.check(state);
        if let Ty::TypeVar { id } = ty {
            state.type_state.add_bound(id, bound.clone());
        } else {
            ty.expect_is_instance_of(db, &bound.super_, state, false, span);
        }
        ty
    }
}

// #[cfg(test)]
// pub mod tests {
//     use chumsky::{input::Input, Parser};
//
//     use crate::{
//         check::{err::CheckError, state::CheckState},
//         lexer::parser::lexer,
//         parser::common::type_::type_parser,
//         project::{check_test_state, Project},
//         util::Span,
//     };
//
//     use super::Ty;
//
//     pub fn try_parse_ty(project: &Project, ty: &str) -> (Ty, Vec<CheckError>) {
//         let eoi = Span::splat(ty.len());
//         let tokens = lexer().parse(ty).unwrap();
//         let ty = type_parser().parse(tokens.spanned(eoi)).unwrap();
//         let file_data = project.get_file(project.get_counter()).unwrap();
//         let mut state = CheckState::from_file(file_data, project);
//         (ty.check(&mut state), state.errors)
//     }
//
//     pub fn try_parse_ty_with_state(state: &mut CheckState, ty: &str) -> (Ty, Vec<CheckError>) {
//         let eoi = Span::splat(ty.len());
//         let tokens = lexer().parse(ty).unwrap();
//         let ty = type_parser().parse(tokens.spanned(eoi)).unwrap();
//         (ty.check(state), state.errors.clone())
//     }
//
//     pub fn parse_ty_with_state(state: &mut CheckState, ty: &str) -> Ty {
//         let eoi = Span::splat(ty.len());
//         let tokens = lexer().parse(ty).unwrap();
//         let ty = type_parser().parse(tokens.spanned(eoi)).unwrap();
//         assert_eq!(state.errors, vec![]);
//         ty.check(state)
//     }
//
//     pub fn parse_ty(project: &Project, ty: &str) -> Ty {
//         let eoi = Span::splat(ty.len());
//         let tokens = lexer().parse(ty).unwrap();
//         let ty = type_parser().parse(tokens.spanned(eoi)).unwrap();
//         let file_data = project.get_file(project.get_counter()).unwrap();
//         let mut state = CheckState::from_file(file_data, project);
//         assert_eq!(state.errors, vec![]);
//         ty.check(&mut state)
//     }
//
//     #[test]
//     fn check_unit() {
//         let project = Project::check_test();
//         let (unit, err) = try_parse_ty(&project, "()");
//         assert_eq!(err, vec![]);
//         if let Ty::Tuple(tys) = unit {
//             assert_eq!(tys.len(), 0);
//         } else {
//             panic!("Expected unit type to be a tuple")
//         }
//     }
//
//     #[test]
//     fn check_string() {
//         let project = Project::check_test();
//         let (string, err) = try_parse_ty(&project, "String");
//         assert_eq!(err, vec![]);
//
//         if let Ty::Named { name, args } = string {
//             assert_eq!(name, 1);
//             assert_eq!(args.len(), 0);
//         } else {
//             panic!("Expected string type to be a named type")
//         }
//     }
//
//     #[test]
//     fn check_foo() {
//         let project = Project::check_test();
//         let (foo, err) = try_parse_ty(&project, "Foo");
//         assert_eq!(err, vec![]);
//
//         if let Ty::Named { name, args } = foo {
//             assert_eq!(project.get_decl(name).name(), "Foo");
//             assert_eq!(args.len(), 0);
//         } else {
//             panic!("Expected foo type to be a named type")
//         }
//     }
//
//     #[test]
//     fn check_bar() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let (bar, _) = try_parse_ty_with_state(&mut state, "Bar[Foo]");
//         state.resolve_type_vars();
//         assert_eq!(state.errors, vec![]);
//         if let Ty::Named { name, args } = bar {
//             assert_eq!(project.get_decl(name).name(), "Bar");
//             assert_eq!(args.len(), 1);
//             assert_eq!(args[0], parse_ty(&project, "Foo"));
//         } else {
//             panic!("Expected bar type to be a named type")
//         }
//     }
//
//     #[test]
//     fn check_tuple() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let (tuple, _) = try_parse_ty_with_state(&mut state, "(Foo, Bar[Foo])");
//         assert_eq!(state.type_state.vars.len(), 0);
//         state.resolve_type_vars();
//         assert_eq!(state.errors, vec![]);
//         if let Ty::Tuple(tys) = tuple {
//             assert_eq!(tys.len(), 2);
//             if let Ty::Named { name, args } = &tys[0] {
//                 assert_eq!(project.get_decl(*name).name(), "Foo");
//                 assert_eq!(args.len(), 0);
//             } else {
//                 panic!("Expected first element of tuple to be a named type")
//             }
//             if let Ty::Named { name, args } = &tys[1] {
//                 assert_eq!(project.get_decl(*name).name(), "Bar");
//                 assert_eq!(args.len(), 1);
//                 assert_eq!(args[0], parse_ty(&project, "Foo"));
//             } else {
//                 panic!("Expected second element of tuple to be a named type")
//             }
//         } else {
//             panic!("Expected tuple type to be a tuple")
//         }
//     }
//
//     #[test]
//     fn check_unresolved() {
//         let project = Project::check_test();
//         let (unresolved, err) = try_parse_ty(&project, "Unresolved");
//         assert_eq!(err.len(), 1);
//         if let CheckError::Unresolved(err) = &err[0] {
//             assert_eq!(err.name.0, "Unresolved");
//         } else {
//             panic!("Expected unresolved error")
//         }
//         assert_eq!(unresolved, Ty::Unknown);
//     }
// }
