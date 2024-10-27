use std::collections::HashMap;

use super::{FuncTy, Generic, Ty};

// TODO: This should use unique ids instead of the String names for generic type args
impl<'db> Ty<'db> {
    pub fn parameterize(&self, generics: &HashMap<String, Ty<'db>>) -> Ty<'db> {
        match self {
            Ty::Generic(arg) => generics
                .get(&arg.name.0)
                .cloned()
                .unwrap_or(arg.super_.parameterize(generics)),
            Ty::Named { name, args } => Ty::Named {
                name: *name,
                args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
            },
            Ty::Tuple(tys) => Ty::Tuple(tys.iter().map(|ty| ty.parameterize(generics)).collect()),
            Ty::Sum(tys) => Ty::Sum(tys.iter().map(|ty| ty.parameterize(generics)).collect()),
            Ty::Function(func) => Ty::Function(func.parameterize(generics)),
            Ty::Meta(_) => unimplemented!("Need to thing about this..."),
            _ => self.clone(),
        }
    }
}

impl<'db> FuncTy<'db> {
    pub fn parameterize(&self, generics: &HashMap<String, Ty<'db>>) -> FuncTy<'db> {
        FuncTy {
            receiver: self.receiver.as_ref().map(|r| Box::new(r.parameterize(generics))),
            args: self.args.iter().map(|ty| ty.parameterize(generics)).collect(),
            ret: Box::new(self.ret.parameterize(generics)),
        }
    }
}

impl<'db> Generic<'db> {
    pub fn parameterize(&self, params: &HashMap<String, Ty<'db>>) -> Generic<'db> {
        Generic {
            name: self.name.clone(),
            variance: self.variance,
            super_: Box::new(self.super_.parameterize(params)),
        }
    }
}
