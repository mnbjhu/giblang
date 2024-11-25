use std::collections::HashMap;

use async_lsp::lsp_types::CompletionItem;
use gvm::format::{instr::ByteCode, literal::Literal};
use salsa::plumbing::AsId as _;

use crate::{
    check::{
        build_state::BuildState,
        err::CheckError,
        scoped_state::Scoped,
        state::{CheckState, VarDecl},
        SemanticToken, TokenKind,
    },
    db::decl::{struct_::StructDecl, DeclKind},
    ir::{
        builder::ByteCodeNode,
        expr::{ident::get_ident_completions, lit::Typed},
        AstKind, ContainsOffset, IrNode, IrState,
    },
    item::definitions::ident::IdentDef,
    parser::common::pattern::{Pattern, StructFieldPattern},
    ty::{Generic, Named, Ty},
    util::{Span, Spanned},
};

pub type SpannedQualifiedNameIR<'db> = Vec<Spanned<IdentDef<'db>>>;

#[derive(Debug, PartialEq, Clone)]
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
    Exact(Spanned<Literal>),
    Wildcard(Span),
    Error,
}

#[derive(Debug, PartialEq, Clone)]
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
    pub fn check(&self, state: &mut CheckState<'db>) -> PatternIR<'db> {
        match self {
            Pattern::Name(name) => {
                let def = state.get_ident_ir(&[name.clone()]);
                if let Some(IdentDef::Decl(decl)) = def.first().map(|(d, _)| d) {
                    if let DeclKind::Struct { body, .. } | DeclKind::Member { body, .. } =
                        decl.kind(state.db)
                    {
                        if let StructDecl::None = body {
                            return PatternIR::UnitStruct(def);
                        }
                    }
                }
                let var = VarDecl {
                    name: name.0.to_string(),
                    ty: Ty::Unknown,
                    kind: TokenKind::Var,
                    span: name.1,
                };
                state.insert_variable(&name.0, var);
                PatternIR::Name(name.clone())
            }
            Pattern::Struct { name, fields } => PatternIR::Struct {
                name: state.get_ident_ir(name),
                fields: fields
                    .iter()
                    .map(|(field, span)| (field.check(state), *span))
                    .collect(),
            },
            Pattern::UnitStruct(name) => PatternIR::UnitStruct(state.get_ident_ir(name)),
            Pattern::TupleStruct { name, fields } => PatternIR::TupleStruct {
                name: state.get_ident_ir(name),
                fields: fields
                    .iter()
                    .map(|(ty, span)| (ty.check(state), *span))
                    .collect(),
            },
            Pattern::Exact(lit) => PatternIR::Exact(lit.clone()),
            Pattern::Wildcard(span) => PatternIR::Wildcard(*span),
        }
    }
    pub fn expect(&self, state: &mut CheckState<'db>, ty: &Ty<'db>) -> PatternIR<'db> {
        if let Pattern::Name(name) = self {
            let def = state.get_ident_ir(&[name.clone()]);
            if let Some(IdentDef::Decl(decl)) = def.first().map(|(d, _)| d) {
                if let DeclKind::Struct { body, .. } | DeclKind::Member { body, .. } =
                    decl.kind(state.db)
                {
                    if let StructDecl::None = body {
                        return PatternIR::UnitStruct(def);
                    }
                }
            }
            let var = VarDecl {
                name: name.0.to_string(),
                ty: ty.clone(),
                kind: TokenKind::Var,
                span: name.1,
            };
            state.insert_variable(&name.0, var);
            return PatternIR::Name(name.clone());
        }
        if let Pattern::Wildcard(span) = self {
            return PatternIR::Wildcard(*span);
        }
        if let Pattern::Exact(lit) = self {
            lit.0
                .to_ty(state.db)
                .expect_is_instance_of(ty, state, lit.1);
            return PatternIR::Exact(lit.clone());
        }
        let name = self.name();
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
                            return self.check(state);
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
                                    .map(|f| (f.0.expect(state, &expected), f.1))
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
                                        (field.expect(state, &ty.parameterize(&generics)), *span)
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
                            &format!("Expected a struct but found type {}", ty.get_name(state)),
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
    pub fn check(&self, state: &mut CheckState<'db>) -> StructFieldPatternIR<'db> {
        match self {
            StructFieldPattern::Implied(name) => {
                StructFieldPatternIR::Implied { name: name.clone() }
            }
            StructFieldPattern::Explicit { field, pattern } => StructFieldPatternIR::Explicit {
                field: field.clone(),
                pattern: (pattern.0.expect(state, &Ty::Unknown), pattern.1),
            },
        }
    }

    pub fn expect(
        &self,
        state: &mut CheckState<'db>,
        fields: &HashMap<String, Ty<'db>>,
    ) -> StructFieldPatternIR<'db> {
        match self {
            StructFieldPattern::Implied(name) => {
                if let Some(ty) = fields.get(&name.0) {
                    let var = VarDecl {
                        name: name.0.to_string(),
                        ty: ty.clone(),
                        kind: TokenKind::Var,
                        span: name.1,
                    };
                    state.insert_variable(&name.0, var);
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
                    pattern.expect(state, ty)
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
        state.kind = AstKind::Pattern;
        match self {
            PatternIR::Struct { fields, .. } => {
                for (field, span) in fields {
                    if span.contains_offset(offset) {
                        return field.at_offset(offset, state);
                    }
                }
                self
            }
            PatternIR::UnitStruct(name) => name.at_offset(offset, state),
            PatternIR::TupleStruct { fields, .. } => {
                for (field, span) in fields {
                    if span.contains_offset(offset) {
                        return field.at_offset(offset, state);
                    }
                }
                self
            }
            PatternIR::Name(_)
            | PatternIR::Error
            | PatternIR::Exact(_)
            | PatternIR::Wildcard(_) => self,
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
                name.tokens(tokens, state);
                for (field, _) in fields {
                    field.tokens(tokens, state);
                }
            }
            PatternIR::UnitStruct(name) => name.tokens(tokens, state),
            PatternIR::TupleStruct { name, fields } => {
                name.tokens(tokens, state);
                for (field, _) in fields {
                    field.tokens(tokens, state);
                }
            }
            PatternIR::Error | PatternIR::Exact(_) | PatternIR::Wildcard(_) => {}
        }
    }

    fn completions(&self, _: usize, state: &mut IrState<'db>) -> Vec<CompletionItem> {
        let mut completions = vec![];
        if let PatternIR::Name(_) = self {
            get_ident_completions(state, &mut completions);
        }
        completions
    }

    fn hover(&self, _: usize, state: &mut IrState<'db>) -> Option<String> {
        match self {
            PatternIR::Name(name) => Some(format!(
                "{}: {}",
                name.0,
                state
                    .get_variable(&name.0)
                    .map_or("Unknown".to_string(), |t| t.ty.get_ir_name(state))
            )),
            PatternIR::Struct { .. } => todo!(),
            PatternIR::UnitStruct(_) => todo!(),
            PatternIR::TupleStruct { .. } => todo!(),
            PatternIR::Wildcard(_) | PatternIR::Exact(_) => None,
            PatternIR::Error => todo!(),
        }
    }

    fn debug_name(&self) -> &'static str {
        "PatternIR"
    }
}

impl<'db> IrNode<'db> for StructFieldPatternIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        match self {
            StructFieldPatternIR::Implied { .. } => self,
            StructFieldPatternIR::Explicit { pattern, .. } => {
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

    fn debug_name(&self) -> &'static str {
        "StructFieldPatternIR"
    }
}

impl<'db> PatternIR<'db> {
    pub fn build_match(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        if let PatternIR::Wildcard(_) | PatternIR::Name(_) = self {
            return ByteCodeNode::Code(vec![]);
        };
        if let PatternIR::Exact(lit) = self {
            let code = ByteCodeNode::Code(vec![
                ByteCode::Copy,
                ByteCode::Push(lit.0.clone()),
                ByteCode::Eq,
            ]);
            return ByteCodeNode::Block(vec![code, ByteCodeNode::Next]);
        };
        let (PatternIR::TupleStruct { name, .. }
        | PatternIR::UnitStruct(name)
        | PatternIR::Struct { name, .. }) = self
        else {
            unreachable!()
        };
        let IdentDef::Decl(decl) = name.last().unwrap().0 else {
            panic!("Expected struct")
        };
        let fields = match self {
            PatternIR::Struct { fields, .. } => {
                let (DeclKind::Struct {
                    body: StructDecl::Fields(decl_fields),
                    ..
                }
                | DeclKind::Member {
                    body: StructDecl::Fields(decl_fields),
                    ..
                }) = decl.kind(state.db)
                else {
                    panic!("Expected a struct but found {}", decl.name(state.db))
                };
                let mut found = Vec::new();
                for field in fields {
                    let StructFieldPatternIR::Explicit { pattern, .. } = &field.0 else {
                        continue;
                    };
                    let mut index = decl_fields
                        .iter()
                        .position(|(f, _)| f == field.0.name())
                        .unwrap();
                    index = decl_fields.len() - index - 1;
                    found.push((index, &pattern.0));
                }
                found
            }
            PatternIR::UnitStruct(_) => vec![],
            PatternIR::TupleStruct { fields, .. } => fields
                .iter()
                .map(|(f, _)| f)
                .enumerate()
                .collect::<Vec<_>>(),
            _ => unreachable!(),
        };

        let id = decl.as_id().as_u32();
        let mut code = vec![];
        code.push(ByteCodeNode::Code(vec![
            ByteCode::Copy,
            ByteCode::Match(id),
        ]));
        code.push(ByteCodeNode::Next);
        for (index, field) in fields.iter().enumerate() {
            let cond = field.1.build_match(state);
            if cond.len() == 0 {
                continue;
            }
            let f = ByteCodeNode::Code(vec![ByteCode::Copy, ByteCode::Index(index as u32)]);
            code.push(f);
            code.push(cond);
        }
        ByteCodeNode::Block(code)
    }

    pub fn build(&self, state: &mut crate::ir::BuildState<'db>) -> ByteCodeNode {
        match self {
            PatternIR::Name(name) => {
                let id = state.add_var(name.0.clone());
                ByteCodeNode::Code(vec![ByteCode::NewLocal(id)])
            }
            PatternIR::Exact(_) | PatternIR::Wildcard(_) => ByteCodeNode::Code(vec![ByteCode::Pop]),
            PatternIR::Struct { name, fields } => {
                let IdentDef::Decl(decl) = name.last().unwrap().0 else {
                    panic!("Expected struct")
                };
                let (DeclKind::Struct {
                    body: StructDecl::Fields(decl_fields),
                    ..
                }
                | DeclKind::Member {
                    body: StructDecl::Fields(decl_fields),
                    ..
                }) = decl.kind(state.db)
                else {
                    panic!("Expected struct")
                };
                let mut code = vec![];
                for field in fields {
                    let mut index = decl_fields
                        .iter()
                        .position(|(f, _)| f == field.0.name())
                        .unwrap();
                    index = decl_fields.len() - index - 1;
                    code.push(field.0.build(state, index as u32));
                }
                code.push(ByteCodeNode::Code(vec![ByteCode::Pop]));
                ByteCodeNode::Block(code)
            }
            PatternIR::UnitStruct(_) => ByteCodeNode::Code(vec![]),
            PatternIR::TupleStruct { fields, .. } => {
                let mut code = vec![];
                for (index, field) in fields.iter().enumerate() {
                    let field_index = fields.len() - index - 1;
                    code.push(ByteCodeNode::Code(vec![
                        ByteCode::Copy,
                        ByteCode::Index(field_index as u32),
                    ]));
                    code.push(field.0.build(state));
                }
                ByteCodeNode::Block(code)
            }
            PatternIR::Error => panic!("Unexpected err pattern"),
        }
    }
}

impl<'db> StructFieldPatternIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>, index: u32) -> ByteCodeNode {
        match self {
            StructFieldPatternIR::Implied { name } => {
                let id = state.add_var(name.0.clone());
                ByteCodeNode::Code(vec![
                    ByteCode::Copy,
                    ByteCode::Index(index),
                    ByteCode::NewLocal(id),
                ])
            }
            StructFieldPatternIR::Explicit { pattern, .. } => {
                let field = ByteCodeNode::Code(vec![ByteCode::Copy, ByteCode::Index(index)]);
                let res = vec![field, pattern.0.build(state)];
                ByteCodeNode::Block(res)
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            StructFieldPatternIR::Implied { name } => &name.0,
            StructFieldPatternIR::Explicit { field, .. } => &field.0,
        }
    }
}
