use std::collections::HashMap;

use crate::check::state::CheckState;

use super::{FuncTy, Ty};

impl<'db> Ty<'db> {
    pub fn get_name(&self, state: &CheckState) -> String {
        match self {
            Ty::Any => "Any".to_string(),
            Ty::Unknown => "Unknown".to_string(),
            Ty::Named { name, args } => {
                let decl = state.project.get_decl(state.db, *name);
                // TODO: check unwrap
                if decl.is_none() {
                    return "{err}".to_string();
                }
                let name = decl.unwrap().name(state.db);
                if args.is_empty() {
                    name.to_string()
                } else {
                    let args = args
                        .iter()
                        .map(|arg| arg.get_name(state))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{name}[{args}]")
                }
            }
            Ty::Generic(g) => g.get_name(state),
            Ty::Meta(_) => todo!(),
            Ty::Function(func) => func.get_name(state),
            Ty::Tuple(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_name(state))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({tys})")
            }
            Ty::Sum(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_name(state))
                    .collect::<Vec<_>>()
                    .join(" + ");
                format!("({tys})")
            }
            Ty::TypeVar { id } => {
                let var = state.try_get_resolved_type_var(*id);
                if let Some(var) = var {
                    var.get_name(state)
                } else {
                    "{unknown}".to_string()
                }
            }
        }
    }

    pub fn get_name_with_types(
        &self,
        state: &mut CheckState<'_, 'db>,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> String {
        match self {
            Ty::Any => "Any".to_string(),
            Ty::Unknown => "Unknown".to_string(),
            Ty::Named { name, args } => {
                let decl = state.project.get_decl(state.db, *name);
                // TODO: check unwrap
                if decl.is_none() {
                    return "{err}".to_string();
                }
                let name = decl.unwrap().name(state.db);
                if args.is_empty() {
                    name.to_string()
                } else {
                    let args = args
                        .iter()
                        .map(|arg| arg.get_name(state))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{name}[{args}]")
                }
            }
            Ty::Generic(g) => g.get_name(state),
            Ty::Meta(_) => todo!(),
            Ty::Function(func) => func.get_name(state),
            Ty::Tuple(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_name(state))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({tys})")
            }
            Ty::Sum(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_name(state))
                    .collect::<Vec<_>>()
                    .join(" + ");
                format!("({tys})")
            }
            Ty::TypeVar { id } => {
                if let Some(ty) = type_vars.get(id) {
                    ty.get_name(state)
                } else {
                    "{unknown}".to_string()
                }
            }
        }
    }

    pub fn unit() -> Self {
        Ty::Tuple(Vec::new())
    }

    pub fn kind(&self) -> String {
        match self {
            Ty::Any => "Any".to_string(),
            Ty::Unknown => "Unknown".to_string(),
            Ty::Named { .. } => "Named".to_string(),
            Ty::TypeVar { .. } => "TypeVar".to_string(),
            Ty::Generic(_) => "Generic".to_string(),
            Ty::Meta(_) => "Meta".to_string(),
            Ty::Function(FuncTy { .. }) => "Function".to_string(),
            Ty::Tuple(_) => "Tuple".to_string(),
            Ty::Sum(_) => "Sum".to_string(),
        }
    }
}

impl<'db> FuncTy<'db> {
    pub fn get_name(&self, state: &CheckState) -> String {
        let receiver = self
            .receiver
            .as_ref()
            .map_or(String::new(), |r| r.get_name(state));
        let args = self
            .args
            .iter()
            .map(|arg| arg.get_name(state))
            .collect::<Vec<_>>()
            .join(", ");
        let ret = self.ret.get_name(state);
        format!("{receiver}({args}) -> {ret}")
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::check::ty::tests::parse_ty_with_state;
//     use crate::project::{check_test_state, Project};
//     use crate::ty::{FuncTy, Generic, Ty};
//     use crate::util::Span;
//
//     #[test]
//     fn named() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_ty_with_state(&mut state, "Int");
//         let name = ty.get_name(&state);
//         assert_eq!(name, "Int");
//     }
//
//     #[test]
//     fn named_args() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_ty_with_state(&mut state, "Result[Int, String]");
//         let name = ty.get_name(&state);
//         assert_eq!(name, "Result[Int, String]");
//     }
//
//     #[test]
//     fn any() {
//         let project = Project::check_test();
//         let state = check_test_state(&project);
//         assert_eq!(Ty::Any.get_name(&state), "Any");
//     }
//
//     #[test]
//     fn unknown() {
//         let project = Project::check_test();
//         let state = check_test_state(&project);
//         assert_eq!(Ty::Any.get_name(&state), "Any");
//     }
//
//     #[test]
//     fn unit() {
//         let project = Project::check_test();
//         let state = check_test_state(&project);
//         assert_eq!(Ty::unit().get_name(&state), "()");
//     }
//
//     #[test]
//     fn tuple() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_ty_with_state(&mut state, "(Int, String)");
//         let name = ty.get_name(&state);
//         assert_eq!(name, "(Int, String)");
//     }
//
//     #[test]
//     fn sum() {
//         let project = Project::check_test();
//         let mut state = check_test_state(&project);
//         let ty = parse_ty_with_state(&mut state, "Int + String");
//         let name = ty.get_name(&state);
//         assert_eq!(name, "(Int + String)");
//     }
//
//     #[test]
//     fn kind() {
//         assert_eq!(Ty::Any.kind(), "Any");
//         assert_eq!(Ty::Unknown.kind(), "Unknown");
//         assert_eq!(
//             Ty::Named {
//                 name: 0,
//                 args: vec![]
//             }
//             .kind(),
//             "Named"
//         );
//         assert_eq!(Ty::TypeVar { id: 0 }.kind(), "TypeVar");
//         assert_eq!(
//             Ty::Generic(Generic::new(("T".to_string(), Span::splat(0)))).kind(),
//             "Generic"
//         );
//         assert_eq!(
//             Ty::Function(FuncTy {
//                 receiver: None,
//                 args: vec![],
//                 ret: Box::new(Ty::Any)
//             })
//             .kind(),
//             "Function"
//         );
//         assert_eq!(Ty::Tuple(vec![]).kind(), "Tuple");
//         assert_eq!(Ty::Sum(vec![]).kind(), "Sum");
//         assert_eq!(Ty::Meta(Box::new(Ty::Any)).kind(), "Meta");
//     }
// }
