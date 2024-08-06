use crate::ty::Ty;

use super::Generic;

impl<'module> Ty {
    pub fn equals(&'module self, other: &Ty) -> bool {
        match (&self, other) {
            (Ty::Unknown, _) | (_, Ty::Unknown) | (Ty::Any, Ty::Any) => true,
            (
                Ty::Generic(Generic {
                    variance, super_, ..
                }),
                Ty::Generic(Generic {
                    variance: other_variance,
                    super_: other_super,
                    ..
                }),
            ) => super_.equals(other_super) && variance == other_variance,
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                name == other_name
                    && args.len() == other_args.len()
                    && args
                        .iter()
                        .zip(other_args)
                        .all(|(first, second)| first.equals(second))
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
                receiver.as_ref().map_or(true, |r| {
                    other_receiver.as_ref().map_or(false, |o| r.equals(o))
                }) && args.len() == other_args.len()
                    && args
                        .iter()
                        .zip(other_args)
                        .all(|(first, second)| first.equals(second))
                    && ret.equals(other_ret)
            }
            (Ty::Tuple(tys), Ty::Tuple(other_tys)) => {
                tys.len() == other_tys.len()
                    && tys
                        .iter()
                        .zip(other_tys)
                        .all(|(first, second)| first.equals(second))
            }
            _ => false,
        }
    }
}
