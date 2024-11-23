use gvm::format::instr::ByteCode;

use crate::{
    check::{build_state::BuildState, state::CheckState},
    ir::{builder::ByteCodeNode, common::condition::ConditionIR, ContainsOffset as _, IrNode},
    parser::expr::if_else::{IfBranch, IfElse},
    ty::Ty,
    util::Spanned,
};

use super::{
    block::{check_block, expect_block, CodeBlockIR},
    ExprIR, ExprIRData,
};

#[derive(Clone, PartialEq, Debug)]
pub struct IfElseIR<'db> {
    pub ifs: Vec<Spanned<IfBranchIR<'db>>>,
    pub else_: Option<Spanned<CodeBlockIR<'db>>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct IfBranchIR<'db> {
    pub condition: Spanned<ConditionIR<'db>>,
    pub body: Spanned<CodeBlockIR<'db>>,
}

impl<'db> IfElse {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let mut ty = Ty::Unknown;
        let ifs = self
            .ifs
            .iter()
            .map(|(branch, span)| {
                if let Ty::Unknown = ty {
                    let ir = branch.check(state);
                    ty = ir
                        .body
                        .0
                        .stmts
                        .last()
                        .map_or(Ty::unit(), |(stmt, _)| stmt.get_ty());
                    (ir, *span)
                } else {
                    (branch.expect(state, &ty), *span)
                }
            })
            .collect();
        let else_ = self.else_.as_ref().map(|(block, span)| {
            let block = if let Ty::Unknown = ty {
                check_block(block, state)
            } else {
                expect_block(block, state, &ty, *span)
            };
            let ExprIR {
                data: ExprIRData::CodeBlock(block),
                ..
            } = block
            else {
                panic!("Expected code block...");
            };
            (block, *span)
        });

        ExprIR {
            data: ExprIRData::IfElse(IfElseIR { ifs, else_ }),
            ty,
        }
    }

    pub fn expect(&self, state: &mut CheckState<'db>, expected: &Ty<'db>) -> ExprIR<'db> {
        let ifs = self
            .ifs
            .iter()
            .map(|(branch, span)| (branch.expect(state, expected), *span))
            .collect();
        let else_ = self.else_.as_ref().map(|(block, span)| {
            let ExprIR {
                data: ExprIRData::CodeBlock(block),
                ..
            } = expect_block(block, state, expected, *span)
            else {
                panic!("Expected code block...");
            };
            (block, *span)
        });

        ExprIR {
            data: ExprIRData::IfElse(IfElseIR { ifs, else_ }),
            ty: expected.clone(),
        }
    }
}

impl<'db> IfBranch {
    pub fn check(&self, state: &mut CheckState<'db>) -> IfBranchIR<'db> {
        let condition = self.condition.0.check(state, self.condition.1);
        let condition = (condition, self.condition.1);
        let ExprIR {
            data: ExprIRData::CodeBlock(body),
            ..
        } = check_block(&self.body.0, state)
        else {
            panic!("Expected code block...")
        };
        let body = (body, self.body.1);
        IfBranchIR { condition, body }
    }

    pub fn expect(&self, state: &mut CheckState<'db>, expected: &Ty<'db>) -> IfBranchIR<'db> {
        let condition = self.condition.0.check(state, self.condition.1);
        let condition = (condition, self.condition.1);
        let ExprIR {
            data: ExprIRData::CodeBlock(body),
            ..
        } = expect_block(&self.body.0, state, expected, self.body.1)
        else {
            panic!("Expected code block...")
        };
        let body = (body, self.body.1);
        IfBranchIR { condition, body }
    }
}

impl<'db> IrNode<'db> for IfElseIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        for (branch, span) in &self.ifs {
            if span.contains_offset(offset) {
                return branch.at_offset(offset, state);
            }
        }
        if let Some((block, span)) = &self.else_ {
            if span.contains_offset(offset) {
                return block.at_offset(offset, state);
            }
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        for (branch, _) in &self.ifs {
            branch.tokens(tokens, state);
        }
        if let Some((block, _)) = &self.else_ {
            block.tokens(tokens, state);
        }
    }
}
impl<'db> IrNode<'db> for IfBranchIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.condition.1.contains_offset(offset) {
            return self.condition.0.at_offset(offset, state);
        }
        if self.body.1.contains_offset(offset) {
            return self.body.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.condition.0.tokens(tokens, state);
        self.body.0.tokens(tokens, state);
    }
}

impl<'db> IfElseIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        ByteCodeNode::If {
            branches: self.ifs.iter().map(|(if_, _)| if_.build(state)).collect(),
            else_: self.else_.as_ref().map(|(e, _)| Box::new(e.build(state))),
        }
    }
}

impl<'db> IfBranchIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> (Box<ByteCodeNode>, Box<ByteCodeNode>) {
        match &self.condition.0 {
            ConditionIR::Let(let_) => {
                let expr = let_.expr.0.build(state);
                let then = let_.pattern.0.build(state);
                let cond = let_.pattern.0.build_match(state);
                let cond = if then.len() == 0 {
                    ByteCodeNode::Block(vec![expr, cond])
                } else {
                    ByteCodeNode::Block(vec![expr, ByteCodeNode::Code(vec![ByteCode::Copy]), cond])
                };
                let then = ByteCodeNode::Block(vec![then, self.body.0.build(state)]);
                (Box::new(cond), Box::new(then))
            }
            ConditionIR::Expr(e) => {
                let cond = ByteCodeNode::Block(vec![e.build(state), ByteCodeNode::Next]);
                let then = ByteCodeNode::Block(vec![self.body.0.build(state)]);
                (Box::new(cond), Box::new(then))
            }
        }
    }
}
