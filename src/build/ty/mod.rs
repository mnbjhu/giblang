use crate::{check::state::CheckState, ty::Ty};

impl Ty {
    pub fn build(&self, state: &mut CheckState) -> String {
        match self {
            Ty::Named { name, args } => {
                let name = type_name(*name);
                let args = if args.is_empty() {
                    String::new()
                } else {
                    let inner = args
                        .iter()
                        .map(|arg| arg.build(state))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("[{inner}]")
                };
                format!("{name}{args}")
            }
            Ty::Tuple(args) => {
                let args = args
                    .iter()
                    .map(|arg| arg.build(state))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({args})")
            }
            Ty::Any => "any".to_string(),
            Ty::Unknown => panic!("Unknown types should not be built"),
            Ty::TypeVar { .. } => todo!(),
            Ty::Generic(g) => g.name.0.to_string(),
            Ty::Meta(_) => todo!(),
            Ty::Function(_) => todo!(),
            Ty::Sum(_) => todo!(),
        }
    }
}
#[must_use]
pub fn type_name(name: u32) -> String {
    match name {
        1 => "string".to_string(),
        2 => "int".to_string(),
        3 => "bool".to_string(),
        4 => "float".to_string(),
        5 => "char".to_string(),
        _ => format!("T{name}"),
    }
}
