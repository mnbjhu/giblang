use std::collections::HashMap;

use gvm::format::instr::ByteCode;

use crate::{
    check::{
        build_state::BuildState,
        state::{CheckState, VarDecl},
        SemanticToken,
    },
    ir::{builder::ByteCodeNode, stmt::StmtIR, ContainsOffset, IrNode, IrState},
    parser::expr::code_block::CodeBlock,
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

use super::{ExprIR, ExprIRData};

#[derive(Debug, PartialEq, Clone)]
pub struct CodeBlockIR<'db> {
    pub vars: HashMap<String, VarDecl<'db>>,
    pub generics: HashMap<String, Generic<'db>>,
    pub stmts: Vec<Spanned<StmtIR<'db>>>,
}

pub fn check_block<'db>(block: &CodeBlock, state: &mut CheckState<'db>) -> ExprIR<'db> {
    state.enter_scope();
    let stmts = block
        .iter()
        .map(|stmt| (stmt.0.check(state), stmt.1))
        .collect::<Vec<_>>();
    let (vars, generics) = state.exit_scope();
    let ty = stmts.last().map_or(Ty::unit(), |stmt| stmt.0.get_ty());
    ExprIR {
        data: ExprIRData::CodeBlock(CodeBlockIR {
            vars,
            generics,
            stmts,
        }),
        ty,
    }
}
pub fn expect_block<'db>(
    block: &CodeBlock,
    state: &mut CheckState<'db>,
    expected: &Ty<'db>,
    span: Span,
) -> ExprIR<'db> {
    if expected.is_unit() {
        return check_block(block, state);
    }
    state.enter_scope();
    if block.is_empty() {
        let (vars, generics) = state.exit_scope();
        Ty::unit().expect_is_instance_of(expected, state, span);
        return ExprIR {
            data: ExprIRData::CodeBlock(CodeBlockIR {
                vars,
                generics,
                stmts: vec![],
            }),
            ty: Ty::unit(),
        };
    }
    let mut stmts = vec![];
    for stmt in &block[0..block.len() - 1] {
        stmts.push((stmt.0.check(state), stmt.1));
    }
    let last = block.last().unwrap();
    let last = (last.0.expect(state, expected, last.1), last.1);
    let ty = last.0.get_ty();
    stmts.push(last);
    let (vars, generics) = state.exit_scope();
    ExprIR {
        data: ExprIRData::CodeBlock(CodeBlockIR {
            vars,
            generics,
            stmts,
        }),
        ty,
    }
}

impl<'db> IrNode<'db> for CodeBlockIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        state.enter_scope(self.vars.clone(), self.generics.clone());
        for stmt in &self.stmts {
            if stmt.1.contains_offset(offset) {
                return stmt.0.at_offset(offset, state);
            }
        }
        self
    }
    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        for stmt in &self.stmts {
            stmt.0.tokens(tokens, state);
        }
    }
}

impl<'db> CodeBlockIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        state.enter_scope();
        let ir = self
            .stmts
            .iter()
            .map(|(stmt, _)| stmt.build(state))
            .collect();
        state.exit_scope();
        ByteCodeNode::Block(ir)
    }
}
