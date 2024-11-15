use chumsky::container::Container;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    ir::{
        common::generic_args::GenericArgsIR,
        expr::{
            block::{check_block, expect_block, CodeBlockIR},
            ExprIR, ExprIRData,
        },
        ty::TypeIR,
        IrNode, IrState,
    },
    item::common::type_::ContainsOffset,
    parser::top::func::Func,
    ty::Ty,
    util::Spanned,
};

use super::arg::FunctionArgIR;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct FuncIR<'db> {
    pub receiver: Option<Spanned<TypeIR<'db>>>,
    pub name: Spanned<String>,
    pub args: Vec<Spanned<FunctionArgIR<'db>>>,
    pub generics: Spanned<GenericArgsIR<'db>>,
    pub ret: Option<Spanned<TypeIR<'db>>>,
    pub body: CodeBlockIR<'db>,
}

impl<'db> Func {
    pub fn check(&self, state: &mut CheckState<'db>, allow_empty: bool) -> FuncIR<'db> {
        state.path.push(self.name.0.to_string());
        let generics = (self.generics.0.check(state), self.generics.1);
        let receiver = self.receiver.as_ref().map(|(rec, span)| {
            let ir = rec.check(state);
            state.add_self_param(ir.ty.clone(), *span);
            (ir, *span)
        });
        let args = self
            .args
            .iter()
            .map(|(arg, span)| (arg.check(state), *span))
            .collect();
        let ret = self.ret.as_ref().map(|(rec, span)| {
            let ir = rec.check(state);
            (ir, *span)
        });
        let expected = ret.as_ref().map_or(Ty::unit(), |ret| ret.0.ty.clone());
        let block = if !allow_empty || self.body.is_some() {
            expect_block(
                self.body.as_ref().unwrap_or(&vec![]),
                state,
                &expected,
                self.name.1,
            )
        } else {
            check_block(self.body.as_ref().unwrap_or(&vec![]), state)
        };
        let ExprIR {
            data: ExprIRData::CodeBlock(body),
            ..
        } = block
        else {
            panic!("Expected a block")
        };
        state.path.pop();
        FuncIR {
            receiver,
            name: self.name.clone(),
            args,
            generics,
            ret,
            body,
        }
    }
}

impl<'db> IrNode<'db> for FuncIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        if let Some(receiver) = &self.receiver {
            if receiver.1.contains_offset(offset) {
                return receiver.0.at_offset(offset, state);
            }
        }
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(offset, state);
        }
        for arg in &self.args {
            if arg.1.contains_offset(offset) {
                return arg.0.at_offset(offset, state);
            }
        }
        if let Some(ret) = &self.ret {
            if ret.1.contains_offset(offset) {
                return ret.0.at_offset(offset, state);
            }
        }
        for (stmt, span) in &self.body.stmts {
            if span.contains_offset(offset) {
                return stmt.at_offset(offset, state);
            }
        }
        self
    }

    fn tokens(&self, tokens: &mut Vec<crate::check::SemanticToken>, state: &mut IrState<'db>) {
        if let Some(receiver) = &self.receiver {
            receiver.0.tokens(tokens, state);
        }
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Func,
        });
        self.generics.0.tokens(tokens, state);
        if let Some(ret) = &self.ret {
            ret.0.tokens(tokens, state);
        }
        self.body.tokens(tokens, state);
    }
}
