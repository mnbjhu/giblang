use super::Ty;

pub mod named;

impl Ty {
    // pub fn get_shared_subtype(&self, other: &Ty, state: &mut CheckState) -> Ty {
    //     if self.is_instance_of(other, state) {
    //         return other.clone();
    //     } else if other.is_instance_of(self, state) {
    //         return self.clone();
    //     }
    //     match (self, other) {
    //         (_, Ty::Any) | (Ty::Any, _) => Ty::Any,
    //         (Ty::Unknown, other) => other.clone(),
    //         (s, Ty::Unknown) => s.clone(),
    //         (Ty::Tuple(v), ty) | (ty, Ty::Tuple(v)) => {
    //             if let Ty::Tuple(other) = ty {
    //                 if v.len() == other.len() {
    //                     return Ty::Tuple(
    //                         v.iter()
    //                             .zip(other)
    //                             .map(|(s, o)| s.clone().get_shared_subtype(o, state))
    //                             .collect(),
    //                     );
    //                 }
    //             }
    //             Ty::Any
    //         }
    //         // TODO: Think about usecases for this
    //         (Ty::Meta(_), _) | (_, Ty::Meta(_)) => Ty::Any,
    //         (
    //             Ty::Named { name, args },
    //             Ty::Named {
    //                 name: other_name,
    //                 args: other_args,
    //             },
    //         ) => {
    //             fn insert_ty(ty: Ty, new: &mut Vec<Ty>) {
    //                 if !new.iter().any(|t| t.equals(&ty)) {
    //                     new.push(ty);
    //                 }
    //             }
    //             let mut new = vec![];
    //             match get_shared_named_subtype(other, *name, args, state) {
    //                 Ty::Any => {}
    //                 Ty::Sum(v) => {
    //                     for ty in v {
    //                         insert_ty(ty, &mut new);
    //                     }
    //                 }
    //                 ty => insert_ty(ty, &mut new),
    //             }
    //             match get_shared_named_subtype(self, *other_name, other_args, state) {
    //                 Ty::Any => {}
    //                 Ty::Sum(v) => {
    //                     for ty in v {
    //                         insert_ty(ty, &mut new);
    //                     }
    //                 }
    //                 ty => insert_ty(ty, &mut new),
    //             }
    //             match new.len() {
    //                 0 => Ty::Any,
    //                 1 => new[0].clone(),
    //                 _ => Ty::Sum(new),
    //             }
    //         }
    //         (Ty::Named { name, args }, other) | (other, Ty::Named { name, args }) => {
    //             get_shared_named_subtype(other, *name, args, state)
    //         }
    //         _ => todo!(),
    //     }
    // }
}
