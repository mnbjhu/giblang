use std::fmt::Display;

use crate::{parser::common::variance::Variance, project::Project};

pub mod combine;
pub mod disp;
pub mod eq;
pub mod expect;
pub mod generics;
pub mod is_instance;

#[derive(Clone, Debug, PartialEq)]
pub struct Generic {
    pub name: String,
    pub variance: Variance,
    pub super_: Box<Ty>,
}

impl Generic {
    pub fn get_name(&self, project: &Project) -> String {
        format!(
            "{}{}: {}",
            self.variance,
            self.name,
            self.super_.get_name(project)
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

#[derive(Clone, PartialEq, Debug)]
pub enum PrimTy {
    String,
    Bool,
    Float,
    Int,
    Char,
}

impl From<&PrimTy> for Ty {
    fn from(value: &PrimTy) -> Self {
        match value {
            PrimTy::String => Ty::Named {
                name: 1,
                args: vec![],
            },
            PrimTy::Int => Ty::Named {
                name: 2,
                args: vec![],
            },
            PrimTy::Bool => Ty::Named {
                name: 3,
                args: vec![],
            },
            PrimTy::Float => Ty::Named {
                name: 4,
                args: vec![],
            },
            PrimTy::Char => Ty::Named {
                name: 5,
                args: vec![],
            },
        }
    }
}

impl Display for PrimTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimTy::String => write!(f, "String"),
            PrimTy::Bool => write!(f, "Bool"),
            PrimTy::Float => write!(f, "Float"),
            PrimTy::Int => write!(f, "Int"),
            PrimTy::Char => write!(f, "Char"),
        }
    }
}

impl Ty {
    pub fn get_name(&self, project: &Project) -> String {
        match self {
            Ty::Any => "Any".to_string(),
            Ty::Unknown => "Unknown".to_string(),
            Ty::Named { name, args } => {
                let decl = project.get_decl(*name);
                let name = decl.name();
                if args.is_empty() {
                    name.to_string()
                } else {
                    let args = args
                        .iter()
                        .map(|arg| arg.get_name(project))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{}<{}>", name, args)
                }
            }
            Ty::Generic(g) => g.get_name(project),
            Ty::Meta(_) => todo!(),
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                let receiver = receiver
                    .as_ref()
                    .map(|r| r.get_name(project))
                    .unwrap_or("".to_string());
                let args = args
                    .iter()
                    .map(|arg| arg.get_name(project))
                    .collect::<Vec<_>>()
                    .join(", ");
                let ret = ret.get_name(project);
                format!("{}({}) -> {}", receiver, args, ret)
            }
            Ty::Tuple(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_name(project))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", tys)
            }
            Ty::Sum(tys) => {
                let tys = tys
                    .iter()
                    .map(|ty| ty.get_name(project))
                    .collect::<Vec<_>>()
                    .join(" + ");
                format!("({})", tys)
            }
        }
    }
}
