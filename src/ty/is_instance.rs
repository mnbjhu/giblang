use salsa::Database;

use crate::{
    check::{
        err::{is_not_instance::IsNotInstance, CheckError},
        state::CheckState,
    },
    db::{input::Db, modules::ModulePath},
    parser::common::variance::Variance,
    project::ImplDecl,
    ty::Ty,
    util::Span,
};

use super::{FuncTy, Generic};

impl<'db> Ty<'db> {
    pub fn expect_is_instance_of(
        &self,
        db: &'db dyn Db,
        other: &Ty<'db>,
        state: &mut CheckState<'_, 'db>,
        explicit: bool,
        span: Span,
    ) -> bool {
        if self.eq(other) {
            return true;
        }
        let res = match (self, other) {
            (Ty::Unknown, _) | (_, Ty::Unknown) => true,
            (Ty::TypeVar { id }, _) => {
                if explicit {
                    state
                        .type_state
                        .add_explicit_type(*id, (other.clone(), span));
                } else {
                    state
                        .type_state
                        .expected_var_is_ty(*id, other.clone(), span);
                }
                true
            }
            (_, Ty::TypeVar { id }) => {
                state.type_state.expected_ty_is_var(*id, self.clone(), span);
                true
            }
            (_, Ty::Any) => true,
            (Ty::Named { .. }, Ty::Named { .. }) => {
                expect_named_is_instance_of_named(db, self, other, state, explicit, span)
            }
            (_, Ty::Sum(tys)) => tys
                .iter()
                .all(|other| self.expect_is_instance_of(db, other, state, explicit, span)),
            // TODO: Fix or remove
            (Ty::Sum(tys), _) => tys
                .iter()
                .any(|ty| ty.expect_is_instance_of(db, other, state, explicit, span)),
            (Ty::Tuple(v), Ty::Tuple(other)) => {
                v.len() == other.len()
                    && v.iter()
                        .zip(other)
                        .all(|(s, o)| s.expect_is_instance_of(db, o, state, explicit, span))
            }
            (Ty::Generic(Generic { super_, .. }), _) => {
                super_.expect_is_instance_of(db, other, state, explicit, span)
            }
            (_, Ty::Generic(Generic { super_, .. })) => {
                self.expect_is_instance_of(db, super_, state, explicit, span)
            }
            (
                Ty::Function(FuncTy {
                    receiver,
                    args,
                    ret,
                }),
                Ty::Function(FuncTy {
                    receiver: other_receiver,
                    args: other_args,
                    ret: other_ret,
                }),
            ) => {
                args.len() == other_args.len()
                    && args.iter().zip(other_args).all(|(first, second)| {
                        second.expect_is_instance_of(db, first, state, explicit, span)
                    })
                    && ret.expect_is_instance_of(db, other_ret, state, explicit, span)
                    && receiver.as_ref().map_or(true, |r| {
                        other_receiver.as_ref().map_or(false, |o| {
                            o.expect_is_instance_of(db, r, state, explicit, span)
                        })
                    })
            }
            _ => false,
        };
        if !res {
            state.error(CheckError::IsNotInstance(IsNotInstance {
                span,
                found: self.get_name(db, state),
                expected: other.get_name(db, state),
                file: state.file_data,
            }));
        }
        res
    }

    fn imply_named_sub_ty(
        &self,
        db: &'db dyn Db,
        sub_ty: ModulePath<'db>,
        state: &mut CheckState<'_, 'db>,
    ) -> Option<Ty<'db>> {
        if let Ty::Named { name, .. } = self {
            let path = path_to_sub_ty(db, *name, sub_ty, state)?;
            let mut ty = self.clone();
            for impl_ in path {
                ty = impl_.map(db, &ty);
            }
            Some(ty)
        } else {
            None
        }
    }
}

fn path_to_sub_ty<'db>(
    db: &'db dyn Db,
    name: ModulePath<'db>,
    sub_ty: ModulePath<'db>,
    state: &mut CheckState<'_, 'db>,
) -> Option<Vec<ImplDecl<'db>>> {
    if name == sub_ty {
        return Some(Vec::new());
    }
    let (id, mut path) = state
        .project
        .get_impls(db, name)
        .into_iter()
        .map(|i| {
            if let Ty::Named { name, .. } = i.to_ty(db) {
                (i, name)
            } else {
                unreachable!()
            }
        })
        .find_map(|(id, n)| path_to_sub_ty(db, n, sub_ty, state).map(|p| (id, p)))?;
    path.insert(0, id);
    Some(path)
}

fn expect_named_is_instance_of_named<'db>(
    db: &'db dyn Db,
    first: &Ty<'db>,
    second: &Ty<'db>,
    state: &mut CheckState<'_, 'db>,
    explicit: bool,
    span: Span,
) -> bool {
    if let (
        Ty::Named { name, .. },
        Ty::Named {
            name: other_name,
            args: other_args,
        },
    ) = (first, second)
    {
        if let Some(Ty::Named {
            args: implied_args, ..
        }) = first.imply_named_sub_ty(db, *other_name, state)
        {
            // TODO: Check this unwrap
            let decl = state.project.get_decl(db, *name).unwrap();
            let generics = decl.generics(db);
            for ((g, arg), other) in generics.iter().zip(implied_args).zip(other_args) {
                let variance = g.variance;
                match variance {
                    Variance::Invariant => {
                        arg.expect_is_instance_of(db, other, state, explicit, span);
                        other.expect_is_instance_of(db, &arg, state, explicit, span);
                    }
                    Variance::Covariant => {
                        arg.expect_is_instance_of(db, other, state, explicit, span);
                    }
                    Variance::Contravariant => {
                        other.expect_is_instance_of(db, &arg, state, explicit, span);
                    }
                }
            }
            true
        } else {
            false
        }
    } else {
        panic!("Expected named types")
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         check::{state::CheckState, ty::tests::parse_ty_with_state},
//         project::Project,
//         util::Span,
//     };
//
//     use super::path_to_sub_ty;
//
//     impl Project {
//         #[must_use]
//         pub fn ty_test() -> Project {
//             let mut project = Project::from(
//                 r"struct Foo
//             struct Bar
//             struct Baz[T]
//             trait Magic {
//                 fn magic(): Self
//             }
//             trait Epic {
//                 fn epic(): Self
//             }
//             trait Strange [T] {
//                 fn strange(): T
//             }
//
//             impl Magic for Foo
//
//             impl Magic for Bar
//             impl Epic for Bar
//
//             impl Strange[T] for Baz[T]",
//             );
//             project.resolve();
//             project
//         }
//     }
//
//     #[test]
//     fn test_path_to_subtype() {
//         let project = Project::ty_test();
//         let file_data = project.get_file(project.get_counter()).unwrap();
//         let mut state = CheckState::from_file(file_data, &project);
//         let foo = state
//             .get_decl_without_error(&[("Foo".to_string(), Span::splat(0))])
//             .unwrap();
//         let magic = state
//             .get_decl_without_error(&[("Magic".to_string(), Span::splat(0))])
//             .unwrap();
//
//         let path = path_to_sub_ty(foo, magic, &mut state).expect("Expected path");
//         assert_eq!(path.len(), 1);
//     }
//
//     fn assert_instance_of(first: &str, second: &str) {
//         let project = Project::ty_test();
//         let file_data = project.get_file(project.get_counter()).unwrap();
//         let mut state = CheckState::from_file(file_data, &project);
//         let first = parse_ty_with_state(&mut state, first);
//         let second = parse_ty_with_state(&mut state, second);
//         let res = first.expect_is_instance_of(&second, &mut state, false, Span::splat(0));
//         assert_eq!(state.errors, vec![]);
//         assert!(
//             res,
//             "{} is not an instance of {}",
//             first.get_name(&state),
//             second.get_name(&state)
//         );
//     }
//
//     fn assert_not_instance_of(first: &str, second: &str) {
//         let project = Project::ty_test();
//         let file_data = project.get_file(project.get_counter()).unwrap();
//         let mut state = CheckState::from_file(file_data, &project);
//         let first = parse_ty_with_state(&mut state, first);
//         let second = parse_ty_with_state(&mut state, second);
//         let res = first.expect_is_instance_of(&second, &mut state, false, Span::splat(0));
//         assert!(
//             !res,
//             "{} is an instance of {}",
//             first.get_name(&state),
//             second.get_name(&state)
//         );
//     }
//
//     #[test]
//     fn any() {
//         assert_instance_of("Any", "Any");
//         assert_not_instance_of("Any", "Foo");
//         assert_instance_of("Foo", "Any");
//     }
//
//     #[test]
//     fn foo_is_foo() {
//         assert_instance_of("Foo", "Foo");
//         assert_instance_of("Foo", "Magic");
//     }
//
//     #[test]
//     fn named() {
//         assert_not_instance_of("Magic", "Foo");
//         assert_not_instance_of("Foo", "Bar");
//         assert_instance_of("Magic", "Magic");
//     }
//
//     // TODO: Maybe remove?
//     // #[test]
//     // fn sum() {
//     //     assert_instance_of("Foo + Bar", "Foo");
//     //     assert_instance_of("Foo + Bar", "Bar");
//     // }
// }
