use crate::{
    check::state::CheckState,
    parser::expr::property::Property,
    project::decl::{struct_::StructDecl, Decl},
    ty::Ty,
    util::Span,
};

impl Property {
    pub fn check(&self, state: &mut CheckState<'_>) -> Ty {
        let expr = self.expr.0.as_ref().check(state);
        if let Ty::Named { name, .. } = expr {
            let decl = state.project.get_decl(name);
            if let Decl::Struct { body, .. } = decl {
                match body {
                    StructDecl::Fields(fields) => fields
                        .iter()
                        .find_map(|(name, ty)| {
                            if name == &self.name.0 {
                                Some(ty.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap_or(Ty::Unknown),
                    StructDecl::Tuple(tys) => {
                        if self.name.0.starts_with('_') {
                            let name = self.name.0.strip_prefix('_').unwrap();
                            let num = name.parse::<usize>();
                            if let Ok(num) = num {
                                tys.get(num).cloned().unwrap_or(Ty::Unknown)
                            } else {
                                state.simple_error(
                                    &format!("'{name}' isn't a valid integer"),
                                    self.name.1,
                                );
                                Ty::Unknown
                            }
                        } else {
                            state.simple_error(
                                "Tuple struct fields must begin with '_'",
                                self.name.1,
                            );
                            Ty::Unknown
                        }
                    }
                    StructDecl::None => {
                        state.simple_error("Unit struct has no fields", self.expr.1);
                        Ty::Unknown
                    }
                }
            } else {
                Ty::Unknown
            }
        } else {
            Ty::Unknown
        }
    }

    pub fn expected_instance_of(&self, expected: &Ty, state: &mut CheckState<'_>, _: Span) {
        let actual = self.check(state);
        actual.expect_is_instance_of(expected, state, false, self.name.1);
    }
}
