use std::default;

use crate::{check::state::CheckState, parser::common::variance::Variance, util::Spanned};

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
    pub name: Spanned<String>,
    pub variance: Variance,
    pub super_: Box<Ty>,
}

impl Generic {
    pub fn new(name: Spanned<String>) -> Generic {
        Generic {
            name,
            variance: Variance::Invariant,
            super_: Box::new(Ty::Any),
        }
    }

    pub fn get_name(&self, state: &CheckState) -> String {
        format!(
            "{}{}: {}",
            self.variance,
            self.name.0,
            self.super_.get_name(state)
        )
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Ty {
    Any,
    #[default]
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
                let var = state.get_resolved_type_var(*id);
                var.get_name(state)
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
            Ty::Function { .. } => "Function".to_string(),
            Ty::Tuple(_) => "Tuple".to_string(),
            Ty::Sum(_) => "Sum".to_string(),
        }
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
