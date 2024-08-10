use std::collections::HashMap;

use crate::{check::state::CheckState, project::ImplData, ty::Ty};

impl ImplData {
    #[must_use]
    pub fn map(&self, ty: &Ty, state: &mut CheckState) -> Ty {
        let mut type_vars = HashMap::new();
        for geneirc in &self.generics {
            let id = state.add_type_var(geneirc.clone());
            type_vars.insert(geneirc.name.clone(), id);
        }
        let from = self.from.inst(&mut type_vars, state);
        from.imply_type_vars(ty, state);
        self.to.inst(&mut type_vars, state)
    }
}
