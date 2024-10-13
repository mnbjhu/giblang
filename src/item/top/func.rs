use std::collections::HashMap;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::func::Func,
    ty::Ty,
};

impl AstItem for Func {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(state, offset);
        }
        self.generics.0.check(state);
        for arg in &self.args {
            if arg.1.contains_offset(offset) {
                return arg.0.at_offset(state, offset);
            }
            arg.0.check(state);
        }
        if let Some(rec) = &self.receiver {
            if rec.1.contains_offset(offset) {
                return rec.0.at_offset(state, offset);
            }
        }
        if let Some(ret) = &self.ret {
            if ret.1.contains_offset(offset) {
                return ret.0.at_offset(state, offset);
            }
        }
        if let Some(body) = &self.body {
            for stmt in body {
                if stmt.1.contains_offset(offset) {
                    return stmt.0.at_offset(state, offset);
                }
                stmt.0.check(state);
            }
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Func,
        });
        self.generics.0.tokens(state, tokens);
        self.generics.0.check(state);
        for arg in &self.args {
            arg.0.tokens(state, tokens);
            arg.0.check(state);
        }
        if let Some(rec) = &self.receiver {
            rec.0.tokens(state, tokens);
        }
        if let Some(ret) = &self.ret {
            ret.0.tokens(state, tokens);
        }
        if let Some(body) = &self.body {
            for stmt in body {
                stmt.0.tokens(state, tokens);
                stmt.0.check(state);
            }
        }
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState,
        _: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> Option<String> {
        Some(format!("Function {}", self.name.0))
    }
}
