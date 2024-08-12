use crate::{
    check::{
        err::{is_not_instance::IsNotInstance, CheckError},
        state::CheckState,
    },
    parser::common::variance::Variance,
    ty::Ty,
    util::Span,
};

use super::Generic;

impl Ty {
    pub fn expect_is_instance_of(
        &self,
        other: &Ty,
        state: &mut CheckState,
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
                state.type_state.expected_var_is_ty(*id, self.clone(), span);
                true
            }
            (_, Ty::Any) => true,
            (Ty::Named { .. }, Ty::Named { .. }) => {
                expect_named_is_instance_of_named(self, other, state, explicit, span)
            }
            (_, Ty::Sum(tys)) => tys
                .iter()
                .all(|other| self.expect_is_instance_of(other, state, explicit, span)),
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
                Ty::Function {
                    receiver,
                    args,
                    ret,
                },
                Ty::Function {
                    receiver: other_receiver,
                    args: other_args,
                    ret: other_ret,
                },
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
                first: self.clone(),
                second: other.clone(),
                file: state.file_data.end,
            }));
        }
        res
    }

    fn imply_named_sub_ty(&self, sub_ty: u32, state: &mut CheckState) -> Option<Ty> {
        if let Ty::Named { name, .. } = self {
            let path = path_to_sub_ty(*name, sub_ty, state)?;
            let mut ty = self.clone();
            for id in path {
                let impl_ = state.project.get_impl(&id);
                ty = impl_.map(&ty);
            }
            Some(ty)
        } else {
            None
        }
    }
}

fn path_to_sub_ty(name: u32, sub_ty: u32, state: &mut CheckState) -> Option<Vec<u32>> {
    if name == sub_ty {
        return Some(Vec::new());
    }
    let (id, mut path) = state
        .project
        .get_impls(name)
        .iter()
        .map(|i| {
            if let Ty::Named { name, .. } = &i.from {
                (i.id, name)
            } else {
                unreachable!()
            }
        })
        .find_map(|(id, n)| path_to_sub_ty(*n, sub_ty, state).map(|p| (id, p)))?;
    path.insert(0, id);
    Some(path)
}

fn expect_named_is_instance_of_named(
    first: &Ty,
    second: &Ty,
    state: &mut CheckState,
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
            let decl = state.project.get_decl(*name);
            let generics = decl.generics();
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

// impl Ty {
//     pub fn is_instance_of(&self, other: &Ty, state: &mut CheckState, imply: bool) -> bool {
//         if self.equals(other) {
//             return true;
//         }
//         match (self, other) {
//             (Ty::Unknown, _) | (_, Ty::Unknown | Ty::Any) => true,
//             (Ty::Any, _) => false,
//             (Ty::TypeVar { id }, other) => {
//                 let var = state.get_type_var(*id);
//                 let s = var.and_then(|var| var.ty.clone());
//                 let super_ = var.map_or(Ty::Any, |var| var.generic.super_.as_ref().clone());
//                 if let Some(s) = s {
//                     s.is_instance_of(other, state, imply)
//                 } else {
//                     if imply {
//                         state.add_type_bound(*id, other.clone());
//                     }
//                     super_.is_instance_of(other, state, imply)
//                 }
//             }
//             (_, Ty::TypeVar { id }) => {
//                 let var = state.get_type_var(*id);
//                 let ty = var.and_then(|var| var.ty.clone());
//                 let super_ = var.map_or(Ty::Any, |var| var.generic.super_.as_ref().clone());
//                 if let Some(ty) = ty {
//                     self.is_instance_of(&ty, state, imply)
//                 } else {
//                     if imply {
//                         state.add_type_bound(*id, self.clone());
//                     }
//                     self.is_instance_of(&super_, state, imply)
//                 }
//             }
//             (
//                 Ty::Named { name, args },
//                 Ty::Named {
//                     name: other_name,
//                     args: other_args,
//                 },
//             ) => {
//                 let decl = state.project.get_decl(*name);
//                 let generics = decl.generics();
//                 if name == other_name {
//                     args.len() == other_args.len()
//                         && args.iter().zip(other_args).zip(generics.iter()).all(
//                             |((first, second), def)| match def.variance {
//                                 Variance::Invariant => first.equals(second),
//                                 Variance::Covariant => first.is_instance_of(second, state, imply),
//                                 Variance::Contravariant => {
//                                     second.is_instance_of(first, state, imply)
//                                 }
//                             },
//                         )
//                 } else {
//                     let declared_impls = state.project.get_impls(*name);
//                     declared_impls.iter().any(|im| {
//                         let ty = im.map(self, state);
//                         ty.is_instance_of(other, state, imply)
//                     })
//                 }
//             }
//             (_, Ty::Sum(tys)) => tys
//                 .iter()
//                 .all(|other| self.is_instance_of(other, state, imply)),
//             (Ty::Sum(tys), _) => tys.iter().any(|ty| ty.is_instance_of(other, state, imply)),
//             (Ty::Tuple(v), Ty::Tuple(other)) => {
//                 v.len() == other.len()
//                     && v.iter()
//                         .zip(other)
//                         .all(|(s, o)| s.is_instance_of(o, state, imply))
//             }
//             (Ty::Generic(Generic { super_, .. }), _) => super_.is_instance_of(other, state, imply),
//             (_, Ty::Generic(Generic { super_, .. })) => self.is_instance_of(super_, state, imply),
//             (
//                 Ty::Function {
//                     receiver,
//                     args,
//                     ret,
//                 },
//                 Ty::Function {
//                     receiver: other_receiver,
//                     args: other_args,
//                     ret: other_ret,
//                 },
//             ) => {
//                 args.len() == other_args.len()
//                     && args
//                         .iter()
//                         .zip(other_args)
//                         .all(|(first, second)| second.is_instance_of(first, state, imply))
//                     && ret.is_instance_of(other_ret, state, imply)
//                     && receiver.as_ref().map_or(true, |r| {
//                         other_receiver
//                             .as_ref()
//                             .map_or(false, |o| o.is_instance_of(r, state, imply))
//                     })
//             }
//             _ => self.equals(other),
//         }
//     }
//
//     // TODO: Finish impl
//     #[allow(dead_code, clippy::only_used_in_recursion)]
//     fn get_member_func(&self, name: &str, state: &CheckState) -> Option<Ty> {
//         match self {
//             Ty::Any | Ty::Unknown | Ty::Meta(_) | Ty::Function { .. } => None,
//             Ty::Named { .. } => {
//                 todo!()
//             }
//             Ty::Generic(Generic { super_, .. }) => super_.get_member_func(name, state),
//             Ty::Tuple(_) => todo!(),
//             Ty::Sum(v) => v.iter().find_map(|ty| ty.get_member_func(name, state)),
//             Ty::TypeVar { id } => {
//                 let var = state.get_type_var(*id);
//                 if let Some(TypeVar { ty: Some(ty), .. }) = var {
//                     ty.get_member_func(name, state)
//                 } else {
//                     None
//                 }
//             }
//         }
//     }
// }
//
// #[cfg(test)]
// macro_rules! assert_instance_of {
//     ($s:expr, $o:expr) => {
//         let project = $crate::project::Project::ty_test();
//         let file_data = project.get_file(project.get_counter()).unwrap();
//         let mut state = $crate::check::state::CheckState::from_file(file_data, &project);
//         let s = $crate::check::ty::tests::parse_ty_with_state(&project, &mut state, $s);
//         if let $crate::ty::Ty::Unknown = s {
//             panic!("First type is unknown");
//         }
//         let o = $crate::check::ty::tests::parse_ty(&project, $o);
//         if let $crate::ty::Ty::Unknown = o {
//             panic!("Second type is unknown");
//         }
//         let res = s.is_instance_of(&o, &mut state, false);
//         if !res {
//             panic!(
//                 "Expected {} to be an instance of {}",
//                 s.get_name(&state),
//                 o.get_name(&state)
//             );
//         }
//     };
// }
//
// #[cfg(test)]
// macro_rules! assert_not_instance_of {
//     ($s:expr, $o:expr) => {
//         let project = $crate::project::Project::ty_test();
//         let file_data = project.get_file(project.get_counter()).unwrap();
//         let mut state = $crate::check::state::CheckState::from_file(file_data, &project);
//         let s = $crate::check::ty::tests::parse_ty_with_state(&project, &mut state, $s);
//         if let $crate::ty::Ty::Unknown = s {
//             panic!("First type is unknown");
//         }
//         let o = $crate::check::ty::tests::parse_ty(&project, $o);
//         if let $crate::ty::Ty::Unknown = o {
//             panic!("Second type is unknown");
//         }
//         let res = s.is_instance_of(&o, &mut state, false);
//         if res {
//             panic!(
//                 "Expected {} to no be an instance of {}",
//                 s.get_name(&state),
//                 o.get_name(&state),
//             );
//         }
//     };
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::project::Project;
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
//     fn any() {
//         assert_instance_of!("Any", "Any");
//         assert_not_instance_of!("Any", "Foo");
//         assert_instance_of!("Foo", "Any");
//     }
//
//     #[test]
//     fn named() {
//         assert_instance_of!("Foo", "Foo");
//         assert_instance_of!("Foo", "Magic");
//         assert_not_instance_of!("Magic", "Foo");
//         assert_not_instance_of!("Foo", "Bar");
//         assert_instance_of!("Magic", "Magic");
//     }
//
//     #[test]
//     fn sum() {
//         assert_instance_of!("Foo + Bar", "Foo");
//         assert_instance_of!("Foo + Bar", "Bar");
//         assert_not_instance_of!("Bar", "Foo + Bar");
//         assert_instance_of!("Foo + Bar", "Foo + Bar");
//         assert_instance_of!("Foo + Bar", "Bar + Foo");
//     }
// }
