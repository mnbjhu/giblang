use gvm::format::instr::ByteCode;
use salsa::plumbing::AsId;

use crate::{
    check::{build_state::BuildState, scoped_state::Scoped as _, SemanticToken},
    db::decl::DeclKind,
    ir::{builder::ByteCodeNode, ContainsOffset, IrNode},
    item::definitions::ident::IdentDef,
    util::Spanned,
};

use super::{ExprIR, ExprIRData};

use crate::{
    check::{
        err::{missing_receiver::MissingReceiver, unexpected_args::UnexpectedArgs, CheckError},
        state::CheckState,
    },
    parser::expr::call::Call,
    ty::{FuncTy, Ty},
    util::Span,
};

#[derive(Clone, PartialEq, Debug)]
pub struct CallIR<'db> {
    pub expr: Box<Spanned<ExprIR<'db>>>,
    pub args: Vec<Spanned<ExprIR<'db>>>,
    pub ty: Option<FuncTy<'db>>,
}
impl<'db> Call {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let name_ir = self.name.0.check(state);
        if let Ty::Unknown = name_ir.ty {
            return ExprIR {
                data: ExprIRData::Call(CallIR {
                    expr: Box::new((name_ir, self.name.1)),
                    args: self
                        .args
                        .iter()
                        .map(|arg| (arg.0.check(state), arg.1))
                        .collect(),
                    ty: None,
                }),
                ty: Ty::Unknown,
            };
        }
        let func_ty = name_ir.ty.try_get_func_ty(state, self.name.1);
        if let Some(func_ty) = &func_ty {
            let FuncTy {
                args: expected_args,
                ret,
                receiver,
            } = &func_ty;
            if let Some(receiver) = receiver {
                if let Some(self_ty) = state.get_variable("self") {
                    self_ty
                        .ty
                        .clone()
                        .expect_is_instance_of(receiver, state, self.name.1);
                } else {
                    state.error(CheckError::MissingReceiver(MissingReceiver {
                        span: self.name.1,
                        file: state.file_data,
                        expected: receiver.get_name(state),
                    }));
                }
            }
            if expected_args.len() != self.args.len() {
                state.error(CheckError::UnexpectedArgs(UnexpectedArgs {
                    expected: expected_args.len(),
                    found: self.args.len(),
                    span: self.name.1,
                    file: state.file_data,
                    func: func_ty.get_name(state),
                }));
            }
            let mut args = vec![];
            for ((arg, span), expected) in self.args.iter().zip(expected_args) {
                args.push((arg.expect(state, expected, *span), *span));
            }
            let ty = ret.as_ref().clone();
            return ExprIR {
                data: ExprIRData::Call(CallIR {
                    expr: Box::new((name_ir, self.name.1)),
                    args,
                    ty: Some(func_ty.clone()),
                }),
                ty,
            };
        } else if !matches!(name_ir.ty, Ty::Unknown) {
            state.simple_error(
                &format!(
                    "Expected a function but found '{}'",
                    name_ir.ty.get_name(state)
                ),
                self.name.1,
            );
        }
        ExprIR {
            data: ExprIRData::Call(CallIR {
                expr: Box::new((name_ir, self.name.1)),
                args: self
                    .args
                    .iter()
                    .map(|arg| (arg.0.check(state), arg.1))
                    .collect(),
                ty: None,
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

impl<'db> IrNode<'db> for CallIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.expr.1.contains_offset(offset) {
            return self.expr.0.at_offset(offset, state);
        }
        for arg in &self.args {
            if arg.1.contains_offset(offset) {
                return arg.0.at_offset(offset, state);
            }
        }
        panic!("Offset not in node");
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut crate::ir::IrState<'db>) {
        self.expr.0.tokens(tokens, state);
        for arg in &self.args {
            arg.0.tokens(tokens, state);
        }
    }

    fn debug_name(&self) -> &'static str {
        "CallIR"
    }
}

impl<'db> CallIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        let mut code = vec![];
        for arg in &self.args {
            code.push(arg.0.build(state));
        }
        state.inc_index(1);
        match &self.expr.0.data {
            ExprIRData::Ident(ident) => match ident.last().unwrap().0 {
                IdentDef::Variable(_) => todo!(),
                IdentDef::Generic(_) => todo!(),
                IdentDef::Decl(decl) => match decl.kind(state.db) {
                    DeclKind::Function(_) => code.push(ByteCodeNode::Code(vec![ByteCode::Call(
                        decl.as_id().as_u32(),
                    )])),
                    DeclKind::Struct { body, .. } | DeclKind::Member { body, .. } => {
                        code.push(ByteCodeNode::Code(vec![ByteCode::Construct {
                            id: decl.as_id().as_u32(),
                            len: body.arg_count(),
                        }]));
                    }
                    _ => todo!(),
                },
                IdentDef::Unresolved => todo!(),
            },
            _ => todo!(),
        };
        ByteCodeNode::Block(code)
    }
}
