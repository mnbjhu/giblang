use crate::parser::common::variance::Variance;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Any,
    Unknown,
    Named {
        name: u32,
        args: Vec<Ty>,
    },
    Generic(Generic),
    Prim(PrimTy),
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
