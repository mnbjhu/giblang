use crate::{fs::export::Export, parser::common::variance::Variance};

pub mod combine;
pub mod disp;
pub mod eq;
pub mod is_instance;

#[derive(Clone, Debug)]
pub enum Ty<'module> {
    Any,
    Unknown,
    Named {
        name: Export<'module>,
        args: Vec<Ty<'module>>,
    },
    Generic {
        name: String,
        variance: Variance,
        super_: Box<Ty<'module>>,
    },
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
