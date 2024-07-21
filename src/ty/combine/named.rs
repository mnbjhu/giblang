use crate::{parser::common::variance::Variance, project::Project, ty::Ty};

pub fn get_shared_named_subtype<'module>(
    other: &Ty,
    name: u32,
    args: &[Ty],
    project: &Project,
) -> Ty {
    if let Ty::Named {
        name: other_name,
        args: other_args,
    } = &other
    {
        let decl = project.get_decl(name);
        if name == *other_name && args.len() == other_args.len() {
            let iter = args.iter().zip(other_args).zip(decl.generics().iter());
            let mut args: Vec<Ty> = vec![];
            for ((first, second), def) in iter {
                match def.variance {
                    Variance::Invariant => {
                        if first.equals(second) {
                            args.push(first.clone());
                        }
                    }
                    Variance::Covariant => args.push(first.get_shared_subtype(second, project)),
                    Variance::Contravariant => todo!(),
                };
            }
            if args.len() == decl.generics().len() {
                return Ty::Named {
                    name: name.clone(),
                    args,
                };
            }
        }
    }
    let impls = project.get_impls(name);
    let mut shared = vec![];
    for impl_ in impls.iter() {
        if let Some(ty) = impl_.map(
            &Ty::Named {
                name,
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
    Ty::Any
}
