use std::collections::HashMap;

use crate::{
    check::CheckState,
    parser::common::pattern::{Pattern, StructFieldPattern},
    project::Project,
    ty::Ty,
    util::Span,
};

impl Pattern {
    pub fn check(&self, project: &Project, state: &mut CheckState, ty: Ty) {
        if let Pattern::Name(name) = self {
            return state.insert_variable(name.to_string(), ty);
        }
        let name = self.name();
        // TODO: Re-implement
    }
}

impl StructFieldPattern {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        fields: &HashMap<String, Ty>,
        span: Span,
    ) {
        match self {
            StructFieldPattern::Implied(name) => {
                if let Some(ty) = fields.get(name) {
                    state.insert_variable(name.to_string(), ty.clone());
                } else {
                    state.error(&format!("Field '{}' not found", name), span);
                }
            }
            StructFieldPattern::Explicit { field, pattern } => {
                if let Some(ty) = fields.get(&field.0) {
                    pattern.0.check(project, state, ty.clone());
                } else {
                    state.error(&format!("Field '{}' not found", field.0), field.1);
                }
            }
        }
    }
}
