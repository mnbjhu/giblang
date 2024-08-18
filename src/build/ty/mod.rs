use crate::{check::state::CheckState, ty::Ty};

impl Ty {
    pub fn build(&self, state: &mut CheckState) -> String {
        match self {
            Ty::Named { name, args } => {
                match name {
                    1 => return "string".to_string(),
                    2 => return "int".to_string(),
                    3 => return "bool".to_string(),
                    4 => return "float".to_string(),
                    5 => return "char".to_string(),
                    _ => {}
                };
                let qualified = state.project.get_qualified_name(*name);
                let args = args
                    .iter()
                    .map(|arg| arg.build(state))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{qualified}[{args}]")
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
