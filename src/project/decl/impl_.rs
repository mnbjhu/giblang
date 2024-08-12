use std::collections::HashMap;

use crate::{project::ImplData, ty::Ty};

// Create generic arg

impl ImplData {
    #[must_use]
    pub fn map(&self, ty: &Ty) -> Ty {
        let mut implied = HashMap::new();
        self.from.imply_generic_args(ty, &mut implied);
        self.to.parameterize(&implied)
    }
}
