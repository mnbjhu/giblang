use gvm::format::instr::ByteCode;

use crate::{
    check::{build_state::BuildState, state::CheckState},
    ir::{common::condition::ConditionIR, ContainsOffset, IrNode},
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
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<ByteCode> {
        match &self.condition.0 {
            ConditionIR::Let(let_) => {
                let mut code = let_.expr.0.build(state);
                code.push(ByteCode::Copy);
                code.extend(let_.pattern.0.build_match(state, &mut 1));
                let mut body = let_.pattern.0.build(state);
                body.extend(self.block.0.build(state));
                let break_ = body.len();
                let continue_ = body.len() + code.len();
                code.push(ByteCode::Jne(break_ as i32 + 2));
                code.extend(body);
                code.push(ByteCode::Jmp(-(continue_ as i32 + 1)));
                code
            }
            ConditionIR::Expr(expr) => {
                let mut expr = expr.build(state);
                let body = self.block.0.build(state);
                let break_ = body.len();
                let continue_ = body.len() + expr.len();
                expr.push(ByteCode::Jne(break_ as i32 + 2));
                expr.extend(body);
                expr.push(ByteCode::Jmp(-(continue_ as i32 + 1)));
                expr
            }
        }
    }
}
