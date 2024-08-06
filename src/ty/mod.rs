use crate::{parser::common::variance::Variance, project::Project};

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
            name: "".to_string(),
            variance: Variance::Invariant,
            super_: Box::new(Ty::Any),
        }
    }
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
