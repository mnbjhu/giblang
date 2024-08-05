use std::collections::HashMap;

use super::{Generic, Ty};

impl Ty {
    pub fn parameterize(&self, generics: &HashMap<String, Ty>) -> Ty {
        match self {
            Ty::Any => Ty::Any,
            Ty::Unknown => Ty::Unknown,
            Ty::Named { name, args } => Ty::Named {
                name: *name,
                args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
            },
            // TODO: Check use of variance/super
            Ty::Generic(Generic { name, .. }) => {
                if let Some(ty) = generics.get(name) {
                    ty.clone()
                } else {
                    self.clone()
                }
            }
            Ty::Tuple(tys) => Ty::Tuple(tys.iter().map(|ty| ty.parameterize(generics)).collect()),
            Ty::Sum(tys) => Ty::Sum(tys.iter().map(|ty| ty.parameterize(generics)).collect()),
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                if let Some(receiver) = receiver {
                    Ty::Function {
                        receiver: Some(Box::new(receiver.parameterize(generics))),
                        args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
                        ret: Box::new(ret.parameterize(generics)),
                    }
                } else {
                    Ty::Function {
                        receiver: None,
                        args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
                        ret: Box::new(ret.parameterize(generics)),
                    }
                }
            }
            Ty::Meta(_) => unimplemented!("Need to thing about this..."),
        }
    }
}
