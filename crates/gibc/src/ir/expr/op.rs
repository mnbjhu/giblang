use gvm::format::instr::ByteCode;

use crate::{
    check::{build_state::BuildState, scoped_state::Scoped, state::CheckState},
    ir::{builder::ByteCodeNode, ContainsOffset, IrNode},
    parser::expr::op::{Op, OpKind},
    ty::Ty,
    util::{Span, Spanned},
};

use super::{ExprIR, ExprIRData};

#[derive(Debug, PartialEq, Clone)]
pub struct OpIR<'db> {
    pub left: Box<Spanned<ExprIR<'db>>>,
    pub right: Box<Spanned<ExprIR<'db>>>,
    pub kind: OpKind,
}

impl<'db> Op {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let left = Box::new((self.left.as_ref().0.check(state), self.left.1));
        let right = Box::new((self.right.as_ref().0.check(state), self.right.1));
        // TODO: Implement operator checking
        ExprIR {
            data: ExprIRData::Op(OpIR {
                left,
                right,
                kind: self.kind.clone(),
            }),
            ty: Ty::Unknown,
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

impl<'db> IrNode<'db> for OpIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.left.1.contains_offset(offset) {
            return self.left.0.at_offset(offset, state);
        }
        if self.right.1.contains_offset(offset) {
            return self.right.0.at_offset(offset, state);
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

    fn debug_name(&self) -> &'static str {
        "OpIR"
    }
}

impl<'db> OpIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        let mut code = vec![self.left.0.build(state)];
        code.push(self.right.0.build(state));
        let op = match &self.kind {
            OpKind::Add => ByteCode::Add,
            OpKind::Sub => ByteCode::Sub,
            OpKind::Mul => ByteCode::Mul,
            OpKind::Div => ByteCode::Div,
            OpKind::Mod => ByteCode::Mod,
            OpKind::Eq => ByteCode::Eq,
            OpKind::Neq => ByteCode::Neq,
            OpKind::Lt => ByteCode::Lt,
            OpKind::Gt => ByteCode::Gt,
            OpKind::Lte => ByteCode::Lte,
            OpKind::Gte => ByteCode::Gte,
            OpKind::And => ByteCode::And,
            OpKind::Or => ByteCode::Or,
        };
        code.push(ByteCodeNode::Code(vec![op]));
        ByteCodeNode::Block(code)
    }
}
