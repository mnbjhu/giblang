use std::collections::HashMap;

use crate::{db::input::Db, project::ImplDecl, ty::Ty};

// Create generic arg

impl<'db> ImplDecl<'db> {
    #[must_use]
    pub fn map(self, db: &'db dyn Db, ty: &Ty<'db>) -> Ty<'db> {
        let mut implied = HashMap::new();
        self.from_ty(db).imply_generic_args(ty, &mut implied);
        self.to_ty(db).parameterize(&implied)
    }
}
