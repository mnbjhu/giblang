use std::collections::HashMap;

use crate::{
    check::CheckState,
    parser::common::pattern::{Pattern, StructFieldPattern},
    project::Project,
    resolve::top::{Decl, StructDecl},
    ty::Ty,
    util::Span,
};

impl Pattern {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        ty: Ty,
    ) {
        if let Pattern::Name(name) = self {
            state.insert_variable(name.to_string(), ty);
            return;
        }
        let name = self.name();
        let decl_id = state.get_decl_with_error(name);
        if let Some(decl_id) = decl_id {
            if let Decl::Member { body, .. } | Decl::Struct { body, .. } = project.get_decl(decl_id)
            {
                match (self, body) {
                    (Pattern::Struct { name, fields }, StructDecl::Fields(expected)) => {
                        let expected = expected.iter().cloned().collect::<HashMap<_, _>>();
                        for field in fields {
                            field.0.check(project, state, &expected, name[0].1);
                        }
                    }
                    (Pattern::UnitStruct(_), StructDecl::None) => {}
                    (Pattern::TupleStruct { fields, .. }, StructDecl::Tuple(tys)) => {
                        for (field, ty) in fields.iter().zip(tys) {
                            field.0.check(project, state, ty.clone());
                        }
                    }
                    (Pattern::Name(_), _) => unreachable!(),
                    _ => state.error("Struct pattern doesn't match expected", name[0].1),
                }
            } else {
                state.error("Expected a struct", name[0].1);
            }
        }
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
