use std::fmt::Display;

use super::{Generic, PrimTy, Ty};

impl Display for Ty<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::Any => write!(f, "Any"),
            Ty::Unknown => write!(f, "Unknown"),
            Ty::Named { name, args } => {
                let name = name.name();
                write!(f, "{}", name)?;
                if !args.is_empty() {
                    write!(f, "[")?;
                    let txt = args
                        .iter()
                        .map(|ty| ty.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    write!(f, "{}", txt)?;
                    write!(f, "]")?;
                }
                Ok(())
            }
            Ty::Generic(Generic {
                name,
                variance,
                super_,
            }) => {
                write!(f, "{variance}{name}: {super_}")
            }
            Ty::Prim(p) => match p {
                PrimTy::String => write!(f, "String"),
                PrimTy::Bool => write!(f, "Bool"),
                PrimTy::Float => write!(f, "Float"),
                PrimTy::Int => write!(f, "Int"),
                PrimTy::Char => write!(f, "Char"),
            },
            Ty::Meta(ty) => write!(f, "Type[{}]", ty),
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                let args = args
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                if let Some(receiver) = receiver {
                    write!(f, "{}.({}) -> {}", receiver, args, ret)
                } else {
                    write!(f, "({}) -> {}", args, ret)
                }
            }
            Ty::Tuple(tys) => {
                let txt = tys
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "({})", txt)
            }
            Ty::Sum(tys) => {
                let txt = tys
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(" + ");
                write!(f, "{}", txt)
            }
        }
    }
}
