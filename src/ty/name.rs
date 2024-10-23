use std::collections::HashMap;

use crate::check::state::CheckState;

use super::{FuncTy, Generic, Ty};

impl<'db> Ty<'db> {
    pub fn get_name(
        &self,
        state: &CheckState<'db>,
        type_vars: Option<&HashMap<u32, Ty<'db>>>,
    ) -> String {
        match self {
            Ty::Any => "Any".to_string(),
            Ty::Nothing => "Nothing".to_string(),
            Ty::Unknown => "Unknown".to_string(),
            Ty::Named { name, args } => {
                let decl = state.try_get_decl(*name);
                // TODO: check unwrap
                if decl.is_none() {
                    return format!("{{err:{}}}", name.name(state.db).join("::"));
                }
                let name = decl.unwrap().name(state.db);
                if args.is_empty() {
                    name.to_string()
                } else {
                    let args = args
                        .iter()
                        .map(|arg| arg.get_name(state, type_vars))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{name}[{args}]")
                }
            }
            Ty::Generic(g) => g.get_name(state, type_vars),
            Ty::Meta(ty) => format!("Meta({})", ty.get_name(state, type_vars)),
            Ty::Function(func) => func.get_name(state, type_vars),
            Ty::Tuple(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_name(state, type_vars))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({tys})")
            }
            Ty::Sum(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_name(state, type_vars))
                    .collect::<Vec<_>>()
                    .join(" + ");
                format!("({tys})")
            }
            Ty::TypeVar { id } => {
                if let Some(resolved) = state.try_get_resolved_type_var(*id) {
                    return resolved.get_name(state, type_vars);
                }
                if let Some(tv) = type_vars {
                    if let Some(ty) = tv.get(id) {
                        return ty.get_name(state, type_vars)
                    }
                }
                format!("{{unknown:{id}}}")
            }
        }
    }

    pub fn unit() -> Self {
        Ty::Tuple(Vec::new())
    }
}

impl<'db> Generic<'db> {
    pub fn get_name(&self, state: &CheckState, type_vars: Option<&HashMap<u32, Ty<'db>>>,) -> String {
        if let Ty::Any = self.super_.as_ref() {
            format!("{}{}", self.variance, self.name.0)
        } else {
            format!(
                "{}{}: {}",
                self.variance,
                self.name.0,
                self.super_.get_name(state, type_vars)
            )
        }
    }
}

impl<'db> FuncTy<'db> {
    pub fn get_name(&self, state: &CheckState, type_vars: Option<&HashMap<u32, Ty<'db>>>,) -> String {
        let args = self
            .args
            .iter()
            .map(|arg| arg.get_name(state, type_vars))
            .collect::<Vec<_>>()
            .join(", ");
        let ret = self.ret.get_name(state, type_vars);
        if let Some(receiver) = &self.receiver {
            let receiver = receiver.get_name(state, type_vars);
            format!("{receiver}.({args}) -> {ret}")
        } else {
            format!("({args}) -> {ret}")
        }
    }
}

