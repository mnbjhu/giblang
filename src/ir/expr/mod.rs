use block::{check_block, CodeBlockIR};
use call::CallIR;
use chumsky::container::Container;
use field::FieldIR;
use ident::{check_ident, expect_ident};
use lambda::LambdaIR;
use match_::MatchIR;
use member::MemberCallIR;
use op::OpIR;
use salsa::plumbing::AsId;
use tuple::{check_tuple, expect_tuple};
use while_::WhileIR;

use crate::{
    check::{build_state::BuildState, state::CheckState, TokenKind},
    db::decl::{struct_::StructDecl, Decl, DeclKind},
    item::definitions::ident::IdentDef,
    lexer::literal::Literal,
    parser::{expr::Expr, top::struct_::Struct},
    run::bytecode::ByteCode,
    ty::Ty,
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
            Expr::IfElse(_) => todo!(),
        }
    }

    pub fn expect(&self, state: &mut CheckState<'db>, ty: &Ty<'db>, span: Span) -> ExprIR<'db> {
        match self {
            Expr::Literal(lit) => ExprIR {
                data: ExprIRData::Literal(lit.clone()),
                ty: lit.to_ty(state.db),
            },
            Expr::Field(field) => field.expect(state, ty, span),
            Expr::Ident(ident) => expect_ident(ident, state, ty, span),
            Expr::CodeBlock(block) => check_block(block, state),
            Expr::Call(call) => call.expect(state, ty, span),
            Expr::MemberCall(member_call) => member_call.expect(state, ty, span),
            Expr::Match(match_) => match_.expect(state, ty, span),
            Expr::Tuple(tuple) => expect_tuple(tuple, state, ty, span),
            Expr::Op(op) => op.expect(state, ty, span),
            Expr::Lambda(lambda) => lambda.expect(state, ty, span),
            Expr::Error => ExprIR {
                data: ExprIRData::Error,
                ty: Ty::Unknown,
            },
            Expr::While(while_) => {
                let ir = while_.check(state);
                Ty::unit().expect_is_instance_of(ty, state, span);
                ir
            }
            Expr::IfElse(_) => todo!(),
        }
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
                IdentDef::Decl(decl) => match decl.kind(state.db) {
                    DeclKind::Struct { body, .. } | DeclKind::Member { body } => {
                        if let StructDecl::None = body {
                            vec![ByteCode::Construct {
                                id: decl.as_id().as_u32(),
                                len: 0,
                            }]
                        } else {
                            panic!("Expected fields")
                        }
                    }
                    DeclKind::Trait { generics, body } => todo!(),
                    DeclKind::Enum { generics, variants } => todo!(),
                    DeclKind::Function(_) => todo!(),
                    DeclKind::Module(_) => todo!(),
                },
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
            ExprIRData::Error => unreachable!(),
        }
    }
}
