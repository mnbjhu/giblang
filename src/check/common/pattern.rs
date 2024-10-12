use std::collections::HashMap;

use crate::{
    check::state::CheckState,
    parser::common::pattern::{Pattern, StructFieldPattern},
    project::decl::{struct_::StructDecl, DeclKind},
    ty::Ty,
    util::Span,
};

impl<'db> Pattern {
    pub fn check(&self, state: &mut CheckState<'_, 'db>, ty: Ty<'db>) {
        if let Pattern::Name(name) = self {
            state.insert_variable(name.to_string(), ty);
            return;
        }
        let name = self.name();
        let decl_id = state.get_decl_with_error(name);
        if let Some(decl_id) = decl_id {
            let decl = state.project.get_decl(state.db, decl_id).unwrap();
            let kind = decl.kind(state.db);
            if let DeclKind::Member { body, .. } | DeclKind::Struct { body, .. } = kind {
                if let Ty::Named {
                    name: expected_name,
                    ..
                } = &ty
                {
                    let ty_decl_id = if let DeclKind::Member { .. } = kind {
                        decl_id.get_parent(state.db)
                    } else {
                        decl_id
                    };
                    if *expected_name != ty_decl_id {
                        state.simple_error(
                            &format!(
                                "Expected struct '{}' but found '{}'",
                                state
                                    .project
                                    .get_decl(state.db, *expected_name)
                                    .unwrap()
                                    .name(state.db),
                                state
                                    .project
                                    .get_decl(state.db, ty_decl_id)
                                    .unwrap()
                                    .name(state.db)
                            ),
                            name.last().unwrap().1,
                        );
                        return;
                    }

                    match (self, body) {
                        (Pattern::Struct { name, fields }, StructDecl::Fields(expected)) => {
                            let expected = expected.iter().cloned().collect::<HashMap<_, _>>();
                            for field in fields {
                                field.0.check(state, &expected, name[0].1);
                            }
                        }
                        (Pattern::UnitStruct(_), StructDecl::None) => {}
                        (Pattern::TupleStruct { fields, .. }, StructDecl::Tuple(tys)) => {
                            for (field, ty) in fields.iter().zip(tys) {
                                field.0.check(state, ty.clone());
                            }
                        }
                        (Pattern::Name(_), _) => unreachable!(),
                        _ => state.simple_error(
                            "Struct pattern doesn't match expected",
                            name.last().unwrap().1,
                        ),
                    }
                } else {
                    state.simple_error("Expected a struct", name.last().unwrap().1);
                }
            } else {
                state.simple_error("Expected a struct", name.last().unwrap().1);
            }
        }
    }
}

impl<'db> StructFieldPattern {
    pub fn check(
        &self,
        state: &mut CheckState<'_, 'db>,
        fields: &HashMap<String, Ty<'db>>,
        span: Span,
    ) {
        match self {
            StructFieldPattern::Implied(name) => {
                if let Some(ty) = fields.get(name) {
                    state.insert_variable(name.to_string(), ty.clone());
                } else {
                    state.simple_error(&format!("Field '{name}' not found"), span);
                }
            }
            StructFieldPattern::Explicit { field, pattern } => {
                if let Some(ty) = fields.get(&field.0) {
                    pattern.0.check(state, ty.clone());
                } else {
                    state.simple_error(&format!("Field '{}' not found", field.0), field.1);
                }
            }
        }
    }
}
