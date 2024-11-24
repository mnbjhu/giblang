use block::{check_block, CodeBlockIR};
use call::CallIR;
use field::FieldIR;
use for_::ForIR;
use gvm::format::{instr::ByteCode, literal::Literal};
use ident::{check_ident, expect_ident};
use if_else::IfElseIR;
use lambda::LambdaIR;
use lit::Typed as _;
use match_::MatchIR;
use member::MemberCallIR;
use op::OpIR;
use salsa::plumbing::AsId;
use tuple::{check_tuple, expect_tuple};
use while_::WhileIR;

use crate::{
    check::{build_state::BuildState, scoped_state::Scoped, state::CheckState, TokenKind},
    db::{
        decl::{struct_::StructDecl, Decl, DeclKind, Project},
        input::Db,
    },
    item::definitions::ident::IdentDef,
    parser::expr::Expr,
    ty::{Generic, Named, Ty},
    util::{Span, Spanned},
};

use super::{
    builder::ByteCodeNode, common::pattern::SpannedQualifiedNameIR, AstKind, ContainsOffset,
    IrNode, IrState,
};

pub mod block;
pub mod call;
pub mod field;
pub mod for_;
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

#[derive(Debug, PartialEq, Clone)]
pub struct ExprIR<'db> {
    pub data: ExprIRData<'db>,
    pub ty: Ty<'db>,
    pub order: usize,
}

#[derive(Debug, PartialEq, Clone)]
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
    Phantom(Box<ExprIR<'db>>),
    For(ForIR<'db>),
    Error,
}

impl<'db> Expr {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        match self {
            Expr::Literal(lit) => ExprIR {
                data: ExprIRData::Literal(lit.clone()),
                ty: lit.to_ty(state.db),
                order: state.inc_order(),
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
                order: state.inc_order(),
            },
            Expr::IfElse(if_else) => if_else.check(state),
            Expr::For(for_) => for_.check(state),
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
                    order: state.inc_order(),
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
                order: state.inc_order(),
            },
            Expr::While(while_) => {
                let ir = while_.check(state);
                Ty::unit().expect_is_instance_of(expected, state, span);
                ir
            }
            Expr::For(for_) => {
                let ir = for_.check(state);
                Ty::unit().expect_is_instance_of(expected, state, span);
                ir
            }
            Expr::IfElse(if_else) => if_else.expect(state, expected),
        };
        if let Ty::Named(Named { name, .. }) = &expected {
            let decl = state.project.get_decl(state.db, *name).unwrap();
            if matches!(decl.kind(state.db), DeclKind::Trait { .. })
                && !res.data.is_dyn(state.db, state.project)
            {
                return ExprIR {
                    data: ExprIRData::ImplicitDyn(Box::new(res), decl),
                    ty: Ty::Unknown,
                    order: state.inc_order(),
                };
            }
        }
        res
    }
}

impl<'db> IrNode<'db> for ExprIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut super::IrState<'db>) -> &dyn IrNode {
        state.kind = AstKind::Expr;
        state.set_order(self.order);
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
            ExprIRData::For(for_) => for_.at_offset(offset, state),
            ExprIRData::IfElse(if_else) => if_else.at_offset(offset, state),
            ExprIRData::ImplicitDyn(expr, _) => expr.at_offset(offset, state),
            ExprIRData::Phantom(_) => unreachable!(),
        }
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut super::IrState<'db>,
    ) {
        match &self.data {
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
            ExprIRData::For(for_) => for_.tokens(tokens, state),
            ExprIRData::IfElse(if_else) => if_else.tokens(tokens, state),
            ExprIRData::ImplicitDyn(expr, _) => expr.tokens(tokens, state),
            ExprIRData::Phantom(_) | ExprIRData::Literal(_) | ExprIRData::Error => {}
        }
    }

    fn hover(&self, _: usize, state: &mut IrState<'db>) -> Option<String> {
        Some(self.ty.get_ir_name(state))
    }

    fn debug_name(&self) -> &'static str {
        "ExprIR"
    }
}

pub fn replace_chars(lit: Literal) -> Literal {
    match lit {
        Literal::String(text) => Literal::String(text.replace("\\n", "\n")),
        _ => lit,
    }
}

impl<'db> ExprIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        match &self.data {
            ExprIRData::Literal(lit) => {
                state.inc_index(1);
                ByteCodeNode::Code(vec![ByteCode::Push(replace_chars(lit.clone()))])
            }
            ExprIRData::Field(field) => field.build(state),
            ExprIRData::Ident(ident) => match &ident.last().unwrap().0 {
                IdentDef::Variable(var) => match &var.kind {
                    TokenKind::Var => {
                        let var = state.get_var(&var.name).unwrap();
                        ByteCodeNode::Code(vec![ByteCode::GetLocal(var)])
                    }
                    TokenKind::Param => {
                        let param = state.get_param(&var.name).unwrap();
                        ByteCodeNode::Code(vec![ByteCode::Param(param)])
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
                        ByteCodeNode::Code(vec![ByteCode::Construct {
                            id: decl.as_id().as_u32(),
                            len: 0,
                        }])
                    } else {
                        panic!("Can only construct unit decl as ident")
                    }
                }
                IdentDef::Unresolved => unreachable!(),
            },
            ExprIRData::CodeBlock(block) => block.build(state),
            ExprIRData::Call(call) => call.build(state),
            ExprIRData::MemberCall(member_call) => member_call.build(state),
            ExprIRData::Match(match_) => match_.build(state),
            ExprIRData::Tuple(tuple) => {
                let mut code = tuple.iter().map(|e| e.0.build(state)).collect::<Vec<_>>();
                code.push(ByteCodeNode::Code(vec![ByteCode::Construct {
                    id: 0,
                    len: code.len() as u32,
                }]));
                ByteCodeNode::Block(code)
            }
            ExprIRData::Op(op) => op.build(state),
            ExprIRData::Lambda(_) => todo!(),
            ExprIRData::While(while_) => while_.build(state),
            ExprIRData::For(for_) => for_.build(state),
            ExprIRData::IfElse(if_else) => if_else.build(state),
            ExprIRData::ImplicitDyn(expr, _) => {
                let mut code = vec![expr.build(state)];
                code.push(ByteCodeNode::Code(vec![ByteCode::Dyn(
                    state.get_vtable(&expr.ty),
                )]));
                ByteCodeNode::Block(code)
            }
            ExprIRData::Error => unreachable!(),
            ExprIRData::Phantom(expr) => expr.build(state),
        }
    }
}

impl<'db> Ty<'db> {
    pub fn is_dyn(&self, db: &'db dyn Db, project: Project) -> bool {
        if let Ty::Named(Named { name, .. }) = self {
            let decl = project.get_decl(db, *name).unwrap();
            if let DeclKind::Trait { .. } = decl.kind(db) {
                return true;
            }
        } else if let Ty::Generic(Generic { super_, .. }) = self {
            return super_.is_dyn(db, project);
        }
        false
    }
}

impl<'db> ExprIRData<'db> {
    pub fn is_dyn(&self, db: &'db dyn Db, project: Project) -> bool {
        match self {
            ExprIRData::Ident(ident) => match &ident.last().unwrap().0 {
                IdentDef::Decl(decl) => matches!(decl.kind(db), DeclKind::Trait { .. }),
                IdentDef::Variable(var) => {
                    var.ty.is_dyn(db, project)
                }
                _ => false,
            },
            ExprIRData::Field(field) => {
                if field.decl.is_none() {
                    return true;
                }
                let decl = field.decl.unwrap();
                let DeclKind::Struct { body, .. } = decl.kind(db) else {
                    panic!("Expected struct")
                };
                if let StructDecl::Fields(fields) = body {
                    let field = fields
                        .iter()
                        .find(|f| f.0 == field.name.0)
                        .unwrap();
                    if let Ty::Named(Named { name, .. }) = field.1 {
                        let decl = project.get_decl(db, name).unwrap();
                        if let DeclKind::Trait { .. } = decl.kind(db) {
                            return true;
                        }
                    }
                    false
                } else {
                    todo!()
                }
            },
            ExprIRData::Call(call) => {
                if call.ty.is_none() {
                    return true;
                }
                call.ty.as_ref().unwrap().ret.as_ref().is_dyn(db, project)
            }
            ExprIRData::MemberCall(member) => {
                if member.ty.is_none() {
                    return true;
                }
                member.ty.as_ref().unwrap().ret.as_ref().is_dyn(db, project)
            }
            ExprIRData::CodeBlock(_)
            | ExprIRData::IfElse(_)
            | ExprIRData::Match(_)
            | ExprIRData::ImplicitDyn(_, _)
            | ExprIRData::While(_)
            | ExprIRData::For(_)
            | ExprIRData::Error  => true,
            ExprIRData::Tuple(_)
            | ExprIRData::Literal(_)
            // TODO: Will change with op overloading
            | ExprIRData::Op(_)
            | ExprIRData::Lambda(_) => false,
            ExprIRData::Phantom(expr) => expr.data.is_dyn(db, project),
        }
    }
}
