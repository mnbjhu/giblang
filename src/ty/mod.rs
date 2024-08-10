use crate::{check::state::CheckState, parser::common::variance::Variance};

pub mod combine;
pub mod disp;
pub mod eq;
pub mod generics;
pub mod imply;
pub mod is_instance;
pub mod parameterize;
pub mod prim;

#[derive(Clone, Debug, PartialEq)]
pub struct Generic {
    pub name: String,
    pub variance: Variance,
    pub super_: Box<Ty>,
}

impl Default for Generic {
    fn default() -> Self {
        Self {
            name: String::new(),
            variance: Variance::Invariant,
            super_: Box::new(Ty::Any),
        }
    }
}

impl Generic {
    pub fn get_name(&self, state: &CheckState) -> String {
        format!(
            "{}{}: {}",
            self.variance,
            self.name,
            self.super_.get_name(state)
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Any,
    Unknown,
    Named {
        name: u32,
        args: Vec<Ty>,
    },
    TypeVar {
        id: u32,
    },
    Generic(Generic),
    Meta(Box<Ty>),
    Function {
        receiver: Option<Box<Ty>>,
        args: Vec<Ty>,
        ret: Box<Ty>,
    },
    Tuple(Vec<Ty>),
    Sum(Vec<Ty>),
}

impl Ty {
    pub fn get_name(&self, state: &CheckState) -> String {
        match self {
            Ty::Any => "Any".to_string(),
            Ty::Unknown => "Unknown".to_string(),
            Ty::Named { name, args } => {
                let decl = state.project.get_decl(*name);
                let name = decl.name();
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
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                let receiver = receiver
                    .as_ref()
                    .map_or(String::new(), |r| r.get_name(state));
                let args = args
                    .iter()
                    .map(|arg| arg.get_name(state))
                    .collect::<Vec<_>>()
                    .join(", ");
                let ret = ret.get_name(state);
                format!("{receiver}({args}) -> {ret}")
            }
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
                let var = state.get_type_var(*id);
                if let Some(var) = var {
                    if let Some(ty) = &var.ty {
                        return ty.get_name(state);
                    }
                }
                format!("unknown")
            }
        }
    }

    pub fn unit() -> Self {
        Ty::Tuple(Vec::new())
    }
}

impl Ty {
    pub fn string() -> Self {
        Ty::Named {
            name: 1,
            args: Vec::new(),
        }
    }

    pub fn int() -> Self {
        Ty::Named {
            name: 2,
            args: Vec::new(),
        }
    }

    pub fn bool() -> Self {
        Ty::Named {
            name: 3,
            args: Vec::new(),
        }
    }

    pub fn float() -> Self {
        Ty::Named {
            name: 4,
            args: Vec::new(),
        }
    }

    pub fn char() -> Self {
        Ty::Named {
            name: 5,
            args: Vec::new(),
        }
    }
}
