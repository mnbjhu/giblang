use std::collections::HashMap;

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::{
        common::{generics::brackets, type_::ContainsOffset},
        AstItem,
    },
    parser::expr::member::MemberCall,
    ty::Ty,
};

impl AstItem for MemberCall {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.rec
            .0
            .pretty(allocator)
            .append(".")
            .append(&self.name.0)
            .append(brackets(allocator, "(", ")", &self.args))
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        self.rec.0.tokens(state, tokens);
        if self
            .rec
            .0
            .check(state)
            .get_member_func(&self.name, state)
            .is_some()
        {
            tokens.push(SemanticToken {
                span: self.name.1,
                kind: TokenKind::Func,
            });
        }
        for args in &self.args {
            args.0.tokens(state, tokens);
        }
    }

    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.rec.1.contains_offset(offset) {
            return self.rec.0.at_offset(state, offset);
        }
        for arg in &self.args {
            if arg.1.contains_offset(offset) {
                return arg.0.at_offset(state, offset);
            }
        }
        self
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        _: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> Option<String> {
        let func_ty = self.rec.0.check(state).get_member_func(&self.name, state)?;
        Some(format!(
            "{}: {}",
            self.name.0,
            func_ty.get_name_with_types(state, type_vars)
        ))
    }

    fn completions(
        &self,
        state: &mut CheckState,
        _: usize,
        _: &HashMap<u32, Ty>,
    ) -> Vec<CompletionItem> {
        let rec = self.rec.0.check(state);
        let mut completions = Vec::new();
        for (name, func_ty) in rec.member_funcs(state) {
            completions.push(CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(func_ty.get_name(state)),
                ..Default::default()
            });
        }
        completions
    }
}
