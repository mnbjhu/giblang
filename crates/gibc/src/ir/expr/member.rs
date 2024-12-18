use gvm::format::instr::ByteCode;
use salsa::plumbing::AsId;

use crate::{
    check::{
        build_state::BuildState, scoped_state::Scoped, state::CheckState, SemanticToken, TokenKind,
    },
    db::decl::{func::Function, DeclKind},
    ir::{builder::ByteCodeNode, ContainsOffset, IrNode, IrState},
    item::definitions::ident::IdentDef,
    parser::expr::member::MemberCall,
    ty::{FuncTy, Ty},
    util::{Span, Spanned},
};

use super::{ExprIR, ExprIRData};

#[derive(Debug, PartialEq, Clone)]
pub struct MemberCallIR<'db> {
    pub receiver: Box<Spanned<ExprIR<'db>>>,
    pub name: Spanned<String>,
    pub args: Vec<Spanned<ExprIR<'db>>>,
    pub def: IdentDef<'db>,
    pub ty: Option<FuncTy<'db>>,
}

impl<'db> MemberCall {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let rec = Box::new((self.rec.0.check(state), self.rec.1));
        let funcs = rec.0.ty.get_member_func(&self.name, state);
        let Some((def, func_ty)) = funcs else {
            state.simple_error(
                &format!(
                    "No function {} found for type {}",
                    self.name.0,
                    rec.0.ty.get_name(state)
                ),
                self.name.1,
            );
            return ExprIR {
                data: ExprIRData::MemberCall(MemberCallIR {
                    receiver: rec,
                    name: self.name.clone(),
                    args: self
                        .args
                        .iter()
                        .map(|(arg, span)| (arg.check(state), *span))
                        .collect(),
                    def: IdentDef::Unresolved,
                    ty: None,
                }),
                ty: Ty::Unknown,
                order: state.inc_order(),
            };
        };
        let FuncTy {
            args: expected_args,
            ret,
            receiver,
        } = func_ty.clone();
        if let Some(expected) = receiver {
            rec.0.ty.expect_is_instance_of(&expected, state, self.rec.1);
        }
        if expected_args.len() != self.args.len() {
            state.simple_error(
                &format!(
                    "Expected {} arguments but found {}",
                    expected_args.len(),
                    self.args.len()
                ),
                self.name.1,
            );
        }

        let args = self
            .args
            .iter()
            .zip(expected_args)
            .map(|((arg, span), expected)| (arg.expect(state, &expected, *span), *span))
            .collect();
        let ty = ret.as_ref().clone();
        ExprIR {
            data: ExprIRData::MemberCall(MemberCallIR {
                receiver: rec,
                name: self.name.clone(),
                args,
                def,
                ty: Some(func_ty),
            }),
            ty,
            order: state.inc_order(),
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

impl<'db> IrNode<'db> for MemberCallIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        if self.receiver.1.contains_offset(offset) {
            return self.receiver.0.at_offset(offset, state);
        }
        for arg in &self.args {
            if arg.1.contains_offset(offset) {
                return arg.0.at_offset(offset, state);
            }
        }
        self
    }

    fn tokens(&self, tokens: &mut Vec<crate::check::SemanticToken>, state: &mut IrState<'db>) {
        self.receiver.0.tokens(tokens, state);
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Func,
        });
        for arg in &self.args {
            arg.0.tokens(tokens, state);
        }
    }

    fn hover(&self, _: usize, state: &mut IrState<'db>) -> Option<String> {
        Some(format!(
            "{}: {}",
            self.def.hover(state),
            self.ty
                .as_ref()
                .map_or("Unknown".to_string(), |func| func.get_ir_name(state))
        ))
    }

    fn goto(
        &self,
        _: usize,
        state: &mut IrState<'db>,
    ) -> Option<(crate::db::input::SourceFile, Span)> {
        self.def.goto(state)
    }

    fn debug_name(&self) -> &'static str {
        "MemberCallIR"
    }
}

impl<'db> MemberCallIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        let mut code = vec![self.receiver.0.build(state)];
        for arg in &self.args {
            code.push(arg.0.build(state));
        }
        match &self.def {
            IdentDef::Variable(_) => todo!(),
            IdentDef::Generic(_) => todo!(),
            IdentDef::Decl(decl) => {
                let DeclKind::Function(Function { virtual_, .. }) = decl.kind(state.db) else {
                    panic!("Expected function")
                };
                if *virtual_ {
                    code.push(ByteCodeNode::Code(vec![ByteCode::DynCall(
                        decl.as_id().as_u32(),
                    )]));
                } else {
                    code.push(ByteCodeNode::Code(vec![ByteCode::Call(
                        decl.as_id().as_u32(),
                    )]));
                }
                ByteCodeNode::Block(code)
            }
            IdentDef::Unresolved => todo!(),
        }
    }
}
