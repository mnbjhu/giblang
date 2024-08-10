use crate::check::state::CheckState;

use super::Ty;

// impl Ty {
//
//     pub fn parameterize(&self, state: &CheckState) -> Ty {
//         match self {
//             Ty::Named { name, args } => Ty::Named {
//                 name: *name,
//                 args: args.iter().map(|ty| ty.parameterize(state)).collect(),
//             },
//             Ty::Tuple(tys) => Ty::Tuple(tys.iter().map(|ty| ty.parameterize(state)).collect()),
//             Ty::Sum(tys) => Ty::Sum(tys.iter().map(|ty| ty.parameterize(state)).collect()),
//             Ty::Function {
//                 receiver,
//                 args,
//                 ret,
//             } => {
//                 let receiver = receiver.as_ref().map(|r| Box::new(r.parameterize(state)));
//                 Ty::Function {
//                     receiver,
//                     args: args.iter().map(|ty| ty.parameterize(state)).collect(),
//                     ret: Box::new(ret.parameterize(state)),
//                 }
//             }
//             Ty::Meta(_) => unimplemented!("Need to thing about this..."),
//             _ => self.clone(),
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//
//     #[test]
//     fn parameterize() {}
// }
