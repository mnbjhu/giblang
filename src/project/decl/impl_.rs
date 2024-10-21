use std::collections::HashMap;

use crate::{db::input::Db, project::ImplForDecl, ty::Ty};

// Create generic arg

impl<'db> ImplForDecl<'db> {
    #[must_use]
    pub fn map(self, db: &'db dyn Db, ty: &Ty<'db>) -> Ty<'db> {
        let mut implied = HashMap::new();
        self.from_ty(db).imply_generic_args(ty, &mut implied);
        self.to_ty(db).unwrap().parameterize(&implied)
    }
}
