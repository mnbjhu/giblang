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

impl From<&PrimTy> for Ty {
    fn from(value: &PrimTy) -> Self {
        match value {
            PrimTy::String => Ty::string(),
            PrimTy::Bool => Ty::bool(),
            PrimTy::Float => Ty::float(),
            PrimTy::Int => Ty::int(),
            PrimTy::Char => Ty::char(),
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
    fn test_display() {
        assert_eq!(PrimTy::String.to_string(), "String");
        assert_eq!(PrimTy::Bool.to_string(), "Bool");
        assert_eq!(PrimTy::Float.to_string(), "Float");
        assert_eq!(PrimTy::Int.to_string(), "Int");
        assert_eq!(PrimTy::Char.to_string(), "Char");
    }

    #[test]
    fn from_prim() {
        assert_eq!(Ty::from(&PrimTy::String), Ty::string());
        assert_eq!(Ty::from(&PrimTy::Bool), Ty::bool());
        assert_eq!(Ty::from(&PrimTy::Float), Ty::float());
        assert_eq!(Ty::from(&PrimTy::Int), Ty::int());
        assert_eq!(Ty::from(&PrimTy::Char), Ty::char());
    }
}
