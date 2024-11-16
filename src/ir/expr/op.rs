use crate::{
    check::{build_state::BuildState, state::CheckState},
    ir::{ContainsOffset, IrNode},
    parser::expr::op::{Op, OpKind},
    run::bytecode::ByteCode,
    ty::Ty,
    util::{Span, Spanned},
};

use super::{ExprIR, ExprIRData};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct OpIR<'db> {
    pub left: Box<Spanned<ExprIR<'db>>>,
    pub right: Box<Spanned<ExprIR<'db>>>,
    pub kind: OpKind,
}

impl<'db> Op {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let left = Box::new((self.left.as_ref().0.check(state), self.right.1));
        let right = Box::new((self.right.as_ref().0.check(state), self.right.1));
        // TODO: Implement operator checking
        ExprIR {
            data: ExprIRData::Op(OpIR {
                left,
                right,
                kind: self.kind.clone(),
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

impl<'db> IrNode<'db> for OpIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.left.1.contains_offset(offset) {
            self.left.0.at_offset(offset, state);
        }
        if self.right.1.contains_offset(offset) {
            self.left.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.left.0.tokens(tokens, state);
        self.right.0.tokens(tokens, state);
    }
}

impl<'db> OpIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<ByteCode> {
        match &self.kind {
            OpKind::Add => vec![ByteCode::Add],
            OpKind::Sub => vec![ByteCode::Sub],
            OpKind::Mul => vec![ByteCode::Mul],
            OpKind::Div => todo!(),
            OpKind::Eq => vec![ByteCode::Eq],
            OpKind::Neq => vec![ByteCode::Eq, ByteCode::Not],
        }
    }
}
