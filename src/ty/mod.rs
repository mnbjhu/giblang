use crate::{fs::export::Export, parser::common::variance::Variance};

pub mod combine;
pub mod disp;
pub mod eq;
pub mod expect;
pub mod generics;
pub mod is_instance;

#[derive(Clone, Debug)]
pub struct Generic<'module> {
    pub name: String,
    pub variance: Variance,
    pub super_: Box<Ty<'module>>,
}

#[derive(Clone, Debug)]
pub enum Ty<'module> {
    Any,
    Unknown,
    Named {
        name: Export<'module>,
        args: Vec<Ty<'module>>,
    },
    Generic(Generic<'module>),
    Prim(PrimTy),
    Meta(Box<Ty<'module>>),
    Function {
        receiver: Option<Box<Ty<'module>>>,
        args: Vec<Ty<'module>>,
        ret: Box<Ty<'module>>,
    },
    Tuple(Vec<Ty<'module>>),
    Sum(Vec<Ty<'module>>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum PrimTy {
    String,
    Bool,
    Float,
    Int,
    Char,
}
