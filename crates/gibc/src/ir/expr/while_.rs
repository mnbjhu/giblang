use chumsky::container::Container;
use gvm::format::instr::ByteCode;

use crate::{
    check::{build_state::BuildState, state::CheckState},
    ir::{builder::ByteCodeNode, common::condition::ConditionIR, ContainsOffset, IrNode},
    parser::expr::while_::While,
    ty::Ty,
    util::Spanned,
};

use super::{
    block::{check_block, CodeBlockIR},
    ExprIR, ExprIRData,
};

#[derive(Debug, PartialEq, Clone)]
pub struct WhileIR<'db> {
    pub condition: Box<Spanned<ConditionIR<'db>>>,
    pub block: Spanned<CodeBlockIR<'db>>,
}

impl<'db> While {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let condition = self.condition.0.check(state, self.condition.1);
        let condition = (condition, self.condition.1);
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
                condition: Box::new(condition),
                block,
            }),
            ty: Ty::unit(),
        }
    }
}

impl<'db> IrNode<'db> for WhileIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.condition.1.contains_offset(offset) {
            return self.condition.0.at_offset(offset, state);
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
        self.condition.0.tokens(tokens, state);
        self.block.0.tokens(tokens, state);
    }
}

impl<'db> WhileIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        match &self.condition.0 {
            ConditionIR::Let(let_) => {
                let cond = vec![let_.expr.0.build(state), let_.pattern.0.build_match(state)];
                let then = vec![
                    let_.pattern.0.build(state),
                    self.block.0.build(state),
                    ByteCodeNode::Continue,
                ];
                ByteCodeNode::While(
                    Box::new(ByteCodeNode::Block(cond)),
                    Box::new(ByteCodeNode::Block(then)),
                )
            }
            ConditionIR::Expr(expr) => {
                let cond = vec![expr.build(state), ByteCodeNode::MaybeBreak];
                let then = vec![self.block.0.build(state), ByteCodeNode::Continue];
                ByteCodeNode::While(
                    Box::new(ByteCodeNode::Block(cond)),
                    Box::new(ByteCodeNode::Block(then)),
                )
            }
        }
    }
}
