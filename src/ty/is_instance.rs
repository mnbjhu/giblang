use crate::{fs::project::Project, parser::common::variance::Variance, ty::Ty};

use super::Generic;

impl<'module> Ty<'module> {
    pub fn is_instance_of(&'module self, other: &Ty<'module>, project: &Project) -> bool {
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
                if name.id() == other_name.id() {
                    args.len() == other_args.len()
                        && args
                            .iter()
                            .zip(other_args)
                            .zip(name.generic_args().0.iter())
                            .all(|((first, second), def)| match def.0.variance {
                                Variance::Invariant => first.equals(second),
                                Variance::Covariant => first.is_instance_of(second, project),
                                Variance::Contravariant => second.is_instance_of(first, project),
                            })
                } else if let Some(impls) = &name.impls() {
                    impls
                        .iter()
                        .filter_map(|impl_| impl_.map(self.clone(), project))
                        .any(|implied| implied.is_instance_of(other, project))
                } else {
                    false
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
    fn get_member_func(&'module self, name: &str, project: &Project) -> Option<Ty<'module>> {
        match self {
            Ty::Any => None,
            Ty::Unknown => None,
            Ty::Named { name, args } => {
                todo!()
            }
            Ty::Generic(Generic { super_, .. }) => super_.get_member_func(name, project),
            Ty::Prim(_) => todo!(),
            Ty::Meta(_) => None,
            Ty::Function {
                receiver,
                args,
                ret,
            } => None,
            Ty::Tuple(_) => todo!(),
            Ty::Sum(v) => v.iter().find_map(|ty| ty.get_member_func(name, project)),
        }
    }
}
