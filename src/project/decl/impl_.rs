use std::collections::HashMap;

use salsa::Database;

use crate::{project::ImplData, ty::Ty};

// Create generic arg

impl<'db> ImplData<'db> {
    #[must_use]
    pub fn map(self, db: &'db dyn Database, ty: &Ty<'db>) -> Ty<'db> {
        let mut implied = HashMap::new();
        self.from_ty(db)
            .imply_generic_args(ty.clone(), &mut implied);
        self.to_ty(db).parameterize(&implied)
    }
}
