use std::fmt::Display;

use super::{Generic, Ty};

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::Any => write!(f, "Any"),
            Ty::Unknown => write!(f, "Unknown"),
            Ty::Named { name, args } => {
                write!(f, "{name}")?;
                if !args.is_empty() {
                    write!(f, "[")?;
                    let txt = args
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ");
                    write!(f, "{txt}")?;
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
            Ty::Meta(ty) => write!(f, "Type[{ty}]"),
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                let args = args
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                if let Some(receiver) = receiver {
                    write!(f, "{receiver}.({args}) -> {ret}")
                } else {
                    write!(f, "({args}) -> {ret}")
                }
            }
            Ty::Tuple(tys) => {
                let txt = tys
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "({txt})")
            }
            Ty::Sum(tys) => {
                let txt = tys
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" + ");
                write!(f, "{txt}")
            }
            Ty::TypeVar { .. } => "{{unknown}}".fmt(f),
        }
    }
}
