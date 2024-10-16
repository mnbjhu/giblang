use std::collections::HashMap;

use crate::{
    check::state::{CheckState, VarDecl},
    ty::Ty,
};

impl<'db> VarDecl<'db> {
    pub fn hover(
        &self,
        state: &mut CheckState<'_, 'db>,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> String {
        format!(
            "{}: {}",
            self.name,
            self.ty.get_name_with_types(state, type_vars)
        )
    }
}
