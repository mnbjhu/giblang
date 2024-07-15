use crate::{
    check::ty::{PrimTy, Ty},
    lexer::literal::Literal,
};

impl<'module> From<&Literal> for Ty<'module> {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Int(_) => Ty::Prim(PrimTy::Int),
            Literal::Float(_) => Ty::Prim(PrimTy::Float),
            Literal::String(_) => Ty::Prim(PrimTy::String),
            Literal::Bool(_) => Ty::Prim(PrimTy::Bool),
            Literal::Char(_) => Ty::Prim(PrimTy::Char),
        }
    }
}
