use std::collections::HashMap;

use decl::Decl;

use crate::{
    db::{input::Db, modules::ModulePath},
    ty::{Generic, Ty},
};

pub mod decl;
pub mod file_data;
pub mod inst;
pub mod name;
pub mod util;

#[salsa::tracked]
pub struct Project<'db> {
    pub decls: Decl<'db>,
    pub impl_map: HashMap<ModulePath<'db>, Vec<ImplForDecl<'db>>>,
}

#[salsa::tracked]
pub struct ImplForDecl<'db> {
    pub generics: Vec<Generic<'db>>,
    #[id]
    pub from_ty: Ty<'db>,
    #[id]
    pub to_ty: Option<Ty<'db>>,
    pub functions: Vec<Decl<'db>>,
}

impl<'db> Project<'db> {
    pub fn get_decl(self, db: &'db dyn Db, path: ModulePath<'db>) -> Option<Decl<'db>> {
        self.decls(db).get_path(db, path)
    }

    pub fn get_impls(self, db: &'db dyn Db, path: ModulePath<'db>) -> Vec<ImplForDecl<'db>> {
        self.impl_map(db).get(&path).cloned().unwrap_or_default()
    }
}
