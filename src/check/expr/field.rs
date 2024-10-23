use std::collections::HashMap;

use crate::{
    check::state::CheckState, db::decl::{struct_::StructDecl, DeclKind}, parser::expr::field::Field, ty::Ty, util::Span
};

impl<'db> Field {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        let struct_ty = self.struct_.0.check(state);
        if self.name.0.is_empty() {
            return Ty::Unknown;
        }
        if let Ty::Named { name, args } = struct_ty {
            let decl = state.try_get_decl(name);
            if let Some(decl) = decl {
                if let DeclKind::Struct { body, generics } = decl.kind(state.db) {
                    let params = generics
                        .iter()
                        .map(|arg| arg.name.0.clone())
                        .zip(args.iter().cloned())
                        .collect::<HashMap<_, _>>();
                    match body {
                        StructDecl::Fields(fields) => {
                            if let Some(field) = fields.iter().find(|field| field.0 == self.name.0)
                            {
                                return field.1.parameterize(&params);
                            }
                            state.simple_error(
                                &format!(
                                    "No field {} found on struct {}",
                                    self.name.0,
                                    name.name(state.db).join("::")
                                ),
                                self.name.1,
                            );
                        }
                        StructDecl::Tuple(tys) => {
                            if let Ok(index) = self.name.0.parse::<usize>() {
                                if index >= tys.len() {
                                    state.simple_error(
                                        &format!("Index out of bounds: {} >= {}", index, tys.len()),
                                        self.name.1,
                                    );
                                }
                                return tys[index].parameterize(&params);
                            }
                            state.simple_error("Expected integer index", self.name.1);
                        }
                        StructDecl::None => {
                            state.simple_error("A unit struct has no fields", self.name.1);
                        }
                    }
                } else {
                    state.simple_error(
                        &format!(
                            "Expected struct but found {}",
                            decl.path(state.db).name(state.db).join("::")
                        ),
                        self.struct_.1,
                    );
                }
            }
        }
        Ty::Unknown
    }

    pub fn expected_instance_of(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) {
        self.check(state)
            .expect_is_instance_of(expected, state, false, span);
    }
}
