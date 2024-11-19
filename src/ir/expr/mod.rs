use block::{check_block, CodeBlockIR};
use call::CallIR;
use field::FieldIR;
use ident::{check_ident, expect_ident};
use if_else::IfElseIR;
use lambda::LambdaIR;
use match_::MatchIR;
use member::MemberCallIR;
use op::OpIR;
use salsa::plumbing::AsId;
use tuple::{check_tuple, expect_tuple};
use while_::WhileIR;

use crate::{
    check::{build_state::BuildState, state::CheckState, TokenKind},
    db::{
        decl::{struct_::StructDecl, Decl, DeclKind},
        input::Db,
    },
    item::definitions::ident::IdentDef,
    lexer::literal::Literal,
    parser::expr::Expr,
    run::bytecode::ByteCode,
    ty::{Named, Ty},
    util::{Span, Spanned},
};

use super::{common::pattern::SpannedQualifiedNameIR, ContainsOffset, IrNode, IrState};

pub mod block;
pub mod call;
pub mod field;
pub mod ident;
pub mod if_else;
pub mod lambda;
pub mod lit;
pub mod match_;
pub mod match_arm;
pub mod member;
pub mod op;
pub mod tuple;
pub mod while_;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ExprIR<'db> {
    pub data: ExprIRData<'db>,
    pub ty: Ty<'db>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum ExprIRData<'db> {
    Literal(Literal),
    Field(FieldIR<'db>),
    Ident(SpannedQualifiedNameIR<'db>),
    CodeBlock(CodeBlockIR<'db>),
    Call(CallIR<'db>),
    MemberCall(MemberCallIR<'db>),
    Match(MatchIR<'db>),
    Tuple(Vec<Spanned<ExprIR<'db>>>),
    Op(OpIR<'db>),
    Lambda(LambdaIR<'db>),
    While(WhileIR<'db>),
    IfElse(IfElseIR<'db>),
    ImplicitDyn(Box<ExprIR<'db>>, Decl<'db>),
    Error,
}

impl<'db> Expr {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        match self {
            Expr::Literal(lit) => ExprIR {
                data: ExprIRData::Literal(lit.clone()),
                ty: lit.to_ty(state.db),
            },
            Expr::Field(field) => field.check(state),
            Expr::Ident(ident) => check_ident(ident, state),
            Expr::CodeBlock(block) => check_block(block, state),
            Expr::Call(call) => call.check(state),
            Expr::MemberCall(member_call) => member_call.check(state),
            Expr::Match(match_) => match_.check(state),
            Expr::Tuple(tuple) => check_tuple(tuple, state),
            Expr::Op(op) => op.check(state),
            Expr::Lambda(lambda) => lambda.check(state),
            Expr::While(while_) => while_.check(state),
            Expr::Error => ExprIR {
                data: ExprIRData::Error,
                ty: Ty::Unknown,
            },
            Expr::IfElse(if_else) => if_else.check(state),
        }
    }

    pub fn expect(
        &self,
        state: &mut CheckState<'db>,
        expected: &Ty<'db>,
        span: Span,
    ) -> ExprIR<'db> {
        let res = match self {
            Expr::Literal(lit) => {
                let ty = lit.to_ty(state.db);
                ty.expect_is_instance_of(expected, state, span);
                ExprIR {
                    data: ExprIRData::Literal(lit.clone()),
                    ty,
                }
            }
            Expr::Field(field) => field.expect(state, expected, span),
            Expr::Ident(ident) => expect_ident(ident, state, expected, span),
            Expr::CodeBlock(block) => check_block(block, state),
            Expr::Call(call) => call.expect(state, expected, span),
            Expr::MemberCall(member_call) => member_call.expect(state, expected, span),
            Expr::Match(match_) => match_.expect(state, expected, span),
            Expr::Tuple(tuple) => expect_tuple(tuple, state, expected, span),
            Expr::Op(op) => op.expect(state, expected, span),
            Expr::Lambda(lambda) => lambda.expect(state, expected, span),
            Expr::Error => ExprIR {
                data: ExprIRData::Error,
                ty: Ty::Unknown,
            },
            Expr::While(while_) => {
                let ir = while_.check(state);
                Ty::unit().expect_is_instance_of(expected, state, span);
                ir
            }
            Expr::IfElse(if_else) => if_else.expect(state, expected, span),
        };
        if let Ty::Named(Named { name, .. }) = &expected {
            let decl = state.project.get_decl(state.db, *name).unwrap();
            if matches!(decl.kind(state.db), DeclKind::Trait { .. }) && !res.data.is_dyn(state) {
                return ExprIR {
                    data: ExprIRData::ImplicitDyn(Box::new(res), decl),
                    ty: Ty::Unknown,
                };
            }
        }
        res
    }
}

impl<'db> IrNode<'db> for ExprIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut super::IrState<'db>) -> &dyn IrNode {
        match &self.data {
            ExprIRData::Literal(_) | ExprIRData::Error => self,
            ExprIRData::Field(field) => field.at_offset(offset, state),
            ExprIRData::Ident(ident) => ident.at_offset(offset, state),
            ExprIRData::CodeBlock(block) => block.at_offset(offset, state),
            ExprIRData::Call(call) => call.at_offset(offset, state),
            ExprIRData::MemberCall(member_call) => member_call.at_offset(offset, state),
            ExprIRData::Match(match_) => match_.at_offset(offset, state),
            ExprIRData::Tuple(tuple) => {
                for (e, span) in tuple {
                    if span.contains_offset(offset) {
                        return e;
                    }
                }
                self
            }
            ExprIRData::Op(op) => op.at_offset(offset, state),
            ExprIRData::Lambda(lambda) => lambda.at_offset(offset, state),
            ExprIRData::While(while_) => while_.at_offset(offset, state),
            ExprIRData::IfElse(if_else) => if_else.at_offset(offset, state),
            ExprIRData::ImplicitDyn(expr, _) => expr.at_offset(offset, state),
        }
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut super::IrState<'db>,
    ) {
        match &self.data {
            ExprIRData::Literal(_) | ExprIRData::Error => {}
            ExprIRData::Field(field) => field.tokens(tokens, state),
            ExprIRData::Ident(ident) => ident.tokens(tokens, state),
            ExprIRData::CodeBlock(block) => block.tokens(tokens, state),
            ExprIRData::Call(call) => call.tokens(tokens, state),
            ExprIRData::MemberCall(member_call) => member_call.tokens(tokens, state),
            ExprIRData::Match(match_) => match_.tokens(tokens, state),
            ExprIRData::Tuple(tuple) => {
                for (e, _) in tuple {
                    e.tokens(tokens, state);
                }
            }
            ExprIRData::Op(op) => op.tokens(tokens, state),
            ExprIRData::Lambda(lambda) => lambda.tokens(tokens, state),
            ExprIRData::While(while_) => while_.tokens(tokens, state),
            ExprIRData::IfElse(if_else) => if_else.tokens(tokens, state),
            ExprIRData::ImplicitDyn(expr, _) => expr.tokens(tokens, state),
        }
    }

    fn hover(&self, _: usize, state: &mut IrState<'db>) -> Option<String> {
        Some(self.ty.get_ir_name(state))
    }
}

impl Literal {
    pub fn replace_chars(&self) -> Self {
        match self {
            Literal::String(text) => Literal::String(text.replace("\\n", "\n")),
            _ => self.clone(),
        }
    }
}

impl<'db> ExprIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<ByteCode> {
        match &self.data {
            ExprIRData::Literal(lit) => vec![ByteCode::Push(lit.replace_chars())],
            ExprIRData::Field(field) => field.build(state),
            ExprIRData::Ident(ident) => match &ident.last().unwrap().0 {
                IdentDef::Variable(var) => match &var.kind {
                    TokenKind::Var => {
                        let var = state.get_var(&var.name).unwrap();
                        vec![ByteCode::GetLocal(var)]
                    }
                    TokenKind::Param => {
                        let param = state.get_param(&var.name).unwrap();
                        vec![ByteCode::Param(param)]
                    }
                    _ => todo!(),
                },
                IdentDef::Generic(_) => todo!(),
                IdentDef::Decl(decl) => {
                    if matches!(
                        decl.kind(state.db),
                        DeclKind::Struct {
                            body: StructDecl::None,
                            ..
                        } | DeclKind::Member {
                            body: StructDecl::None,
                        }
                    ) {
                        vec![ByteCode::Construct {
                            id: decl.as_id().as_u32(),
                            len: 0,
                        }]
                    } else {
                        panic!("Can only construct unit decl as ident")
                    }
                }
                IdentDef::Unresolved => unreachable!(),
            },
            ExprIRData::CodeBlock(CodeBlockIR { stmts, .. }) => {
                state.enter_scope();
                let code = stmts
                    .iter()
                    .flat_map(|(stmt, _)| stmt.build(state))
                    .collect();
                state.exit_scope();
                code
            }
            ExprIRData::Call(call) => call.build(state),
            ExprIRData::MemberCall(member_call) => member_call.build(state),
            ExprIRData::Match(match_) => match_.build(state),
            ExprIRData::Tuple(tuple) => {
                let mut code = tuple
                    .iter()
                    .flat_map(|e| e.0.build(state))
                    .collect::<Vec<_>>();
                code.push(ByteCode::Construct {
                    id: 0,
                    len: code.len() as u32,
                });
                code
            }
            ExprIRData::Op(op) => op.build(state),
            ExprIRData::Lambda(lambda) => todo!(),
            ExprIRData::While(while_) => while_.build(state),
            ExprIRData::IfElse(if_else) => if_else.build(state),
            ExprIRData::ImplicitDyn(expr, trait_decl) => {
                let mut code = expr.build(state);
                code.push(ByteCode::Dyn(state.get_vtable(&expr.ty)));
                code
            }
            ExprIRData::Error => unreachable!(),
        }
    }
}

impl<'db> ExprIRData<'db> {
    pub fn is_dyn(&self, state: &mut CheckState<'db>) -> bool {
        match self {
            ExprIRData::Ident(ident) => match &ident.last().unwrap().0 {
                IdentDef::Decl(decl) => matches!(decl.kind(state.db), DeclKind::Trait { .. }),
                IdentDef::Variable(var) => {
                    if let Ty::Named(Named { name, .. }) = var.ty {
                        let decl = state.project.get_decl(state.db, name).unwrap();
                        if let DeclKind::Trait { .. } = decl.kind(state.db) {
                            return true;
                        }
                    }
                    false
                }
                _ => false,
            },
            ExprIRData::Field(field) => {
                let decl = field.decl.unwrap();
                let DeclKind::Struct { body, .. } = decl.kind(state.db) else {
                    panic!("Expected struct")
                };
                if let StructDecl::Fields(fields) = body {
                    let field = fields
                        .iter()
                        .find(|f| f.0 == field.name.0)
                        .unwrap();
                    if let Ty::Named(Named { name, .. }) = field.1 {
                        let decl = state.project.get_decl(state.db, name).unwrap();
                        if let DeclKind::Trait { .. } = decl.kind(state.db) {
                            return true;
                        }
                    }
                    false
                } else {
                    todo!()
                }
            },
            ExprIRData::CodeBlock(_)
            | ExprIRData::Call(_)
            | ExprIRData::IfElse(_)
            | ExprIRData::Match(_)
            | ExprIRData::ImplicitDyn(_, _)
            | ExprIRData::While(_) => true,
            ExprIRData::MemberCall(_) => todo!(),
            ExprIRData::Tuple(_)
            | ExprIRData::Literal(_)
            // TODO: Will change with op overloading
            | ExprIRData::Op(_)
            | ExprIRData::Lambda(_) => false,
            ExprIRData::Error => unreachable!(),
        }
    }
}
