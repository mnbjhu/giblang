use std::collections::HashMap;

use crate::{
    check::{err::CheckError, state::CheckState, SemanticToken, TokenKind},
    db::decl::{struct_::StructDecl, DeclKind},
    ir::{ContainsOffset, IrNode},
    item::definitions::ident::IdentDef,
    parser::common::pattern::{Pattern, StructFieldPattern},
    ty::{Generic, Named, Ty},
    util::{Span, Spanned},
};

pub type SpannedQualifiedNameIR<'db> = Vec<Spanned<IdentDef<'db>>>;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum PatternIR<'db> {
    Name(Spanned<String>),
    Struct {
        name: SpannedQualifiedNameIR<'db>,
        fields: Vec<Spanned<StructFieldPatternIR<'db>>>,
    },
    UnitStruct(SpannedQualifiedNameIR<'db>),
    TupleStruct {
        name: SpannedQualifiedNameIR<'db>,
        fields: Vec<Spanned<PatternIR<'db>>>,
    },
    Error,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum StructFieldPatternIR<'db> {
    Implied {
        name: Spanned<String>,
    },
    Explicit {
        field: Spanned<String>,
        pattern: Spanned<PatternIR<'db>>,
    },
}

#[allow(clippy::too_many_lines)]
impl<'db> Pattern {
    pub fn check(&self, state: &mut CheckState<'db>, ty: &Ty<'db>) -> PatternIR<'db> {
        if let Pattern::Name(name) = self {
            state.insert_variable(name.0.to_string(), ty.clone(), TokenKind::Var, name.1);
            return PatternIR::Name(name.clone());
        }
        let name = self.name();
        let name_span = Span::new(name[0].1.start, name.last().unwrap().1.end);
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
                            // TODO: This should still create the ir
                            return PatternIR::Error;
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
                                let fields = fields
                                    .iter()
                                    .map(|f| (f.0.check(state, &expected), f.1))
                                    .collect();
                                PatternIR::Struct {
                                    name: state.get_ident_ir(name),
                                    fields,
                                }
                            }
                            (Pattern::UnitStruct(name), StructDecl::None) => {
                                PatternIR::UnitStruct(state.get_ident_ir(name))
                            }
                            (Pattern::TupleStruct { fields, name }, StructDecl::Tuple(tys)) => {
                                let fields = fields
                                    .iter()
                                    .zip(tys)
                                    .map(|((field, span), ty)| {
                                        (field.check(state, &ty.parameterize(&generics)), *span)
                                    })
                                    .collect();
                                PatternIR::TupleStruct {
                                    name: state.get_ident_ir(name),
                                    fields,
                                }
                            }
                            (Pattern::Name(_), _) => unreachable!(),
                            _ => {
                                state.simple_error(
                                    "Struct pattern doesn't match expected",
                                    name.last().unwrap().1,
                                );
                                PatternIR::Error
                            }
                        }
                    } else {
                        state.simple_error(
                            &format!(
                                "Expected a struct but found type {}",
                                ty.get_name(state, None)
                            ),
                            name.last().unwrap().1,
                        );
                        PatternIR::Error
                    }
                } else {
                    state.simple_error("Expected a struct", name.last().unwrap().1);
                    PatternIR::Error
                }
            }
            Err(e) => {
                state.error(CheckError::Unresolved(e));
                PatternIR::Error
            }
        }
    }
}

impl<'db> StructFieldPattern {
    pub fn check(
        &self,
        state: &mut CheckState<'db>,
        fields: &HashMap<String, Ty<'db>>,
    ) -> StructFieldPatternIR<'db> {
        match self {
            StructFieldPattern::Implied(name) => {
                if let Some(ty) = fields.get(&name.0) {
                    state.insert_variable(name.0.to_string(), ty.clone(), TokenKind::Var, name.1);
                } else {
                    state.simple_error(&format!("Field '{}' not found", name.0), name.1);
                }
                StructFieldPatternIR::Implied { name: name.clone() }
            }
            StructFieldPattern::Explicit {
                field,
                pattern: (pattern, span),
            } => {
                let pattern = if let Some(ty) = fields.get(&field.0) {
                    pattern.check(state, ty)
                } else {
                    state.simple_error(&format!("Field '{}' not found", field.0), field.1);
                    PatternIR::Error
                };
                StructFieldPatternIR::Explicit {
                    field: field.clone(),
                    pattern: (pattern, *span),
                }
            }
        }
    }
}

impl<'db> IrNode<'db> for PatternIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        match self {
            PatternIR::Name(name) => self,
            PatternIR::Struct { name, fields } => {
                for (field, span) in fields {
                    if span.contains_offset(offset) {
                        return field.at_offset(offset, state);
                    }
                }
                self
            }
            PatternIR::UnitStruct(name) => name.at_offset(offset, state),
            PatternIR::TupleStruct { name, fields } => {
                for (field, span) in fields {
                    if span.contains_offset(offset) {
                        return field.at_offset(offset, state);
                    }
                }
                self
            }
            PatternIR::Error => self,
        }
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        match self {
            PatternIR::Name(name) => {
                tokens.push(SemanticToken {
                    span: name.1,
                    kind: TokenKind::Var,
                });
            }
            PatternIR::Struct { name, fields } => {
                for (field, _) in fields {
                    field.tokens(tokens, state);
                }
            }
            PatternIR::UnitStruct(name) => name.tokens(tokens, state),
            PatternIR::TupleStruct { name, fields } => {
                for (field, _) in fields {
                    field.tokens(tokens, state);
                }
            }
            PatternIR::Error => {}
        }
    }
}

impl<'db> IrNode<'db> for StructFieldPatternIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        match self {
            StructFieldPatternIR::Implied { name } => self,
            StructFieldPatternIR::Explicit { field, pattern } => {
                if pattern.1.contains_offset(offset) {
                    return pattern.0.at_offset(offset, state);
                }
                self
            }
        }
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        match self {
            StructFieldPatternIR::Implied { name } => {
                tokens.push(SemanticToken {
                    span: name.1,
                    kind: TokenKind::Var,
                });
            }
            StructFieldPatternIR::Explicit { field, pattern } => {
                pattern.0.tokens(tokens, state);
                tokens.push(SemanticToken {
                    span: field.1,
                    kind: TokenKind::Property,
                });
            }
        }
    }
}
