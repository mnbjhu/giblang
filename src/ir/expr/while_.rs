use crate::{
    check::{build_state::BuildState, state::CheckState},
    ir::{ContainsOffset, IrNode},
    parser::expr::while_::While,
    run::bytecode::ByteCode,
    ty::Ty,
    util::Spanned,
};

use super::{
    block::{check_block, CodeBlockIR},
    ExprIR, ExprIRData,
};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct WhileIR<'db> {
    pub expr: Box<Spanned<ExprIR<'db>>>,
    pub block: Spanned<CodeBlockIR<'db>>,
}

impl<'db> While {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let expr = (
            self.expr.0.expect(state, &Ty::bool(state.db), self.expr.1),
            self.expr.1,
        );
        let ExprIR {
            data: ExprIRData::CodeBlock(block),
            ..
        } = check_block(&self.block.0, state)
        else {
            panic!("Expected block")
        };
        let block = (block, self.block.1);
        ExprIR {
            data: ExprIRData::While(WhileIR {
                expr: Box::new(expr),
                block,
            }),
            ty: Ty::unit(),
        }
    }
}

impl<'db> IrNode<'db> for WhileIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.expr.1.contains_offset(offset) {
            return self.expr.0.at_offset(offset, state);
        }
        if self.block.1.contains_offset(offset) {
            return self.block.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.expr.0.tokens(tokens, state);
        self.block.0.tokens(tokens, state);
    }
}

impl<'db> WhileIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<ByteCode> {
        let mut expr = self.expr.0.build(state);
        let body = self.block.0.build(state);
        let break_ = body.len();
        let continue_ = body.len() + expr.len();
        expr.push(ByteCode::Jne(break_ as i32 + 2));
        expr.extend(body);
        expr.push(ByteCode::Jmp(-(continue_ as i32 + 1)));
        expr
    }
}
