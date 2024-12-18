use std::collections::HashMap;

use super::{FuncTy, Named, Ty};

// TODO: Makes assumptions about correct generic args
impl<'db> Ty<'db> {
    pub fn imply_generic_args(self, other: &Ty<'db>, implied: &mut HashMap<String, Ty<'db>>) {
        match (&self, &other.clone()) {
            (Ty::Generic(g), _) => {
                implied.insert(g.name.0.to_string(), other.clone());
            }
            (
                Ty::Named(Named { name, args }),
                Ty::Named(Named {
                    name: other_name,
                    args: other_args,
                }),
            ) => {
                if name == other_name && args.len() == other_args.len() {
                    for (s, o) in args.iter().zip(other_args) {
                        s.clone().imply_generic_args(o, implied);
                    }
                }
            }
            (Ty::Sum(s), Ty::Sum(other)) | (Ty::Tuple(s), Ty::Tuple(other)) => {
                for (s, o) in s.iter().zip(other) {
                    s.clone().imply_generic_args(o, implied);
                }
            }
            (
                Ty::Function(FuncTy {
                    receiver,
                    args,
                    ret,
                }),
                Ty::Function(FuncTy {
                    receiver: other_receiver,
                    args: other_args,
                    ret: other_ret,
                }),
            ) => {
                if let (Some(s), Some(other)) = (receiver, other_receiver) {
                    s.clone().imply_generic_args(other, implied);
                }
                for (s, o) in args.iter().zip(other_args) {
                    s.clone().imply_generic_args(o, implied);
                }
                ret.clone().imply_generic_args(other_ret.as_ref(), implied);
            }
            _ => {}
        }
    }
}
