use std::collections::HashMap;

use crate::{
    project::{ImplData, Project},
    ty::{Generic, Ty},
};

impl ImplData {
    pub fn map(&self, ty: &Ty, project: &Project) -> Option<Ty> {
        let implied_generics = self.from.imply_generics(ty)?;
        if implied_generics.len() == self.generics.len()
            && self
                .generics
                .iter()
                .all(|arg| implied_generics.contains_key(&arg.name))
        {
            for generic in &self.generics {
                let implied = implied_generics.get(&generic.name).unwrap();
                if !implied.is_instance_of(&generic.super_, project) {
                    return None;
                }
            }
            Some(self.to.parameterize(&implied_generics))
        } else {
            None
        }
    }
}

impl Ty {
    pub fn imply_generics(&self, other: &Ty) -> Option<HashMap<String, Ty>> {
        match (self, other) {
            // TODO: Check use of variance/super
            (Ty::Generic(Generic { name, .. }), other) => {
                let mut res = HashMap::new();
                res.insert(name.to_string(), other.clone());
                return Some(res);
            }
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                if name == other_name && args.len() == other_args.len() {
                    let mut res = HashMap::new();
                    for (s, o) in args.iter().zip(other_args) {
                        res.extend(s.imply_generics(o)?)
                    }
                    return Some(res);
                } else {
                    return None;
                }
            }
            (Ty::Tuple(s), Ty::Tuple(other)) => {
                let mut res = HashMap::new();
                for (s, o) in s.iter().zip(other) {
                    res.extend(s.imply_generics(o)?);
                }
                return Some(res);
            }
            (Ty::Sum(s), Ty::Sum(other)) => {
                let mut res = HashMap::new();
                for (s, o) in s.iter().zip(other) {
                    res.extend(s.imply_generics(o)?);
                }
                return Some(res);
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
                let mut res = HashMap::new();
                match (receiver, other_receiver) {
                    (None, None) => {}
                    (Some(s), Some(other)) => res.extend(s.imply_generics(other)?),
                    _ => return None,
                }
                for (s, o) in args.iter().zip(other_args) {
                    res.extend(s.imply_generics(o)?);
                }
                res.extend(ret.imply_generics(other_ret)?);
            }
            _ => {
                if self.equals(other) {
                    return Some(HashMap::new());
                }
            }
        };
        None
    }

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
