use std::collections::HashMap;

use super::{Generic, Ty};

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
}
