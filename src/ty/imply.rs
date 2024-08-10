use crate::check::state::CheckState;

use super::Ty;

impl Ty {
    pub fn imply_type_vars(&self, other: &Ty, state: &mut CheckState) {
        match (self, other) {
            (Ty::TypeVar { id }, other) => {
                if let Ty::TypeVar { id: other_id } = other {
                    if id == other_id {
                        return;
                    }
                }
                state.add_type_bound(*id, other.clone());
            }
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                if name == other_name && args.len() == other_args.len() {
                    for (s, o) in args.iter().zip(other_args) {
                        s.imply_type_vars(o, state);
                    }
                }
            }
            (Ty::Sum(s), Ty::Sum(other)) | (Ty::Tuple(s), Ty::Tuple(other)) => {
                for (s, o) in s.iter().zip(other) {
                    s.imply_type_vars(o, state);
                }
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
                if let (Some(s), Some(other)) = (receiver, other_receiver) {
                    s.imply_type_vars(other, state);
                }
                for (s, o) in args.iter().zip(other_args) {
                    s.imply_type_vars(o, state);
                }
                ret.imply_type_vars(other_ret, state);
            }
            _ => {}
        }
    }
}
