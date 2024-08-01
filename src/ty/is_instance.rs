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
}
