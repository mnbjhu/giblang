use crate::ty::Ty;

impl<'module> Ty<'module> {
    pub fn equals(&'module self, other: &Ty<'module>) -> bool {
        match (&self, other) {
            (Ty::Any, Ty::Any) => true,
            (Ty::Unknown, Ty::Unknown) => true,
            (Ty::Prim(s), Ty::Prim(o)) => s == o,
            (
                Ty::Generic {
                    variance, super_, ..
                },
                Ty::Generic {
                    variance: other_variance,
                    super_: other_super,
                    ..
                },
            ) => super_.equals(other_super) && variance == other_variance,
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                name.id() == other_name.id()
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
