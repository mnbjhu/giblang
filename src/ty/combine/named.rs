// pub fn get_shared_named_subtype(other: &Ty, name: u32, args: &[Ty], state: &mut CheckState) -> Ty {
//     if let Ty::Named {
//         name: other_name,
//         args: other_args,
//     } = &other
//     {
//         let decl = state.project.get_decl(name);
//         if name == *other_name && args.len() == other_args.len() {
//             let generics = decl.generics();
//             let iter = args.iter().zip(other_args).zip(&generics);
//             let mut args: Vec<Ty> = vec![];
//             for ((first, second), def) in iter {
//                 match def.variance {
//                     Variance::Invariant => {
//                         if first.equals(second) {
//                             args.push(first.clone());
//                         }
//                     }
//                     Variance::Covariant => args.push(first.get_shared_subtype(second, state)),
//                     Variance::Contravariant => todo!(),
//                 };
//             }
//             if args.len() == generics.len() {
//                 return Ty::Named { name, args };
//             }
//         }
//     }
//     let impls = state.project.get_impls(name);
//     let mut shared = vec![];
//     for impl_ in &impls {
//         if let Some(ty) = impl_.map(
//             &Ty::Named {
//                 name,
//                 args: args.to_vec(),
//             },
//             state,
//         ) {
//             let found = ty.get_shared_subtype(other, state);
//             if let Ty::Sum(v) = found {
//                 shared.extend(v);
//             } else if let Ty::Any = found {
//             } else {
//                 shared.push(found);
//             }
//         }
//     }
//     if shared.is_empty() {
//         Ty::Any
//     } else if shared.len() == 1 {
//         shared[0].clone()
//     } else {
//         Ty::Sum(shared)
//     }
// }
