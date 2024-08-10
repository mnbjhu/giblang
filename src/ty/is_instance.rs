use crate::{
    check::state::CheckState, parser::common::variance::Variance, project::TypeVar, ty::Ty,
};

use super::Generic;

impl Ty {
    pub fn is_instance_of(&self, other: &Ty, state: &mut CheckState, imply: bool) -> bool {
        if self.equals(other) {
            return true;
        }
        match (self, other) {
            (Ty::Unknown, _) | (_, Ty::Unknown | Ty::Any) => true,
            (Ty::Any, _) => false,
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                let decl = state.project.get_decl(*name);
                let generics = decl.generics();
                if name == other_name {
                    args.len() == other_args.len()
                        && args.iter().zip(other_args).zip(generics.iter()).all(
                            |((first, second), def)| match def.variance {
                                Variance::Invariant => first.equals(second),
                                Variance::Covariant => first.is_instance_of(second, state, imply),
                                Variance::Contravariant => {
                                    second.is_instance_of(first, state, imply)
                                }
                            },
                        )
                } else {
                    let impls = state.project.get_impls(*name);
                    for impl_ in impls {
                        let ty = impl_.map(&self, state);
                        return ty.is_instance_of(other, state, imply);
                    }
                    false
                }
            }
            (Ty::TypeVar { id }, other) => {
                println!("TypeVar: {} {:?}", id, other);
                let var = state.get_type_var(*id);
                let s = var.map(|var| var.ty.clone()).unwrap_or(None);
                let super_ = var
                    .map(|var| var.generic.clone().super_.as_ref().clone())
                    .unwrap_or(Ty::Any);
                if let Some(s) = s {
                    return s.is_instance_of(other, state, imply);
                } else {
                    if imply {
                        state.add_type_bound(*id, other.clone());
                    }
                    super_.is_instance_of(other, state, imply)
                }
            }
            (_, Ty::TypeVar { id }) => {
                println!("TypeVar: {} {:?}", id, self);
                let var = state.get_type_var(*id);
                let ty = var.map(|var| var.ty.clone()).unwrap_or(None);
                let super_ = var
                    .map(|var| var.generic.clone().super_.as_ref().clone())
                    .unwrap_or(Ty::Any);
                if let Some(ty) = ty {
                    return self.is_instance_of(&ty, state, imply);
                } else {
                    if imply {
                        state.add_type_bound(*id, self.clone());
                    }
                    self.is_instance_of(&super_, state, imply)
                }
            }
            (_, Ty::Sum(tys)) => tys
                .iter()
                .all(|other| self.is_instance_of(other, state, imply)),
            (Ty::Sum(tys), _) => tys.iter().any(|ty| ty.is_instance_of(other, state, imply)),
            (Ty::Tuple(v), Ty::Tuple(other)) => {
                v.len() == other.len()
                    && v.iter()
                        .zip(other)
                        .all(|(s, o)| s.is_instance_of(o, state, imply))
            }
            (Ty::Generic(Generic { super_, .. }), _) => super_.is_instance_of(other, state, imply),
            (_, Ty::Generic(Generic { super_, .. })) => self.is_instance_of(super_, state, imply),
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
                    && args
                        .iter()
                        .zip(other_args)
                        .all(|(first, second)| second.is_instance_of(first, state, imply))
                    && ret.is_instance_of(other_ret, state, imply)
                    && receiver.as_ref().map_or(true, |r| {
                        other_receiver
                            .as_ref()
                            .map_or(false, |o| o.is_instance_of(r, state, imply))
                    })
            }
            _ => self.equals(other),
        }
    }

    // TODO: Finish impl
    #[allow(dead_code, clippy::only_used_in_recursion)]
    fn get_member_func(&self, name: &str, state: &CheckState) -> Option<Ty> {
        match self {
            Ty::Any | Ty::Unknown | Ty::Meta(_) | Ty::Function { .. } => None,
            Ty::Named { .. } => {
                todo!()
            }
            Ty::Generic(Generic { super_, .. }) => super_.get_member_func(name, state),
            Ty::Tuple(_) => todo!(),
            Ty::Sum(v) => v.iter().find_map(|ty| ty.get_member_func(name, state)),
            Ty::TypeVar { id } => {
                let var = state.get_type_var(*id);
                if let Some(TypeVar { ty: Some(ty), .. }) = var {
                    ty.get_member_func(name, state)
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
macro_rules! assert_instance_of {
    ($s:expr, $o:expr) => {
        let project = $crate::project::Project::ty_test();
        let file_data = project.get_file(project.get_counter()).unwrap();
        let mut state = $crate::check::state::CheckState::from_file(file_data, &project);
        let s = $crate::check::ty::tests::parse_ty_with_state(&project, &mut state, $s);
        if let $crate::ty::Ty::Unknown = s {
            panic!("First type is unknown");
        }
        let o = $crate::check::ty::tests::parse_ty(&project, $o);
        if let $crate::ty::Ty::Unknown = o {
            panic!("Second type is unknown");
        }
        let res = s.is_instance_of(&o, &mut state, false);
        if !res {
            panic!(
                "Expected {} to be an instance of {}",
                s.get_name(&state),
                o.get_name(&state)
            );
        }
    };
}

#[cfg(test)]
macro_rules! assert_not_instance_of {
    ($s:expr, $o:expr) => {
        let project = $crate::project::Project::ty_test();
        let file_data = project.get_file(project.get_counter()).unwrap();
        let mut state = $crate::check::state::CheckState::from_file(file_data, &project);
        let s = $crate::check::ty::tests::parse_ty_with_state(&project, &mut state, $s);
        if let $crate::ty::Ty::Unknown = s {
            panic!("First type is unknown");
        }
        let o = $crate::check::ty::tests::parse_ty(&project, $o);
        if let $crate::ty::Ty::Unknown = o {
            panic!("Second type is unknown");
        }
        let res = s.is_instance_of(&o, &mut state, false);
        if res {
            panic!(
                "Expected {} to no be an instance of {}",
                s.get_name(&state),
                o.get_name(&state),
            );
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::project::Project;

    impl Project {
        #[must_use]
        pub fn ty_test() -> Project {
            let mut project = Project::from(
                r"struct Foo
            struct Bar
            struct Baz[T]
            trait Magic {
                fn magic(): Self
            }
            trait Epic {
                fn epic(): Self
            }
            trait Strange [T] {
                fn strange(): T
            }

            impl Magic for Foo

            impl Magic for Bar
            impl Epic for Bar

            impl Strange[T] for Baz[T]",
            );
            project.resolve();
            project
        }
    }

    #[test]
    fn any() {
        assert_instance_of!("Any", "Any");
        assert_not_instance_of!("Any", "Foo");
        assert_instance_of!("Foo", "Any");
    }

    #[test]
    fn named() {
        assert_instance_of!("Foo", "Foo");
        assert_instance_of!("Foo", "Magic");
        assert_not_instance_of!("Magic", "Foo");
        assert_not_instance_of!("Foo", "Bar");
        assert_instance_of!("Magic", "Magic");
    }

    #[test]
    fn sum() {
        assert_instance_of!("Foo + Bar", "Foo");
        assert_instance_of!("Foo + Bar", "Bar");
        assert_not_instance_of!("Bar", "Foo + Bar");
        assert_instance_of!("Foo + Bar", "Foo + Bar");
        assert_instance_of!("Foo + Bar", "Bar + Foo");
    }
}
