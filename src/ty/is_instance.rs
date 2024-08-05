use crate::{parser::common::variance::Variance, project::Project, ty::Ty};

use super::Generic;

impl Ty {
    pub fn is_instance_of(&self, other: &Ty, project: &Project) -> bool {
        if self.equals(other) {
            return true;
        }
        match (&self, other) {
            (Ty::Unknown, _) => true,
            (_, Ty::Unknown) => true,
            (Ty::Any, _) => false,
            (_, Ty::Any) => true,
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                let decl = project.get_decl(*name);
                let generics = decl.generics();
                if name == other_name {
                    args.len() == other_args.len()
                        && args.iter().zip(other_args).zip(generics.iter()).all(
                            |((first, second), def)| match def.variance {
                                Variance::Invariant => first.equals(second),
                                Variance::Covariant => first.is_instance_of(second, project),
                                Variance::Contravariant => second.is_instance_of(first, project),
                            },
                        )
                } else {
                    let impls = project.get_impls(*name);
                    impls
                        .iter()
                        .filter_map(|impl_| impl_.map(self, project))
                        .any(|implied| implied.is_instance_of(other, project))
                }
            }
            (_, Ty::Sum(tys)) => tys.iter().all(|other| self.is_instance_of(other, project)),
            (Ty::Sum(tys), _) => tys.iter().any(|ty| ty.is_instance_of(other, project)),
            (Ty::Tuple(v), Ty::Tuple(other)) => {
                v.len() == other.len()
                    && v.iter()
                        .zip(other)
                        .all(|(s, o)| s.is_instance_of(o, project))
            }
            (Ty::Generic(Generic { super_, .. }), _) => super_.is_instance_of(other, project),
            (_, Ty::Generic(Generic { super_, .. })) => self.is_instance_of(super_, project),
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
                        .all(|(first, second)| second.is_instance_of(first, project))
                    && ret.is_instance_of(other_ret, project)
                    && receiver.as_ref().map_or(true, |r| {
                        other_receiver
                            .as_ref()
                            .map_or(false, |o| o.is_instance_of(r, project))
                    })
            }
            _ => false,
        }
    }

    // TODO: Finish impl
    #[allow(dead_code, clippy::only_used_in_recursion)]
    fn get_member_func(&self, name: &str, project: &Project) -> Option<Ty> {
        match self {
            Ty::Any => None,
            Ty::Unknown => None,
            Ty::Named { .. } => {
                todo!()
            }
            Ty::Generic(Generic { super_, .. }) => super_.get_member_func(name, project),
            Ty::Meta(_) => None,
            Ty::Function { .. } => None,
            Ty::Tuple(_) => todo!(),
            Ty::Sum(v) => v.iter().find_map(|ty| ty.get_member_func(name, project)),
        }
    }
}

#[cfg(test)]
macro_rules! assert_instance_of {
    ($s:expr, $o:expr) => {
        let project = $crate::project::Project::ty_test();
        let s = $crate::check::ty::tests::parse_ty(&project, $s);
        if let $crate::ty::Ty::Unknown = s {
            panic!("First type is unknown");
        }
        let o = $crate::check::ty::tests::parse_ty(&project, $o);
        if let $crate::ty::Ty::Unknown = o {
            panic!("Second type is unknown");
        }
        let res = s.is_instance_of(&o, &project);
        if !res {
            panic!(
                "Expected {} to be instance of {}",
                s.get_name(&project),
                o.get_name(&project)
            );
        }
    };
}

#[cfg(test)]
macro_rules! assert_not_instance_of {
    ($s:expr, $o:expr) => {
        let project = $crate::project::Project::ty_test();
        let s = $crate::check::ty::tests::parse_ty(&project, $s);
        let o = $crate::check::ty::tests::parse_ty(&project, $o);
        let res = s.is_instance_of(&o, &project);
        if res {
            panic!(
                "Expected {} to not be instance of {}",
                s.get_name(&project),
                o.get_name(&project)
            );
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::project::Project;

    impl Project {
        pub fn ty_test() -> Project {
            let mut project = Project::from(
                r#"struct Foo
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

            impl Strange[T] for Baz[T]"#,
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
