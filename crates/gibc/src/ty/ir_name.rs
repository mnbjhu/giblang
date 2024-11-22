use crate::ir::IrState;

use super::{FuncTy, Generic, Named, Ty};

impl<'db> Ty<'db> {
    pub fn get_ir_name(&self, state: &IrState<'db>) -> String {
        match self {
            Ty::Any => "Any".to_string(),
            Ty::Nothing => "Nothing".to_string(),
            Ty::Unknown => "Unknown".to_string(),
            Ty::Named(Named { name, args }) => {
                let decl = state.try_get_decl_path(*name);
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
                        .map(|arg| arg.get_ir_name(state))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{name}[{args}]")
                }
            }
            Ty::Generic(g) => g.get_ir_name(state),
            Ty::Meta(ty) => format!("Meta({})", ty.get_ir_name(state)),
            Ty::Function(func) => func.get_ir_name(state),
            Ty::Tuple(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_ir_name(state))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({tys})")
            }
            Ty::Sum(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_ir_name(state))
                    .collect::<Vec<_>>()
                    .join(" + ");
                format!("({tys})")
            }
            Ty::TypeVar { id } => {
                if let Some(ty) = state.type_vars.get(id) {
                    return ty.get_ir_name(state);
                }
                format!("{{unknown:{id}}}")
            }
        }
    }
}

impl<'db> Generic<'db> {
    pub fn get_ir_name(&self, state: &IrState<'db>) -> String {
        if let Ty::Any = self.super_.as_ref() {
            format!("{}{}", self.variance, self.name.0)
        } else {
            format!(
                "{}{}: {}",
                self.variance,
                self.name.0,
                self.super_.get_ir_name(state)
            )
        }
    }
}

impl<'db> FuncTy<'db> {
    pub fn get_ir_name(&self, state: &IrState<'db>) -> String {
        let args = self
            .args
            .iter()
            .map(|arg| arg.get_ir_name(state))
            .collect::<Vec<_>>()
            .join(", ");
        let ret = self.ret.get_ir_name(state);
        if let Some(receiver) = &self.receiver {
            let receiver = receiver.get_ir_name(state);
            format!("{receiver}.({args}) -> {ret}")
        } else {
            format!("({args}) -> {ret}")
        }
    }
}
