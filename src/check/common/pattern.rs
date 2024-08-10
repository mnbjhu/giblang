use std::collections::HashMap;

use crate::{
    check::CheckState,
    parser::common::pattern::{Pattern, StructFieldPattern},
    project::{
        decl::{struct_::StructDecl, Decl},
        Project,
    },
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
            let decl = project.get_decl(decl_id);
            if let Decl::Member { body, .. } | Decl::Struct { body, .. } = decl {
                if let Ty::Named {
                    name: expected_name,
                    args,
                } = &ty
                {
                    let ty_decl_id = if let Decl::Member { .. } = decl {
                        project.get_parent(decl_id).unwrap()
                    } else {
                        decl_id
                    };
                    if *expected_name != ty_decl_id {
                        state.simple_error(
                            &format!(
                                "Expected struct '{}' but found '{}'",
                                project.get_decl(*expected_name).name(),
                                project.get_decl(ty_decl_id).name()
                            ),
                            name.last().unwrap().1,
                        );
                        return;
                    }
                    let implied = project
                        .get_decl(ty_decl_id)
                        .generics()
                        .iter()
                        .map(|arg| arg.name.to_string())
                        .zip(args.iter().cloned())
                        .collect::<HashMap<_, _>>();

                    match (self, body) {
                        (Pattern::Struct { name, fields }, StructDecl::Fields(expected)) => {
                            let expected = expected.iter().cloned().collect::<HashMap<_, _>>();
                            for field in fields {
                                field
                                    .0
                                    .check(project, state, &expected, name[0].1, &implied);
                            }
                        }
                        (Pattern::UnitStruct(_), StructDecl::None) => {}
                        (Pattern::TupleStruct { fields, .. }, StructDecl::Tuple(tys)) => {
                            for (field, ty) in fields.iter().zip(tys) {
                                field.0.check(project, state, ty.clone());
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

impl StructFieldPattern {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        fields: &HashMap<String, Ty>,
        span: Span,
        implied: &HashMap<String, Ty>,
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
                    pattern.0.check(project, state, ty.clone());
                } else {
                    state.simple_error(&format!("Field '{}' not found", field.0), field.1);
                }
            }
        }
    }
}
