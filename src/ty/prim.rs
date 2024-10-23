use crate::db::{input::Db, modules::ModulePath};

use super::Ty;

impl<'db> Ty<'db> {
    pub fn string(db: &'db dyn Db) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["String".to_string()]),
            args: Vec::new(),
        }
    }

    pub fn int(db: &'db dyn Db) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["Int".to_string()]),
            args: Vec::new(),
        }
    }

    pub fn bool(db: &'db dyn Db) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["Bool".to_string()]),
            args: Vec::new(),
        }
    }

    pub fn float(db: &'db dyn Db) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["Float".to_string()]),
            args: Vec::new(),
        }
    }

    pub fn char(db: &'db dyn Db) -> Self {
        Ty::Named {
            name: ModulePath::new(db, vec!["Char".to_string()]),
            args: Vec::new(),
        }
    }
}

