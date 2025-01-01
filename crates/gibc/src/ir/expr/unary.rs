use gvm::format::instr::ByteCode;

use crate::{
    check::{build_state::BuildState, scoped_state::Scoped as _, state::CheckState},
    ir::{builder::ByteCodeNode, ContainsOffset, IrNode},
    parser::expr::unary::{UnaryOp, UnaryOpKind},
    ty::Ty,
    util::{Span, Spanned},
};

use super::{ExprIR, ExprIRData};

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOpIR<'db> {
    pub expr: Box<Spanned<ExprIR<'db>>>,
    pub kind: UnaryOpKind,
}

impl<'db> UnaryOp {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        match self.kind {
            UnaryOpKind::Not => {
                let bool_ty = Ty::bool(state.db);
                let expr = self.expr.as_ref().0.expect(state, &bool_ty, self.expr.1);
                ExprIR {
                    data: ExprIRData::Unary(UnaryOpIR {
                        kind: self.kind.clone(),
                        expr: Box::new((expr, self.expr.1)),
                    }),
                    ty: bool_ty,
                    order: state.inc_order(),
                }
            }
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

impl<'db> IrNode<'db> for UnaryOpIR<'db> {
    fn debug_name(&self) -> &'static str {
        "UnaryOpIR"
    }

    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.expr.1.contains_offset(offset) {
            self.expr.0.at_offset(offset, state)
        } else {
            self
        }
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.expr.0.tokens(tokens, state)
    }
}

impl<'db> UnaryOpIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        match self.kind {
            UnaryOpKind::Not => ByteCodeNode::Block(vec![
                self.expr.0.build(state),
                ByteCodeNode::Code(vec![ByteCode::Not]),
            ]),
        }
    }
}
