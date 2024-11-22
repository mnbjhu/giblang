use std::collections::HashMap;

use crate::{
    db::{
        decl::Decl,
        input::{Db, SourceFile},
    },
    ty::{Generic, Ty},
};

#[salsa::tracked]
pub struct ImplForDecl<'db> {
    #[id]
    pub id: u32,
    pub file: SourceFile,
    pub generics: Vec<Generic<'db>>,
    pub from_ty: Ty<'db>,
    pub to_ty: Option<Ty<'db>>,
    #[return_ref]
    pub functions: Vec<Decl<'db>>,
}

impl<'db> ImplForDecl<'db> {
    #[must_use]
    pub fn map(self, db: &'db dyn Db, ty: &Ty<'db>) -> Ty<'db> {
        let mut implied = HashMap::new();
        self.from_ty(db).imply_generic_args(ty, &mut implied);
        self.to_ty(db).unwrap().parameterize(&implied)
    }

    #[must_use]
    pub fn try_map(self, db: &'db dyn Db, ty: &Ty<'db>) -> Ty<'db> {
        let mut implied = HashMap::new();
        self.from_ty(db).imply_generic_args(ty, &mut implied);
        self.to_ty(db).unwrap().parameterize(&implied)
    }
}
