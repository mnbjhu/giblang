use crate::{
    fs::{export::Export, project::Project},
    parser::common::variance::Variance,
    ty::Ty,
};

pub fn get_shared_named_subtype<'module>(
    other: &Ty<'module>,
    name: &Export<'module>,
    args: &[Ty<'module>],
    project: &'module Project,
) -> Ty<'module> {
    if let Ty::Named {
        name: other_name,
        args: other_args,
    } = &other
    {
        if name.id() == other_name.id() && args.len() == other_args.len() {
            let iter = args
                .iter()
                .zip(other_args)
                .zip(name.generic_args().0.iter());
            let mut args: Vec<Ty<'_>> = vec![];
            for ((first, second), def) in iter {
                match def.0.variance {
                    Variance::Invariant => {
                        if first.equals(second) {
                            args.push(first.clone());
                        }
                    }
                    Variance::Covariant => args.push(first.get_shared_subtype(second, project)),
                    Variance::Contravariant => todo!(),
                };
            }
            if args.len() == name.generic_args().0.len() {
                return Ty::Named {
                    name: name.clone(),
                    args,
                };
            }
        }
    }
    if let Some(impls) = name.impls() {
        let mut shared = vec![];
        for impl_ in impls.iter() {
            if let Some(ty) = impl_.map(
                Ty::Named {
                    name: name.clone(),
                    args: args.to_vec(),
                },
                project,
            ) {
                let found = ty.get_shared_subtype(other, project);
                if let Ty::Sum(v) = found {
                    shared.extend(v);
                } else if let Ty::Any = found {
                } else {
                    shared.push(found)
                }
            }
        }
        if shared.is_empty() {
            return Ty::Any;
        } else if shared.len() == 1 {
            return shared[0].clone();
        }
        return Ty::Sum(shared);
    }
    Ty::Any
}
