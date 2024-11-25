use crate::check::{is_scoped::IsScoped, scoped_state::Scoped};

use super::{FuncTy, Generic, Named, Ty};

impl<'db> Ty<'db> {
    pub fn get_name(&self, state: &impl IsScoped<'db>) -> String {
        match self {
            Ty::Any => "Any".to_string(),
            Ty::Nothing => "Nothing".to_string(),
            Ty::Unknown => "Unknown".to_string(),
            Ty::Named(Named { name, args }) => {
                let decl = state.try_get_decl_path(*name);
                // TODO: check unwrap
                if decl.is_none() {
                    return format!("{{err:{}}}", name.name(state.db()).join("::"));
                }
                let name = decl.unwrap().name(state.db());
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
            Ty::Meta(ty) => format!("Meta({})", ty.get_name(state)),
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
                state.get_type_var(*id).get_name(state);
                format!("{{unknown:{id}}}")
            }
        }
    }

    pub fn unit() -> Self {
        Ty::Tuple(Vec::new())
    }
}

impl<'db> Generic<'db> {
    pub fn get_name(&self, state: &impl IsScoped<'db>) -> String {
        if let Ty::Any = self.super_.as_ref() {
            format!("{}{}", self.variance, self.name.0)
        } else {
            format!(
                "{}{}: {}",
                self.variance,
                self.name.0,
                self.super_.get_name(state)
            )
        }
    }
}

impl<'db> FuncTy<'db> {
    pub fn get_name(&self, state: &impl IsScoped<'db>) -> String {
        let args = self
            .args
            .iter()
            .map(|arg| arg.get_name(state))
            .collect::<Vec<_>>()
            .join(", ");
        let ret = self.ret.get_name(state);
        if let Some(receiver) = &self.receiver {
            let receiver = receiver.get_name(state);
            format!("{receiver}.({args}) -> {ret}")
        } else {
            format!("({args}) -> {ret}")
        }
    }
}
