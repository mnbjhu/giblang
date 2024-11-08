use std::{collections::HashMap, ops::ControlFlow};

use crate::{
    check::{err::CheckError, state::CheckState, Check, ControlIter, Dir, TokenKind},
    db::decl::{struct_::StructDecl, DeclKind},
    item::AstItem,
    parser::common::pattern::{Pattern, StructFieldPattern},
    ty::{Generic, Named, Ty},
    util::Span,
};

#[allow(clippy::too_many_lines)]
impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, (), &Ty<'db>> for Pattern {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        ty: &Ty<'db>,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        control.act(self, state, Dir::Enter, span)?;
        if let Pattern::Name(name) = self {
            state.insert_variable(name.0.to_string(), ty.clone(), TokenKind::Var, name.1);
            control.act(self, state, Dir::Exit(ty.clone()), span)?;
            return ControlFlow::Continue(());
        }
        let name = self.name();
        let name_span = Span::new(name[0].1.start, name.last().unwrap().1.end);
        control.act(name, state, Dir::Enter, name_span)?;
        control.act(name, state, Dir::Exit(Ty::unit()), name_span)?;
        let decl = state.get_decl_with_error(name);
        match decl {
            Ok(decl) => {
                let kind = decl.kind(state.db);
                if let DeclKind::Member { body } | DeclKind::Struct { body, .. } = kind {
                    // TODO: THIS NEEDS TESTING - Remove block to fallback
                    let mut ty = if let Ty::TypeVar { .. } = &ty {
                        let new = decl.get_named_ty(state).inst(state, name.last().unwrap().1);
                        ty.expect_is_instance_of(&new, state, name.last().unwrap().1);
                        new
                    } else {
                        ty.clone()
                    };
                    if let Ty::Generic(Generic { name, super_, .. }) = ty.clone() {
                        if name.0 == "Self" {
                            ty = super_.as_ref().clone();
                        }
                    }
                    if let Ty::Named(Named {
                        name: expected_name,
                        args,
                    }) = &ty
                    {
                        let ty_decl_id = if let DeclKind::Member { .. } = kind {
                            decl.path(state.db).get_parent(state.db)
                        } else {
                            decl.path(state.db)
                        };
                        if *expected_name != ty_decl_id {
                            state.simple_error(
                                &format!(
                                    "Expected struct '{}' but found '{}'",
                                    state.try_get_decl_path(*expected_name).map_or(
                                        format!(
                                            "Error getting name {:?}",
                                            expected_name.name(state.db)
                                        ),
                                        |t| t.name(state.db)
                                    ),
                                    state.try_get_decl_path(ty_decl_id).map_or(
                                        format!(
                                            "Error getting name {:?}",
                                            ty_decl_id.name(state.db)
                                        ),
                                        |t| t.name(state.db)
                                    ),
                                ),
                                name.last().unwrap().1,
                            );
                            control.act(self, state, Dir::Exit(ty.clone()), span)?;
                            return ControlFlow::Continue(());
                        }
                        let parent_decl = state.try_get_decl_path(*expected_name).unwrap();
                        let generics = parent_decl
                            .generics(state.db)
                            .iter()
                            .zip(args)
                            .map(|(gen, arg)| (gen.name.0.clone(), arg.clone()))
                            .collect::<HashMap<_, _>>();

                        match (self, body) {
                            (Pattern::Struct { name, fields }, StructDecl::Fields(expected)) => {
                                let expected = expected
                                    .iter()
                                    .map(|(field, ty)| (field.clone(), ty.parameterize(&generics)))
                                    .collect::<HashMap<_, _>>();
                                for field in fields {
                                    field.0.check(state, control, name[0].1, &expected)?;
                                }
                            }
                            (Pattern::UnitStruct(_), StructDecl::None) => {}
                            (Pattern::TupleStruct { fields, .. }, StructDecl::Tuple(tys)) => {
                                for (field, ty) in fields.iter().zip(tys) {
                                    field.0.check(
                                        state,
                                        control,
                                        field.1,
                                        &ty.parameterize(&generics),
                                    )?;
                                }
                            }
                            (Pattern::Name(_), _) => unreachable!(),
                            _ => state.simple_error(
                                "Struct pattern doesn't match expected",
                                name.last().unwrap().1,
                            ),
                        }
                    } else {
                        state.simple_error(
                            &format!(
                                "Expected a struct but found type {}",
                                ty.get_name(state, None)
                            ),
                            name.last().unwrap().1,
                        );
                    }
                } else {
                    state.simple_error("Expected a struct", name.last().unwrap().1);
                }
            }
            Err(e) => {
                state.error(CheckError::Unresolved(e));
            }
        }
        control.act(self, state, Dir::Exit(ty.clone()), span)?;
        ControlFlow::Continue(())
    }
}

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, (), &HashMap<String, Ty<'db>>>
    for StructFieldPattern
{
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        fields: &HashMap<String, Ty<'db>>,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        match self {
            StructFieldPattern::Implied(name) => {
                if let Some(ty) = fields.get(&name.0) {
                    state.insert_variable(name.0.to_string(), ty.clone(), TokenKind::Var, name.1);
                } else {
                    state.simple_error(&format!("Field '{}' not found", name.0), span);
                }
            }
            StructFieldPattern::Explicit { field, pattern } => {
                if let Some(ty) = fields.get(&field.0) {
                    pattern.0.check(state, control, pattern.1, ty)?;
                } else {
                    state.simple_error(&format!("Field '{}' not found", field.0), field.1);
                }
            }
        };
        ControlFlow::Continue(())
    }
}
