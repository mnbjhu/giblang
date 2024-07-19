use std::collections::HashMap;

use crate::{
    check::{ty::Ty, CheckState, NamedExpr},
    fs::{export::Export, project::Project},
    parser::{
        common::pattern::{Pattern, StructFieldPattern},
        top::{
            enum_member::EnumMember, struct_::Struct, struct_body::StructBody,
            struct_field::StructField,
        },
    },
    util::Span,
};

impl Pattern {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        ty: Ty<'module>,
    ) {
        if let Pattern::Name(name) = self {
            return state.insert(name.to_string(), NamedExpr::Variable(ty));
        }
        let name = self.name();
        let def = state.get_path(name, project, true);
        if let NamedExpr::Imported(e, path) = def {
            let file = if let Export::Member { .. } = e {
                project.get_file(&path[0..path.len() - 2])
            } else {
                project.get_file(&path[0..path.len() - 1])
            };

            let mut im_state = CheckState::from_file(file);
            im_state.import_all(&file.ast, project);
            if let Export::Struct(Struct { body: expected, .. })
            | Export::Member {
                member: EnumMember { body: expected, .. },
                ..
            } = e
            {
                match (self, expected) {
                    (Pattern::TupleStruct { fields, .. }, StructBody::Tuple(tys)) => {
                        let mut expected = vec![];
                        for ty in tys {
                            expected.push(ty.0.check(project, &mut im_state, false));
                        }
                        expected.iter().zip(fields).for_each(|(ty, field)| {
                            field.0.check(project, state, ty.clone());
                        });
                    }
                    (Pattern::Struct { fields, .. }, StructBody::Fields(expected)) => {
                        let mut e = HashMap::new();
                        for (StructField { name, ty }, _) in expected {
                            e.insert(name.0.to_string(), ty.0.check(project, state, false));
                        }
                        for field in fields {
                            field.0.check(project, state, &e, field.1);
                        }
                    }
                    (Pattern::UnitStruct(_), StructBody::None) => {}
                    _ => {
                        state.error(
                            "Struct fields don't match it's definition",
                            name.last().unwrap().1,
                        );
                    }
                }
            } else {
                state.error("Expected struct or enum member", name.last().unwrap().1);
            }
        }
    }
}

impl StructFieldPattern {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        fields: &HashMap<String, Ty<'module>>,
        span: Span,
    ) {
        match self {
            StructFieldPattern::Implied(name) => {
                if let Some(ty) = fields.get(name) {
                    state.insert(name.to_string(), NamedExpr::Variable(ty.clone()));
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
