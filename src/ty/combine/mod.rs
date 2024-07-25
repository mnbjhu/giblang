use crate::{project::Project, ty::combine::named::get_shared_named_subtype};

use super::Ty;

pub mod named;

impl Ty {
    pub fn get_shared_subtype(&self, other: &Ty, project: &Project) -> Ty {
        if self.is_instance_of(other, project) {
            return other.clone();
        } else if other.is_instance_of(self, project) {
            return self.clone();
        }
        match (self, other) {
            (_, Ty::Any) | (Ty::Any, _) => Ty::Any,
            (Ty::Unknown, other) => other.clone(),
            (s, Ty::Unknown) => s.clone(),
            (Ty::Tuple(v), ty) | (ty, Ty::Tuple(v)) => {
                if let Ty::Tuple(other) = ty {
                    if v.len() == other.len() {
                        return Ty::Tuple(
                            v.iter()
                                .zip(other)
                                .map(|(s, o)| s.clone().get_shared_subtype(o, project))
                                .collect(),
                        );
                    }
                }
                Ty::Any
            }
            // TODO: Think about usecases for this
            (Ty::Meta(_), _) | (_, Ty::Meta(_)) => Ty::Any,
            (Ty::Prim(s), Ty::Prim(o)) => {
                if s == o {
                    self.clone()
                } else {
                    Ty::Any
                }
            }
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                let mut new = vec![];
                fn insert_ty(ty: Ty, new: &mut Vec<Ty>) {
                    if !new.iter().any(|t| t.equals(&ty)) {
                        new.push(ty)
                    }
                }
                match get_shared_named_subtype(other, *name, args, project) {
                    Ty::Any => {}
                    Ty::Sum(v) => {
                        for ty in v {
                            insert_ty(ty, &mut new)
                        }
                    }
                    ty => insert_ty(ty, &mut new),
                }
                match get_shared_named_subtype(self, *other_name, other_args, project) {
                    Ty::Any => {}
                    Ty::Sum(v) => {
                        for ty in v {
                            insert_ty(ty, &mut new)
                        }
                    }
                    ty => insert_ty(ty, &mut new),
                }
                match new.len() {
                    0 => Ty::Any,
                    1 => new[0].clone(),
                    _ => Ty::Sum(new),
                }
            }
            (Ty::Named { name, args }, other) | (other, Ty::Named { name, args }) => {
                get_shared_named_subtype(other, *name, args, project)
            }
            _ => todo!(),
        }
    }
}
