use std::collections::HashMap;

use gvm::format::instr::ByteCode;

use crate::{
    check::{build_state::BuildState, state::CheckState, SemanticToken, TokenKind},
    db::decl::{struct_::StructDecl, Decl, DeclKind},
    ir::{ContainsOffset, IrNode, IrState},
    parser::expr::field::Field,
    ty::{Named, Ty},
    util::{Span, Spanned},
};

use super::{ExprIR, ExprIRData};

#[derive(Debug, PartialEq, Clone)]
pub struct FieldIR<'db> {
    pub name: Spanned<String>,
    pub struct_: Box<Spanned<ExprIR<'db>>>,
    pub decl: Option<Decl<'db>>,
}

impl<'db> Field {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let struct_ty = self.struct_.0.check(state);
        if self.name.0.is_empty() {
            return ExprIR {
                data: ExprIRData::Field(FieldIR {
                    name: self.name.clone(),
                    struct_: Box::new((struct_ty, self.struct_.1)),
                    decl: None,
                }),
                ty: Ty::Unknown,
            };
        }
        if let Ty::Named(Named { name, args }) = &struct_ty.ty {
            let decl = state.try_get_decl_path(*name);
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
                                let ty = field.1.parameterize(&params);
                                return ExprIR {
                                    data: ExprIRData::Field(FieldIR {
                                        name: self.name.clone(),
                                        struct_: Box::new((struct_ty, self.struct_.1)),
                                        decl: Some(decl),
                                    }),
                                    ty,
                                };
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
                                let ty = tys[index].parameterize(&params);
                                return ExprIR {
                                    data: ExprIRData::Field(FieldIR {
                                        name: self.name.clone(),
                                        struct_: Box::new((struct_ty, self.struct_.1)),
                                        decl: Some(decl),
                                    }),
                                    ty,
                                };
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
        ExprIR {
            data: ExprIRData::Field(FieldIR {
                name: self.name.clone(),
                struct_: Box::new((struct_ty, self.struct_.1)),
                decl: None,
            }),
            ty: Ty::Unknown,
        }
    }

    pub fn expect(
        &self,
        state: &mut CheckState<'db>,
        expected: &Ty<'db>,
        span: Span,
    ) -> ExprIR<'db> {
        let ir = self.check(state);
        ir.ty.expect_is_instance_of(expected, state, span);
        ir
    }
}

impl<'db> IrNode<'db> for FieldIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        if self.struct_.1.contains_offset(offset) {
            return self.struct_.0.at_offset(offset, state);
        }
        self
    }
    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        self.struct_.0.tokens(tokens, state);
        tokens.push(SemanticToken {
            kind: TokenKind::Property,
            span: self.name.1,
        });
    }
}

impl<'db> FieldIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<ByteCode> {
        let mut code = self.struct_.0.build(state);
        let DeclKind::Struct { body, .. } = self.decl.unwrap().kind(state.db) else {
            panic!("Expected struct")
        };
        match body {
            StructDecl::Fields(fields) => {
                let mut field = fields
                    .iter()
                    .position(|field| field.0 == self.name.0)
                    .unwrap();
                field = fields.len() - field - 1;
                code.push(ByteCode::Index(field as u32));
            }
            StructDecl::Tuple(_) => todo!(),
            StructDecl::None => panic!("A unit struct has no fields"),
        }
        code
    }
}
