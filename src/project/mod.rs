use std::collections::HashMap;

use decl::Decl;

use crate::{
    db::{
        input::Db,
        modules::{Module, ModuleData, ModulePath},
    },
    ty::{Generic, Ty},
};

pub mod decl;
pub mod file_data;
pub mod inst;
pub mod name;
pub mod util;

#[salsa::tracked]
pub struct Project<'db> {
    pub decls: Module<'db>,
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
        let module = self.decls(db).get_path(db, path)?;
        match module.content(db) {
            ModuleData::Export(decl) => Some(*decl),
            ModuleData::Package(_) => None,
        }
    }

    pub fn get_impls(self, db: &'db dyn Db, path: ModulePath<'db>) -> Vec<ImplForDecl<'db>> {
        self.impl_map(db).get(&path).cloned().unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct TypeVar<'db> {
    pub id: ModulePath<'db>,
    pub generic: Generic<'db>,
    pub ty: Option<Ty<'db>>,
}
