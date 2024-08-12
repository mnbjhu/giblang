use std::collections::HashMap;

use super::Ty;

// TODO: This should use unique ids instead of the String names for generic type args
impl Ty {
    pub fn parameterize(&self, generics: &HashMap<String, Ty>) -> Ty {
        match self {
            Ty::Generic(arg) => generics.get(&arg.name.0).unwrap_or(self).clone(),
            Ty::Named { name, args } => Ty::Named {
                name: *name,
                args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
            },
            Ty::Tuple(tys) => Ty::Tuple(tys.iter().map(|ty| ty.parameterize(generics)).collect()),
            Ty::Sum(tys) => Ty::Sum(tys.iter().map(|ty| ty.parameterize(generics)).collect()),
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                let receiver = receiver
                    .as_ref()
                    .map(|r| Box::new(r.parameterize(generics)));
                Ty::Function {
                    receiver,
                    args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
                    ret: Box::new(ret.parameterize(generics)),
                }
            }
            Ty::Meta(_) => unimplemented!("Need to thing about this..."),

            _ => self.clone(),
        }
    }
}
