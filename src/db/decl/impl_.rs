use std::collections::HashMap;

use crate::{
    db::{decl::Decl, input::Db},
    ty::{Generic, Ty},
};

#[salsa::tracked]
#[derive()]
pub struct ImplForDecl<'db> {
    pub generics: Vec<Generic<'db>>,
    #[id]
    pub from_ty: Ty<'db>,
    #[id]
    pub to_ty: Option<Ty<'db>>,
    pub functions: Vec<Decl<'db>>,
}

impl<'db> ImplForDecl<'db> {
    #[must_use]
    pub fn map(self, db: &'db dyn Db, ty: &Ty<'db>) -> Ty<'db> {
        let mut implied = HashMap::new();
        self.from_ty(db).imply_generic_args(ty, &mut implied);
        self.to_ty(db).unwrap().parameterize(&implied)
    }
}
