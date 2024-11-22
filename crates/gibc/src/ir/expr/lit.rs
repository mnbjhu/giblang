use gvm::format::literal::Literal;

use crate::{
    db::{input::Db, path::ModulePath},
    ty::{Named, Ty},
};

pub trait Typed {
    fn to_ty<'db>(&self, db: &'db dyn Db) -> Ty<'db>;
}

impl Typed for Literal {
    fn to_ty<'db>(&self, db: &'db dyn Db) -> Ty<'db> {
        match self {
            Literal::String(_) => Ty::Named(Named {
                name: ModulePath::new(db, vec!["std".to_string(), "String".to_string()]),
                args: vec![],
            }),
            Literal::Int(_) => Ty::Named(Named {
                name: ModulePath::new(db, vec!["std".to_string(), "Int".to_string()]),
                args: vec![],
            }),
            Literal::Bool(_) => Ty::bool(db),
            Literal::Float(_) => Ty::Named(Named {
                name: ModulePath::new(db, vec!["std".to_string(), "Float".to_string()]),
                args: vec![],
            }),
            Literal::Char(_) => Ty::Named(Named {
                name: ModulePath::new(db, vec!["std".to_string(), "Char".to_string()]),
                args: vec![],
            }),
        }
    }
}

impl<'db> Ty<'db> {
    pub fn bool(db: &'db dyn Db) -> Self {
        Ty::Named(Named {
            name: ModulePath::new(db, vec!["std".to_string(), "Bool".to_string()]),
            args: vec![],
        })
    }
}
