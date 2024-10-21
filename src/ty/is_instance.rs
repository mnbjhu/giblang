use crate::{
    check::{
        err::{is_not_instance::IsNotInstance, CheckError},
        state::CheckState,
    },
    db::modules::ModulePath,
    parser::common::variance::Variance,
    project::{decl::DeclKind, ImplForDecl},
    ty::Ty,
    util::{Span, Spanned},
};

use super::{FuncTy, Generic};

impl<'db> Ty<'db> {
    pub fn expect_is_instance_of(
        &self,
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
                expect_named_is_instance_of_named(self, other, state, explicit, span)
            }
            (_, Ty::Sum(tys)) => tys
                .iter()
                .all(|other| self.expect_is_instance_of(other, state, explicit, span)),
            // TODO: Fix or remove
            (Ty::Sum(tys), _) => tys
                .iter()
                .any(|ty| ty.expect_is_instance_of(other, state, explicit, span)),
            (Ty::Tuple(v), Ty::Tuple(other)) => {
                v.len() == other.len()
                    && v.iter()
                        .zip(other)
                        .all(|(s, o)| s.expect_is_instance_of(o, state, explicit, span))
            }
            (Ty::Generic(Generic { super_, .. }), _) => {
                super_.expect_is_instance_of(other, state, explicit, span)
            }
            (_, Ty::Generic(Generic { super_, .. })) => {
                self.expect_is_instance_of(super_, state, explicit, span)
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
                        second.expect_is_instance_of(first, state, explicit, span)
                    })
                    && ret.expect_is_instance_of(other_ret, state, explicit, span)
                    && receiver.as_ref().map_or(true, |r| {
                        other_receiver
                            .as_ref()
                            .map_or(false, |o| o.expect_is_instance_of(r, state, explicit, span))
                    })
            }
            _ => false,
        };
        if !res {
            state.error(CheckError::IsNotInstance(IsNotInstance {
                span,
                found: self.get_name(state),
                expected: other.get_name(state),
                file: state.file_data,
            }));
        }
        res
    }

    fn imply_named_sub_ty(
        &self,
        sub_ty: ModulePath<'db>,
        state: &mut CheckState<'_, 'db>,
    ) -> Option<Ty<'db>> {
        if let Ty::Named { name, .. } = self {
            let path = path_to_sub_ty(*name, sub_ty, state)?;
            let mut ty = self.clone();
            for impl_ in path {
                ty = impl_.map(state.db, &ty);
            }
            Some(ty)
        } else {
            None
        }
    }
}

pub fn get_sub_decls<'db>(
    name: ModulePath<'db>,
    state: &mut CheckState<'_, 'db>,
) -> Vec<ModulePath<'db>> {
    state
        .project
        .get_impls(state.db, name)
        .into_iter()
        .filter(|i| i.to_ty(state.db).is_some())
        .flat_map(|i| {
            if let Ty::Named { name, .. } = i.to_ty(state.db).unwrap() {
                let mut sub = get_sub_decls(name, state);
                sub.push(name);
                sub
            } else {
                unreachable!()
            }
        })
        .collect()
}

pub fn get_sub_tys<'db>(name: &Ty<'db>, state: &mut CheckState<'_, 'db>) -> Vec<Ty<'db>> {
    match name {
        Ty::Named { name, .. } => state
            .project
            .get_impls(state.db, *name)
            .into_iter()
            .filter(|i| i.to_ty(state.db).is_some())
            .flat_map(|i| {
                let ty = i.to_ty(state.db);
                let mut tys = get_sub_tys(ty.as_ref().unwrap(), state);
                tys.push(ty.unwrap());
                tys
            })
            .collect(),
        _ => vec![],
    }
}

impl<'db> Ty<'db> {
    pub fn get_func(
        &self,
        name: &Spanned<String>,
        state: &mut CheckState<'_, 'db>,
    ) -> Option<FuncTy<'db>> {
        if let Ty::Named { name: id, .. } = self {
            if let DeclKind::Trait { body, .. } = state
                .project
                .get_decl(state.db, *id)
                .unwrap()
                .kind(state.db)
            {
                let found = body.iter()
                    .find(|func| func.name(state.db) == name.0)
                    .map(|func| {
                        let Ty::Function(func) = func.get_ty(state, Some(self.clone()), name.1)
                        else {
                            panic!("Expected function");
                        };
                        func
                    });
                    if let Some(found) = found {
                        return Some(found);
                    }
            };
            let impl_func = state
                .project
                .get_impls(state.db, *id)
                .into_iter()
                .filter(|i| {
                    let Ty::Named { name: n, .. } = i.from_ty(state.db) else {
                        unreachable!()
                    };
                    *id == n && i.to_ty(state.db).is_none()
                })
                .flat_map(|i| {
                    i.functions(state.db)
                })
                .find(|decl| decl.name(state.db) == name.0)
                .map(|decl| {
                    let Ty::Function(func) = decl.get_ty(decl.path(state.db), state)
                    else {
                        panic!("Expected function");
                    };
                    func
                });
            impl_func
        } else {
            None
        }
    }

    pub fn get_funcs(&self, state: &mut CheckState<'_, 'db>) -> Vec<(String, FuncTy<'db>)> {
        let mut funcs = Vec::new();
        if let Ty::Named { name, .. } = self {
            if let DeclKind::Trait { body, .. } = state
                .project
                .get_decl(state.db, *name)
                .unwrap()
                .kind(state.db)
            {
                funcs.extend(body
                    .iter()
                    .map(|mod_| {
                        let Ty::Function(func) =
                            mod_.get_ty(state, Some(self.clone()), Span::splat(0))
                        else {
                            panic!("Expected function");
                        };
                        (mod_.name(state.db), func)
                    })
                );
            }
            let impls = state
                .project
                .get_impls(state.db, *name)
                .into_iter()
                .filter(|i| {
                    let Ty::Named { name: n, .. } = i.from_ty(state.db) else {
                        unreachable!()
                    };
                    *name == n && i.to_ty(state.db).is_none()
                })
                .flat_map(|i| {
                    i.functions(state.db)
                })
                .map(|decl| {
                    let Ty::Function(func) =
                        decl.get_ty(decl.path(state.db) ,state)
                    else {
                        panic!("Expected function");
                    };
                    (decl.name(state.db), func)
                })
            ;
            funcs.extend(impls);
        }
        funcs
    }
}

fn path_to_sub_ty<'db>(
    name: ModulePath<'db>,
    sub_ty: ModulePath<'db>,
    state: &mut CheckState<'_, 'db>,
) -> Option<Vec<ImplForDecl<'db>>> {
    if name == sub_ty {
        return Some(Vec::new());
    }
    let (id, mut path) = state
        .project
        .get_impls(state.db, name)
        .into_iter()
        .filter(|i| i.to_ty(state.db).is_some())
        .map(|i| {
            if let Ty::Named { name, .. } = i.to_ty(state.db).unwrap() {
                (i, name)
            } else {
                unreachable!()
            }
        })
        .find_map(|(id, n)| path_to_sub_ty(n, sub_ty, state).map(|p| (id, p)))?;
    path.insert(0, id);
    Some(path)
}

fn expect_named_is_instance_of_named<'db>(
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
        }) = first.imply_named_sub_ty(*other_name, state)
        {
            // TODO: Check this unwrap
            let decl = state.project.get_decl(state.db, *name).unwrap();
            let generics = decl.generics(state.db);
            for ((g, arg), other) in generics.iter().zip(implied_args).zip(other_args) {
                let variance = g.variance;
                match variance {
                    Variance::Invariant => {
                        arg.expect_is_instance_of(other, state, explicit, span);
                        other.expect_is_instance_of(&arg, state, explicit, span);
                    }
                    Variance::Covariant => {
                        arg.expect_is_instance_of(other, state, explicit, span);
                    }
                    Variance::Contravariant => {
                        other.expect_is_instance_of(&arg, state, explicit, span);
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
