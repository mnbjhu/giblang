use std::fmt::Display;

use super::Ty;

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

#[cfg(test)]
mod tests {
    use crate::ty::{prim::PrimTy, Ty};

    #[test]
    fn from_prim() {
        assert_eq!(
            Ty::from(&PrimTy::String),
            Ty::Named {
                name: 1,
                args: vec![]
            }
        );

        assert_eq!(
            Ty::from(&PrimTy::Int),
            Ty::Named {
                name: 2,
                args: vec![]
            }
        );

        assert_eq!(
            Ty::from(&PrimTy::Bool),
            Ty::Named {
                name: 3,
                args: vec![]
            }
        );

        assert_eq!(
            Ty::from(&PrimTy::Float),
            Ty::Named {
                name: 4,
                args: vec![]
            }
        );

        assert_eq!(
            Ty::from(&PrimTy::Char),
            Ty::Named {
                name: 5,
                args: vec![]
            }
        );
    }
}
