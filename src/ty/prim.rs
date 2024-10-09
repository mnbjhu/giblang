use std::fmt::Display;

use salsa::Database;

use crate::db::modules::ModulePath;

use super::Ty;

#[derive(Clone, PartialEq, Debug)]
pub enum PrimTy {
    String,
    Bool,
    Float,
    Int,
    Char,
}

impl<'db> Ty<'db> {
    pub fn string(db: &'db dyn Database) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["String".to_string()]),
            args: Vec::new(),
        }
    }

    pub fn int(db: &'db dyn Database) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["Int".to_string()]),
            args: Vec::new(),
        }
    }

    pub fn bool(db: &'db dyn Database) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["Bool".to_string()]),
            args: Vec::new(),
        }
    }

    pub fn float(db: &'db dyn Database) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["Float".to_string()]),
            args: Vec::new(),
        }
    }

    pub fn char(db: &'db dyn Database) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["Char".to_string()]),
            args: Vec::new(),
        }
    }
}

impl<'db> Ty<'db> {
    pub fn from_prim(prim: PrimTy, db: &'db dyn Database) -> Self {
        match prim {
            PrimTy::String => Ty::string(db),
            PrimTy::Bool => Ty::bool(db),
            PrimTy::Float => Ty::float(db),
            PrimTy::Int => Ty::int(db),
            PrimTy::Char => Ty::char(db),
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

    // #[test]
    // fn from_prim() {
    //     assert_eq!(Ty::from(&PrimTy::String), Ty::string());
    //     assert_eq!(Ty::from(&PrimTy::Bool), Ty::bool());
    //     assert_eq!(Ty::from(&PrimTy::Float), Ty::float());
    //     assert_eq!(Ty::from(&PrimTy::Int), Ty::int());
    //     assert_eq!(Ty::from(&PrimTy::Char), Ty::char());
    // }
}
